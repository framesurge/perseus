use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, FnArg, Generics, Ident, Item, ItemFn, Result, ReturnType, Type, Visibility,
};

/// A function that can be wrapped in the Perseus test sub-harness.
pub struct HeadFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// The argument for the reactive scope.
    pub cx_arg: FnArg,
    /// The possible single argument for custom properties, or there might be no
    /// arguments.
    pub arg: Option<FnArg>,
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
impl Parse for HeadFn {
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
                        "head functions cannot be asynchronous",
                    ));
                }
                // Can't be const
                if sig.constness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.constness,
                        "const functions can't be used as head functions",
                    ));
                }
                // Can't be external
                if sig.abi.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.abi,
                        "external functions can't be used as head functions",
                    ));
                }
                // Must have an explicit return type
                let return_type = match sig.output {
                    ReturnType::Default => {
                        return Err(syn::Error::new_spanned(
                            sig,
                            "head functions can't return default/inferred type",
                        ))
                    }
                    ReturnType::Type(_, ty) => ty,
                };
                // Can accept two arguments (scope, properties), or one (scope only)
                let mut inputs = sig.inputs.into_iter();
                let cx_arg = match inputs.next() {
                    Some(arg) => arg,
                    None => {
                        let params: TokenStream = inputs.map(|it| it.to_token_stream()).collect();
                        return Err(syn::Error::new_spanned(
                            params,
                            "head functions must accept an argument for the reactive scope",
                        ));
                    }
                };
                let arg = inputs.next();
                // We don't care what the type is, as long as it's not `self`
                if let Some(FnArg::Receiver(arg)) = arg {
                    return Err(syn::Error::new_spanned(
                        arg,
                        "head functions can't take `self`",
                    ));
                }

                // This operates on what's left over after calling `.next()`
                if inputs.len() > 0 {
                    let params: TokenStream = inputs.map(|it| it.to_token_stream()).collect();
                    return Err(syn::Error::new_spanned(
                        params,
                        "head functions must accept either two arguments for scope, and custom properties or just one argument for scope",
                    ));
                }

                Ok(Self {
                    block,
                    cx_arg,
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
                "only functions can be used as head functions",
            )),
        }
    }
}

pub fn head_impl(input: HeadFn) -> TokenStream {
    let HeadFn {
        block,
        cx_arg,
        arg,
        generics,
        vis,
        attrs,
        name,
        return_type,
    } = input;

    // We create a wrapper function that can be easily provided to `.head()` that
    // does deserialization automatically if needed This is dependent on what
    // arguments the template takes
    if arg.is_some() {
        // There's an argument that will be provided as a `String`, so the wrapper will
        // deserialize it
        quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            #[cfg(not(target_arch = "wasm32"))]
            #vis fn #name(cx: ::sycamore::prelude::Scope, props: ::perseus::template::PageProps) -> ::sycamore::prelude::View<::sycamore::prelude::SsrNode> {
                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                fn #name #generics(#cx_arg, #arg) -> #return_type {
                    #block
                }
                #name(
                    cx,
                    // If there are props, they will always be provided, the compiler just doesn't know that
                    ::serde_json::from_str(&props.state.unwrap()).unwrap()
                )
            }
        }
    } else {
        // There are no arguments
        quote! {
            // We create a normal version of the function and one to appease the handlers in Wasm (which expect functions that take no arguments, etc.)
            #[cfg(target_arch = "wasm32")]
            #vis fn #name() {}
            #[cfg(not(target_arch = "wasm32"))]
            #vis fn #name(cx: ::sycamore::prelude::Scope, props: ::perseus::template::PageProps) -> ::sycamore::prelude::View<::sycamore::prelude::SsrNode> {
                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                fn #name #generics(#cx_arg) -> #return_type {
                    #block
                }
                #name(cx)
            }
        }
    }
}
