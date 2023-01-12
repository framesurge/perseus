use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Block, Generics, Item, ItemFn, Path, Result, ReturnType, Type};

/// A function that can be made into a Perseus app's entrypoint.
///
/// The signature of this function is extremely restrictive. It takes one
/// generic for the `Html` backend to target, and produces an instance of
/// `PerseusApp`.
pub struct MainFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// The return type of the function.
    pub return_type: Box<Type>,
    /// Any generics the function takes.
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
                "only functions can be used as entrypoints",
            )),
        }
    }
}

/// An async function that can be made into a Perseus app's entrypoint.
/// (Specifically, the engine entrypoint.)
pub struct EngineMainFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// Any generics the function takes (shouldn't be any, but it could in
    /// theory).
    pub generics: Generics,
}
impl Parse for EngineMainFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => {
                let ItemFn {
                    attrs, sig, block, ..
                } = func;
                // Validate each part of this function to make sure it fulfills the requirements
                // Must not be async
                if sig.asyncness.is_none() {
                    return Err(syn::Error::new_spanned(
                        sig.asyncness,
                        "the engine entrypoint must be async",
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
                match sig.output {
                    ReturnType::Default => (),
                    ReturnType::Type(_, _) => {
                        return Err(syn::Error::new_spanned(
                            sig,
                            "the engine entrypoint must have no return value",
                        ))
                    }
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
                    generics: sig.generics,
                })
            }
            item => Err(syn::Error::new_spanned(
                item,
                "only functions can be used as entrypoints",
            )),
        }
    }
}

pub fn main_impl(input: MainFn, server_fn: Path) -> TokenStream {
    let MainFn {
        block,
        generics,
        attrs,
        return_type,
    } = input;

    // We split the user's function out into one for the browser and one for the
    // engine (all based around the default engine)
    let output = quote! {
        // The engine-specific `main` function
        #[cfg(engine)]
        #[tokio::main]
        async fn main() {
            // Get the operation we're supposed to run (serve, build, export, etc.) from an environment variable
            let op = ::perseus::engine::get_op().unwrap();
            let exit_code = ::perseus::engine::run_dflt_engine(op, __perseus_simple_main, #server_fn).await;
            std::process::exit(exit_code);
        }

        // The browser-specific `main` function
        #[cfg(client)]
        pub fn main() -> ::perseus::client::ClientReturn {
            ::perseus::client::run_client(__perseus_simple_main);
            Ok(())
        }

        // The user's function (which gets the `PerseusApp`)
        #(#attrs)*
        #[doc(hidden)]
        pub fn __perseus_simple_main #generics() -> #return_type {
            #block
        }
    };

    output
}

pub fn main_export_impl(input: MainFn) -> TokenStream {
    let MainFn {
        block,
        generics,
        attrs,
        return_type,
    } = input;

    // We split the user's function out into one for the browser and one for the
    // engine (all based around the default engine)
    let output = quote! {
        // The engine-specific `main` function
        #[cfg(engine)]
        #[tokio::main]
        async fn main() {
            // Get the operation we're supposed to run (serve, build, export, etc.) from an environment variable
            let op = ::perseus::engine::get_op().unwrap();
            let exit_code = ::perseus::engine::run_dflt_engine_export_only(op, __perseus_simple_main).await;
            std::process::exit(exit_code);
        }

        // The browser-specific `main` function
        #[cfg(client)]
        pub fn main() -> ::perseus::ClientReturn {
            ::perseus::run_client(__perseus_simple_main);
            Ok(())
        }

        // The user's function (which gets the `PerseusApp`)
        #(#attrs)*
        #[doc(hidden)]
        pub fn __perseus_simple_main #generics() -> #return_type {
            #block
        }
    };

    output
}

pub fn browser_main_impl(input: MainFn) -> TokenStream {
    let MainFn {
        block,
        attrs,
        return_type,
        ..
    } = input;

    // We split the user's function out into one for the browser and one for the
    // engine (all based around the default engine)
    let output = quote! {
        // The browser-specific `main` function
        // This absolutely MUST be called `main`, otherwise the hardcodes Wasm importer will fail (and then interactivity is gone completely with a really weird error message)
        #[cfg(client)]
        #(#attrs)*
        pub fn main() -> #return_type {
            #block
        }
    };

    output
}

pub fn engine_main_impl(input: EngineMainFn) -> TokenStream {
    let EngineMainFn { block, attrs, .. } = input;

    // We split the user's function out into one for the browser and one for the
    // engine (all based around the default engine)
    let output = quote! {
        // The engine-specific `main` function
        #[cfg(engine)]
        #[tokio::main]
        #(#attrs)*
        async fn main() {
            #block
        }
    };

    output
}
