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
                    return Err(syn::Error::new_spanned(&sig.inputs, "template functions accept between one and three arguments (reactive scope; then one for custom properties and another for global state, both optional)"));
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
                "only funtions can be used as templates",
            )),
        }
    }
}

/// Converts the user-given name of a final reactive `struct` into the
/// intermediary name used for the one we'll interface with. This will remove
/// any associated lifetimes because we want just the type name. This will leave
/// generics intact though.
fn make_mid(ty: &Type) -> Type {
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
            // And now actually make the replacement we need (ref to intermediate)
            let ty_str = ty_str.trim().to_string() + "PerseusRxIntermediary";
            Type::Verbatim(TokenStream::from_str(&ty_str).unwrap())
        }
    }
}

pub fn template_impl(input: TemplateFn) -> TokenStream {
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

    // We create a wrapper function that can be easily provided to `.template()`
    // that does deserialization automatically if needed This is dependent on
    // what arguments the template takes
    if fn_args.len() == 3 {
        // Get the argument for the reactive scope
        let cx_arg = &fn_args[0];
        // There's an argument for page properties that needs to have state extracted,
        // so the wrapper will deserialize it We'll also make it reactive and
        // add it to the page state store
        let state_arg = &fn_args[1];
        let rx_props_ty = match state_arg {
            FnArg::Typed(PatType { ty, .. }) => make_mid(&**ty),
            FnArg::Receiver(_) => unreachable!(),
        };
        // There's also a second argument for the global state, which we'll deserialize
        // and make global if it's not already (aka. if any other pages have loaded
        // before this one) Sycamore won't let us have more than one argument to
        // a component though, so we sneakily extract it and literally construct it as a
        // variable (this should be fine?)
        let global_state_arg = &fn_args[2];
        let (global_state_arg_pat, global_state_rx) = match global_state_arg {
            FnArg::Typed(PatType { pat, ty, .. }) => (pat, make_mid(&**ty)),
            FnArg::Receiver(_) => unreachable!(),
        };
        let name_string = name.to_string();
        // Handle the case in which the template is just using global state and the
        // first argument is the unit type That's represented for Syn as a typle
        // with no elements
        match rx_props_ty {
            // This template takes dummy state and global state
            Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty() => quote! {
                #vis fn #name<G: ::sycamore::prelude::Html>(cx: ::sycamore::prelude::Scope, props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                    use ::perseus::state::MakeRx;

                    let render_ctx = ::perseus::get_render_ctx!(cx);
                    // Get the frozen or active global state (the render context manages thawing preferences)
                    // This isn't completely pointless, this method mutates as well to set up the global state as appropriate
                    // If there's no active or frozen global state, then we'll fall back to the generated one from the server (which we know will be there, since if this is `None` we must be
                    // the first page to access the global state).
                    if render_ctx.get_active_or_frozen_global_state::<#global_state_rx>().is_none() {
                        // Because this came from the server, we assume it's valid
                        render_ctx.register_global_state_str::<#global_state_rx>(&props.global_state.unwrap()).unwrap();
                    }

                    // The user's function
                    // We know this won't be async because Sycamore doesn't allow that
                    #(#attrs)*
                    #[::sycamore::component]
                    // WARNING: I removed the `#state_arg` here because the new Sycamore throws errors for unit type props (possible consequences?)
                    fn #component_name #generics(#cx_arg) -> #return_type {
                        let __perseus_global_state_intermediate: #global_state_rx = {
                            let global_state = ::perseus::get_render_ctx!(cx).global_state.0.borrow();
                            // We can guarantee that it will downcast correctly now, because we'll only invoke the component from this function, which sets up the global state correctly
                            let global_state_ref = global_state.as_any().downcast_ref::<#global_state_rx>().unwrap();
                            (*global_state_ref).clone()
                        };
                        let #global_state_arg_pat = __perseus_global_state_intermediate.to_ref_struct(cx);
                        #block
                    }

                    #component_name(cx)
                }
            },
            // This template takes its own state and global state
            _ => quote! {
                #vis fn #name<G: ::sycamore::prelude::Html>(cx: ::sycamore::prelude::Scope, props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                    use ::perseus::state::MakeRx;

                    let render_ctx = ::perseus::get_render_ctx!(cx).clone();
                    // Get the frozen or active global state (the render context manages thawing preferences)
                    // This isn't completely pointless, this method mutates as well to set up the global state as appropriate
                    // If there's no active or frozen global state, then we'll fall back to the generated one from the server (which we know will be there, since if this is `None` we must be
                    // the first page to access the global state).
                    if render_ctx.get_active_or_frozen_global_state::<#global_state_rx>().is_none() {
                        // Because this came from the server, we assume it's valid
                        render_ctx.register_global_state_str::<#global_state_rx>(&props.global_state.unwrap()).unwrap();
                    }

                    // The user's function
                    // We know this won't be async because Sycamore doesn't allow that
                    #(#attrs)*
                    #[::sycamore::component]
                    fn #component_name #generics(#cx_arg, #state_arg) -> #return_type {
                        let #global_state_arg_pat: #global_state_rx = {
                            let global_state = ::perseus::get_render_ctx!(cx).global_state.0.borrow();
                            // We can guarantee that it will downcast correctly now, because we'll only invoke the component from this function, which sets up the global state correctly
                            let global_state_ref = global_state.as_any().downcast_ref::<#global_state_rx>().unwrap();
                            (*global_state_ref).clone()
                        };
                        let #global_state_arg_pat = #global_state_arg_pat.to_ref_struct(cx);
                        #block
                    }

                    let props = {
                        // Check if properties of the reactive type are already in the page state store
                        // If they are, we'll use them (so state persists for templates across the whole app)
                        let render_ctx = ::perseus::get_render_ctx!(cx);
                        // The render context will automatically handle prioritizing frozen or active state for us for this page as long as we have a reactive state type, which we do!
                        match render_ctx.get_active_or_frozen_page_state::<#rx_props_ty>(&props.path) {
                            ::std::option::Option::Some(existing_state) => existing_state,
                            // Again, frozen state has been dealt with already, so we'll fall back to generated state
                            ::std::option::Option::None => {
                                // Again, the render context can do the heavy lifting for us (this returns what we need, and can do type checking)
                                // We also assume that any state we have is valid because it comes from the server
                                // The user really should have a generation function, but if they don't then they'd get a panic, so give them a nice error message
                                render_ctx.register_page_state_str::<#rx_props_ty>(&props.path, &props.state.unwrap_or_else(|| panic!("template `{}` takes a state, but no state generation functions were provided (please add at least one to use state)", #name_string))).unwrap()
                            }
                        }
                    };

                    #component_name(cx, props.to_ref_struct(cx))
                }
            },
        }
    } else if fn_args.len() == 2 {
        // Get the argument for the reactive scope
        let cx_arg = &fn_args[0];
        // There's an argument for page properties that needs to have state extracted,
        // so the wrapper will deserialize it We'll also make it reactive and
        // add it to the page state store
        let arg = &fn_args[1];
        let rx_props_ty = match arg {
            FnArg::Typed(PatType { ty, .. }) => make_mid(&**ty),
            FnArg::Receiver(_) => unreachable!(),
        };
        let name_string = name.to_string();
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(cx: ::sycamore::prelude::Scope, props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                use ::perseus::state::MakeRx;

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
                    let render_ctx = ::perseus::get_render_ctx!(cx);
                    // The render context will automatically handle prioritizing frozen or active state for us for this page as long as we have a reactive state type, which we do!
                    match render_ctx.get_active_or_frozen_page_state::<#rx_props_ty>(&props.path) {
                        ::std::option::Option::Some(existing_state) => existing_state,
                        // Again, frozen state has been dealt with already, so we'll fall back to generated state
                        ::std::option::Option::None => {
                            // Again, the render context can do the heavy lifting for us (this returns what we need, and can do type checking)
                            // We also assume that any state we have is valid because it comes from the server
                            // The user really should have a generation function, but if they don't then they'd get a panic, so give them a nice error message
                            render_ctx.register_page_state_str::<#rx_props_ty>(&props.path, &props.state.unwrap_or_else(|| panic!("template `{}` takes a state, but no state generation functions were provided (please add at least one to use state)", #name_string))).unwrap()
                        }
                    }
                };

                #component_name(cx, props.to_ref_struct(cx))
            }
        }
    } else if fn_args.len() == 1 {
        // Get the argument for the reactive scope
        let cx_arg = &fn_args[0];
        // There are no arguments except for the scope
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(cx: ::sycamore::prelude::Scope, props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                use ::perseus::state::MakeRx;

                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component]
                fn #component_name #generics(#cx_arg) -> #return_type {
                    #block
                }

                #component_name(cx)
            }
        }
    } else {
        // We filtered out this possibility in the function parsing
        unreachable!()
    }
}
