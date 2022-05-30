use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, FnArg, Generics, Ident, Item, ItemFn, Result, ReturnType, Type, Visibility,
};

use crate::template_rx::{get_hsr_thaw_frag, get_live_reload_frag};

/// A function that can be wrapped in the Perseus test sub-harness.
pub struct TemplateFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// The arguments to the function. One is mandatory for the reactive scope, and then there can be an optional state type.
    pub args: Vec<FnArg>,
    /// The visibility of the function.
    pub vis: Visibility,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// The actual name of the function.
    pub name: Ident,
    /// The return type of the function.
    pub return_type: Box<Type>,
    /// Any generics the function takes (should be one for the Sycamore `GenericNode`).
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
                    return Err(syn::Error::new_spanned(&sig.inputs, "template functions accept between one and two arguments (reactive scope; then one optional for custom properties)"));
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

pub fn template_impl(input: TemplateFn) -> TokenStream {
    let TemplateFn {
        block,
        args,
        generics,
        vis,
        attrs,
        name,
        return_type,
    } = input;

    let component_name = Ident::new(&(name.to_string() + "_component"), Span::call_site());

    // Set up a code fragment for responding to live reload events
    let live_reload_frag = get_live_reload_frag();
    let hsr_thaw_frag = get_hsr_thaw_frag();

    // We create a wrapper function that can be easily provided to `.template()` that does deserialization automatically if needed
    // This is dependent on what arguments the template takes
    if args.len() == 2 {
        // There's an argument that will be provided as a `String`, so the wrapper will deserialize it (also the reactive state)
        let cx_arg = &args[0];
        let arg = &args[1];

        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(cx: ::sycamore::prelude::Scope, props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                #[cfg(target_arch = "wasm32")]
                #hsr_thaw_frag

                #live_reload_frag

                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                fn #component_name #generics(#cx_arg, #arg) -> #return_type {
                    #block
                }

                // If there are props, they will always be provided, the compiler just doesn't know that
                let props = ::serde_json::from_str(&props.state.unwrap()).unwrap();

                #component_name(cx, props)
            }
        }
    } else {
        // There is one argument for the reactive scope
        let cx_arg = &args[0];

        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(cx: ::sycamore::prelude::Scope, props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                #[cfg(target_arch = "wasm32")]
                #hsr_thaw_frag

                #live_reload_frag

                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                fn #component_name #generics(#cx_arg) -> #return_type {
                    #block
                }

                #component_name(cx, ())
            }
        }
    }
}
