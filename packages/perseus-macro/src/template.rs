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
                // We can have 2 arguments only (scope, state)
                // Any other kind of template doesn't need this macro
                if args.len() != 2 {
                    return Err(syn::Error::new_spanned(&sig.inputs, "you only need to use `[#template]` if you're using reactive state (which requires two arguments)"));
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
    quote! {
        #vis fn #name<G: ::sycamore::prelude::Html>(
            cx: ::sycamore::prelude::Scope,
            state: <#rx_props_ty as ::perseus::state::RxRef>::RxNonRef
        ) -> #return_type {
            use ::perseus::state::MakeRxRef;

            // The user's function, with Sycamore component annotations and the like preserved
            // We know this won't be async because Sycamore doesn't allow that
            #(#attrs)*
            #[::sycamore::component]
            fn #component_name #generics(#cx_arg, #arg) -> #return_type {
                #block
            }

            #component_name(cx, state.to_ref_struct(cx))
        }
    }
}
