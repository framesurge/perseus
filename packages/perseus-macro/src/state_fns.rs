// This file contains all the macros that supersede `autoserde`

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Block, FnArg, Generics, Ident, Item, ItemFn,
    Result, ReturnType, Type, Visibility,
};

/// A function that can be wrapped in the Perseus test sub-harness.
pub struct StateFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// The arguments that the function takes. We don't need to modify these
    /// because we wrap them with a functin that does serializing/
    /// deserializing.
    pub args: Punctuated<FnArg, Comma>,
    /// The visibility of the function.
    pub vis: Visibility,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// The actual name of the function.
    pub name: Ident,
    /// The return type of the function.
    pub return_type: Box<Type>,
    /// Any generics the function takes (shouldn't be any, but it's possible).
    pub generics: Generics,
}
impl Parse for StateFn {
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
                // Can't be const
                if sig.constness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.constness,
                        "const functions can't be automatically serialized and deserialized for",
                    ));
                }
                // Can't be external
                if sig.abi.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.abi,
                        "external functions can't be automatically serialized and deserialized for",
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

                Ok(Self {
                    block,
                    args: sig.inputs,
                    vis,
                    attrs,
                    name: sig.ident,
                    return_type,
                    generics: sig.generics,
                })
            }
            item => Err(syn::Error::new_spanned(
                item,
                "only funtions can be used as tests",
            )),
        }
    }
}

/// The different types of state functions.
pub enum StateFnType {
    BuildState,
    BuildPaths,
    RequestState,
    SetHeaders,
    AmalgamateStates,
    GlobalBuildState,
    ShouldRevalidate,
}

// We just use a single implementation function for ease, but there's a separate
// macro for each type of state function
pub fn state_fn_impl(input: StateFn, fn_type: StateFnType) -> TokenStream {
    let StateFn {
        block,
        args,
        generics,
        vis,
        attrs,
        name,
        return_type,
    } = input;

    match fn_type {
        StateFnType::BuildState => quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            #[cfg(not(target_arch = "wasm32"))]
            #vis async fn #name(path: ::std::string::String, locale: ::std::string::String) -> ::perseus::RenderFnResultWithCause<::std::string::String> {
                // The user's function
                // We can assume the return type to be `RenderFnResultWithCause<CustomTemplatePropsType>`
                #(#attrs)*
                async fn #name #generics(#args) -> #return_type {
                    #block
                }
                // Call the user's function with the usual arguments and then serialize the result to a string
                // We only serialize the `Ok` outcome, errors are left as-is
                // We also assume that this will serialize correctly
                let build_state = #name(path, locale).await;
                let build_state_with_str = build_state.map(|val| ::serde_json::to_string(&val).unwrap());
                build_state_with_str
            }
        },
        // This one only exists to appease the server-side/client-side division
        StateFnType::BuildPaths => quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            // This normal version is identical to the user's (we know it won't have any arguments, and we know its return type)
            // We use the user's return type to prevent unused imports warnings in their code
            #[cfg(not(target_arch = "wasm32"))]
            #vis async fn #name() -> #return_type {
                #block
            }
        },
        StateFnType::RequestState => quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            #[cfg(not(target_arch = "wasm32"))]
            #vis async fn #name(path: ::std::string::String, locale: ::std::string::String, req: ::perseus::Request) -> ::perseus::RenderFnResultWithCause<::std::string::String> {
                // The user's function
                // We can assume the return type to be `RenderFnResultWithCause<CustomTemplatePropsType>`
                #(#attrs)*
                async fn #name #generics(#args) -> #return_type {
                    #block
                }
                // Call the user's function with the usual arguments and then serialize the result to a string
                // We only serialize the `Ok` outcome, errors are left as-is
                // We also assume that this will serialize correctly
                let req_state = #name(path, locale, req).await;
                let req_state_with_str = req_state.map(|val| ::serde_json::to_string(&val).unwrap());
                req_state_with_str
            }
        },
        // Always synchronous
        StateFnType::SetHeaders => quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            #[cfg(not(target_arch = "wasm32"))]
            #vis fn #name(props: ::std::option::Option<::std::string::String>) -> ::perseus::http::header::HeaderMap {
                // The user's function
                // We can assume the return type to be `HeaderMap`
                #(#attrs)*
                fn #name #generics(#args) -> #return_type {
                    #block
                }
                // Deserialize the props and then call the user's function
                let props_de = props.map(|val| ::serde_json::from_str(&val).unwrap());
                #name(props_de)
            }
        },
        // Always synchronous
        StateFnType::AmalgamateStates => quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            #[cfg(not(target_arch = "wasm32"))]
            #vis async fn #name(path: ::std::string::String, locale: ::std::string::String, build_state: ::std::string::String, request_state: ::std::string::String) -> ::perseus::RenderFnResultWithCause<::std::string::String> {
                // The user's function
                // We can assume the return type to be `RenderFnResultWithCause<Option<CustomTemplatePropsType>>`
                #(#attrs)*
                async fn #name #generics(#args) -> #return_type {
                    #block
                }
                // Deserialize both the states if they exist
                let build_state_de = ::serde_json::from_str(&build_state).unwrap();
                let request_state_de = ::serde_json::from_str(&request_state).unwrap();
                // Call the user's function with the usual arguments and then serialize the result to a string
                // We only serialize the `Ok(Some(_))` outcome, errors are left as-is
                // We also assume that this will serialize correctly
                let amalgamated_state = #name(path, locale, build_state_de, request_state_de).await;
                let amalgamated_state_with_str = amalgamated_state.map(|val| ::serde_json::to_string(&val).unwrap());
                amalgamated_state_with_str
            }
        },
        StateFnType::GlobalBuildState => quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            #[cfg(not(target_arch = "wasm32"))]
            #vis async fn #name() -> ::perseus::RenderFnResult<::std::string::String> {
                // The user's function
                // We can assume the return type to be `RenderFnResultWithCause<CustomGlobalStateType>`
                #(#attrs)*
                async fn #name #generics(#args) -> #return_type {
                    #block
                }
                // Call the user's function and then serialize the result to a string
                // We only serialize the `Ok` outcome, errors are left as-is
                // We also assume that this will serialize correctly
                let build_state = #name().await;
                let build_state_with_str = build_state.map(|val| ::serde_json::to_string(&val).unwrap());
                build_state_with_str
            }
        },
        // This one only exists to appease the server-side/client-side division
        StateFnType::ShouldRevalidate => quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            // This normal version is identical to the user's (we know it won't have any arguments, and we know its return type)
            // We use the user's return type to prevent unused imports warnings in their code
            #[cfg(not(target_arch = "wasm32"))]
            #vis async fn #name() -> #return_type {
                #block
            }
        },
    }
}
