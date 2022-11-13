use std::str::FromStr;

use darling::ToTokens;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, FnArg, Generics, Ident, Item, ItemFn, PatType, Result, ReturnType, Type,
    TypeTuple, Visibility,
};

/// A function that can be wrapped in the Perseus test sub-harness.
pub struct TemplateFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// The arguments for custom properties and a global state, both of which
    /// are optional. (But global state needs custom properties, which can be a
    /// dummy `struct`.)
    pub args: Vec<FnArg>,
    /// The visibility of the function.
    pub vis: Visibility,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// The actual name of the function.
    pub name: Ident,
    /// The return type of the function.
    pub return_type: Box<Type>,
    /// Any generics the function takes (should be one for the Sycamore
    /// `GenericNode`).
    pub generics: Generics,
}
impl Parse for TemplateFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => {
                let ItemFn {
                    attrs,
                    vis,
                    sig,
                    block,
                } = func;
                // Validate each part of this function to make sure it fulfills the requirements
                // Mustn't be async
                if sig.asyncness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.asyncness,
                        "templates cannot be asynchronous",
                    ));
                }
                // Can't be const
                if sig.constness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.constness,
                        "const functions can't be used as templates",
                    ));
                }
                // Can't be external
                if sig.abi.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.abi,
                        "external functions can't be used as templates",
                    ));
                }
                // Must have an explicit return type
                let return_type = match sig.output {
                    ReturnType::Default => {
                        return Err(syn::Error::new_spanned(
                            sig,
                            "template functions can't return default/inferred type",
                        ))
                    }
                    ReturnType::Type(_, ty) => ty,
                };
                let mut args = Vec::new();
                for arg in sig.inputs.iter() {
                    // We don't care what the type is, as long as it's not `self`
                    if let FnArg::Receiver(arg) = arg {
                        return Err(syn::Error::new_spanned(arg, "templates can't take `self`"));
                    }
                    args.push(arg.clone())
                }
                // We can have anywhere between 1 and 3 arguments (scope, ?state, ?global state)
                if args.len() > 3 || args.is_empty() {
                    return Err(syn::Error::new_spanned(&sig.inputs, "template functions accept either one or two arguments (reactive scope; then one optional for custom properties)"));
                }

                Ok(Self {
                    block,
                    args,
                    vis,
                    attrs,
                    name: sig.ident,
                    return_type,
                    generics: sig.generics,
                })
            }
            item => Err(syn::Error::new_spanned(
                item,
                "only functions can be used as templates",
            )),
        }
    }
}

/// Converts the user-given name of a final reactive `struct` with lifetimes
/// into the same type, just without those lifetimes, so we can use it outside
/// the scope in which those lifetimes have been defined.
///
/// See the callers of this function to see exactly why it's necessary.
fn remove_lifetimes(ty: &Type) -> Type {
    // Don't run any transformation if this is the unit type
    match ty {
        Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty() => ty.clone(),
        _ => {
            let ty_str = ty.to_token_stream().to_string();
            // Remove any lifetimes from the type (anything in angular brackets beginning
            // with `'`) This regex just removes any lifetimes next to generics
            // or on their own, allowing for the whitespace Syn seems to insert
            let ty_str = Regex::new(r#"(('.*?) |<\s*('[^, ]*?)\s*>)"#)
                .unwrap()
                .replace_all(&ty_str, "");
            Type::Verbatim(TokenStream::from_str(&ty_str).unwrap())
        }
    }
}

pub fn template_impl(input: TemplateFn, is_reactive: bool) -> TokenStream {
    let TemplateFn {
        block,
        // We know that these are all typed (none are `self`)
        args: fn_args,
        generics,
        vis,
        attrs,
        name,
        return_type,
    } = input;

    let component_name = Ident::new(&(name.to_string() + "_component"), Span::call_site());

    if fn_args.len() == 2 && is_reactive {
        // Get the argument for the reactive scope
        let cx_arg = &fn_args[0];
        // There's an argument for page properties that needs to have state extracted,
        // so the wrapper will deserialize it We'll also make it reactive and
        // add it to the page state store
        let arg = &fn_args[1];
        let rx_props_ty = match arg {
            FnArg::Typed(PatType { ty, .. }) => remove_lifetimes(ty),
            FnArg::Receiver(_) => unreachable!(),
        };
        let name_string = name.to_string();
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(
                cx: ::sycamore::prelude::Scope,
                curr_view: &::sycamore::prelude::Signal<::sycamore::prelude::View<G>>,
                scope_disposers: &::sycamore::prelude::Signal<::std::vec::Vec<::sycamore::prelude::ScopeDisposer>>,
                props: ::perseus::template::PageProps
            ) {
                use ::perseus::state::{MakeRx, MakeRxRef, RxRef};

                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component]
                fn #component_name #generics(#cx_arg, #arg) -> #return_type {
                    #block
                }

                let props = {
                    // Check if properties of the reactive type are already in the page state store
                    // If they are, we'll use them (so state persists for templates across the whole app)
                    let render_ctx = ::perseus::RenderCtx::from_ctx(cx);
                    // The render context will automatically handle prioritizing frozen or active state for us for this page as long as we have a reactive state type, which we do!
                    // We need there to be no lifetimes in `rx_props_ty` here, since the lifetimes the user decalred are defined inside the above function, which we
                    // aren't inside!
                    match render_ctx.get_active_or_frozen_page_state::<<#rx_props_ty as RxRef>::RxNonRef>(&props.path) {
                            // If we navigated back to this page, and it's still in the PSS, the given state will be a dummy, but we don't need to worry because it's never checked if this evaluates
                        ::std::option::Option::Some(existing_state) => existing_state,
                        // Again, frozen state has been dealt with already, so we'll fall back to generated state
                        ::std::option::Option::None => {
                            // Again, the render context can do the heavy lifting for us (this returns what we need, and can do type checking)
                            // We also assume that any state we have is valid because it comes from the server
                            // The user really should have a generation function, but if they don't then they'd get a panic, so give them a nice error message
                            render_ctx.register_page_state_str::<<#rx_props_ty as RxRef>::RxNonRef>(&props.path, &props.state.unwrap_or_else(|| panic!("template `{}` takes a state, but no state generation functions were provided (please add at least one to use state)", #name_string))).unwrap()
                        }
                    }
                };

                ::sycamore::reactive::create_child_scope(cx, |child_cx| {
                    let view = #component_name(cx, props.to_ref_struct(cx));
                    curr_view.set(view);
                });
            }
        }
    } else if fn_args.len() == 2 && is_reactive == false {
        // This template takes state that isn't reactive (but it must implement
        // `UnreactiveState`) Get the argument for the reactive scope
        let cx_arg = &fn_args[0];
        // There's an argument for page properties that needs to have state extracted,
        // so the wrapper will deserialize it
        // We'll also make it reactive and add it to the page state store
        let arg = &fn_args[1];
        let props_ty = match arg {
            // This type isn't reactive, so we shouldn't need to remove lifetimes (this also acts as
            // a way of ensuring that users don't mix unreactive with reactive
            // accidentally, since this part should lead to compile-time errors)
            FnArg::Typed(PatType { ty, .. }) => ty,
            FnArg::Receiver(_) => unreachable!(),
        };
        let name_string = name.to_string();
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(
                cx: ::sycamore::prelude::Scope,
                curr_view: &::sycamore::prelude::Signal<::sycamore::prelude::View<G>>,
                scope_disposers: &::sycamore::prelude::Signal<::std::vec::Vec<::sycamore::prelude::ScopeDisposer>>,
                props: ::perseus::template::PageProps
            ) {
                use ::perseus::state::{MakeRx, MakeRxRef, RxRef, MakeUnrx};

                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component]
                fn #component_name #generics(#cx_arg, #arg) -> #return_type {
                    #block
                }

                let props = {
                    // Check if properties of the reactive type are already in the page state store
                    // If they are, we'll use them (so state persists for templates across the whole app)
                    let render_ctx = ::perseus::RenderCtx::from_ctx(cx);
                    // The render context will automatically handle prioritizing frozen or active state for us for this page as long as we have a reactive state type, which we do!
                    // We need there to be no lifetimes in `rx_props_ty` here, since the lifetimes the user decalred are defined inside the above function, which we
                    // aren't inside!
                    // We're taking normal, unwrapped types, so we use the fact that anything implementing
                    // `UnreactiveState` can be turned into `UnreactiveStateWrapper` reactively to manage this
                    match render_ctx.get_active_or_frozen_page_state::<<#props_ty as MakeRx>::Rx>(&props.path) {
                            // If we navigated back to this page, and it's still in the PSS, the given state will be a dummy, but we don't need to worry because it's never checked if this evaluates
                        ::std::option::Option::Some(existing_state) => existing_state,
                        // Again, frozen state has been dealt with already, so we'll fall back to generated state
                        ::std::option::Option::None => {
                            // Again, the render context can do the heavy lifting for us (this returns what we need, and can do type checking)
                            // We also assume that any state we have is valid because it comes from the server
                            // The user really should have a generation function, but if they don't then they'd get a panic, so give them a nice error message
                            render_ctx.register_page_state_str::<<#props_ty as MakeRx>::Rx>(&props.path, &props.state.unwrap_or_else(|| panic!("template `{}` takes a state, but no state generation functions were provided (please add at least one to use state)", #name_string))).unwrap()
                        }
                    }
                };

                // The `.make_unrx()` function will just convert back to the user's type
                ::sycamore::reactive::create_child_scope(cx, |child_cx| {
                    let view = #component_name(cx, props.make_unrx());
                    curr_view.set(view);
                });
            }
        }
    } else if fn_args.len() == 1 {
        // Get the argument for the reactive scope
        let cx_arg = &fn_args[0];
        // There are no arguments except for the scope
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(
                cx: ::sycamore::prelude::Scope,
                curr_view: &::sycamore::prelude::Signal<::sycamore::prelude::View<G>>,
                scope_disposers: &::sycamore::prelude::Signal<::std::vec::Vec<::sycamore::prelude::ScopeDisposer>>,
                props: ::perseus::template::PageProps
            ) {
                use ::perseus::state::{MakeRx, MakeRxRef};

                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component]
                fn #component_name #generics(#cx_arg) -> #return_type {
                    #block
                }

                // Declare that this page will never take any state to enable full caching
                let render_ctx = ::perseus::RenderCtx::from_ctx(cx);
                render_ctx.register_page_no_state(&props.path);

                ::sycamore::reactive::create_child_scope(cx, |child_cx| {
                    let view = #component_name(cx);
                    curr_view.set(view);
                });
            }
        }
    } else {
        // We filtered out this possibility in the function parsing
        unreachable!()
    }
}
