use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, FnArg, Generics, Ident, Item, ItemFn, Result, ReturnType, Type, Visibility,
};

/// The arguments that the `autoserde` annotation takes.
#[derive(Debug, FromMeta)]
pub enum AutoserdeArgs {
    #[darling(rename = "build_state")]
    BuildState,
    #[darling(rename = "request_state")]
    RequestState,
    #[darling(rename = "set_headers")]
    SetHeaders,
    #[darling(rename = "amalgamate_states")]
    AmalgamateStates,
}

/// A function that can be wrapped in the Perseus test sub-harness.
pub struct AutoserdeFn {
    /// The body of the function.
    pub block: Box<Block>,
    // The possible single argument for custom properties, or there might be no arguments.
    pub arg: Option<FnArg>,
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
impl Parse for AutoserdeFn {
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
                // Must return `std::result::Result<(), fantoccini::error::CmdError>`
                let return_type = match sig.output {
                    ReturnType::Default => {
                        return Err(syn::Error::new_spanned(
                            sig,
                            "test function must return `std::result::Result<(), fantoccini::error::CmdError>`",
                        ))
                    }
                    ReturnType::Type(_, ty) => ty,
                };
                // Can either accept a single argument for properties or no arguments
                let mut inputs = sig.inputs.into_iter();
                let arg: Option<FnArg> = inputs.next();
                // We don't care what the type is, as long as it's not `self`
                if let Some(FnArg::Receiver(arg)) = arg {
                    return Err(syn::Error::new_spanned(arg, "templates can't take `self`"));
                }

                // This operates on what's left over after calling `.next()`
                if inputs.len() > 0 {
                    let params: TokenStream = inputs.map(|it| it.to_token_stream()).collect();
                    return Err(syn::Error::new_spanned(
                        params,
                        "test functions must accept either one argument for custom properties or no arguments",
                    ));
                }

                Ok(Self {
                    block,
                    arg,
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

pub fn autoserde_impl(input: AutoserdeFn, args: AutoserdeArgs) -> TokenStream {
    // let AutoserdeFn {
    //     block,
    //     arg,
    //     generics,
    //     vis,
    //     attrs,
    //     name,
    //     return_type,
    // } = input;

    todo!()
}
