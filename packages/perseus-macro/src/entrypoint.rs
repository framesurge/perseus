use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Block, Generics, Item, ItemFn, Result, ReturnType, Type};

/// A function that can be made into a Perseus app's entrypoint.
///
/// The signature of this function is extremely restrictive. It takes one generic for the `Html` backend to target, and produces an instance of `PerseusApp`.
pub struct MainFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// The return type of the function.
    pub return_type: Box<Type>,
    /// Any generics the function takes (shouldn't be any, but it could in theory).
    pub generics: Generics,
}
impl Parse for MainFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => {
                let ItemFn {
                    attrs, sig, block, ..
                } = func;
                // Validate each part of this function to make sure it fulfills the requirements
                // Must not be async
                if sig.asyncness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.asyncness,
                        "the entrypoint can't be async",
                    ));
                }
                // Can't be const
                if sig.constness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.constness,
                        "the entrypoint can't be a const function",
                    ));
                }
                // Can't be external
                if sig.abi.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.abi,
                        "the entrypoint can't be an external function",
                    ));
                }
                // Must return something (type checked by the existence of the wrapper code)
                let return_type = match sig.output {
                    ReturnType::Default => {
                        return Err(syn::Error::new_spanned(
                            sig,
                            "the entrypoint must return an instance of `PerseusAppBase` or one of its aliases (e.g. `PerseusApp`)",
                        ))
                    }
                    ReturnType::Type(_, ty) => ty,
                };
                // Must accept no arguments
                let inputs = sig.inputs;
                if !inputs.is_empty() {
                    return Err(syn::Error::new_spanned(
                        inputs,
                        "the entrypoint can't take any arguments",
                    ));
                }

                Ok(Self {
                    block,
                    attrs,
                    return_type,
                    generics: sig.generics,
                })
            }
            item => Err(syn::Error::new_spanned(
                item,
                "only funtions can be used as entrypoints",
            )),
        }
    }
}

pub fn main_impl(input: MainFn) -> TokenStream {
    let MainFn {
        block,
        generics,
        attrs,
        return_type,
    } = input;

    // We wrap the user's function to noramlize the name for the engine
    let output = quote! {
        pub fn __perseus_main<G: ::perseus::Html>() -> #return_type {
            // The user's function
            #(#attrs)*
            fn fn_internal#generics() -> #return_type {
                #block
            }
            fn_internal()
        }
    };

    output
}
