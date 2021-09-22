use proc_macro2::TokenStream;
use syn::{ItemFn, Block, Type, FnArg, Generics, Visibility, Ident, Attribute, Item, Result, ReturnType};
use syn::parse::{Parse, ParseStream};
use quote::{quote, ToTokens};
use darling::{FromMeta};

/// The arguments that the test annotation macro takes.
#[derive(Debug, FromMeta)]
pub struct TestArgs {
    // We'll fall back to a sensible default if no URL is given for the WebDriver
    #[darling(default)]
    webdriver_url: Option<String>,
}

/// A function that can be wrapped in the Perseus test sub-harness.
pub struct TestFn {
    /// The body of the function.
    pub block: Box<Block>,
    // The single argument for the Fantoccini client.
    pub arg: FnArg,
    /// THe visibility of the function.
    pub vis: Visibility,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// The actual name of the function.
    pub name: Ident,
    /// The return type of the function.
    pub return_type: Box<Type>,
    /// Any generics the function takes (shouldn't be any, but it could in theory).
    pub generics: Generics,
}
impl Parse for TestFn {
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
                // Must be async
                if sig.asyncness.is_none() {
                    return Err(syn::Error::new_spanned(
                        sig.asyncness,
                        "tests must be async",
                    ));
                }
                // Can't be const
                if sig.constness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.constness,
                        "const functions can't be used as tests",
                    ));
                }
                // Can't be external
                if sig.abi.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.abi,
                        "external functions can't be used as tests",
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
                // Must accept a single argument for the Fantoccini client
                let mut inputs = sig.inputs.into_iter();
                let arg: FnArg = inputs.next().unwrap_or_else(|| syn::parse_quote! { _: () });
                match &arg {
                    FnArg::Typed(_) => (),
                    // Can't accept `self`
                    FnArg::Receiver(arg) => {
                        return Err(syn::Error::new_spanned(
                            arg,
                            "test functions can't take `self`",
                        ))
                    }
                };

                if inputs.len() > 0 {
                    let params: TokenStream = inputs.map(|it| it.to_token_stream()).collect();
                    return Err(syn::Error::new_spanned(
                        params,
                        "test functions must accept a single argument for the Fantoccini client",
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

pub fn test_impl(input: TestFn, args: TestArgs) -> TokenStream {
    let TestFn {
        block,
        arg,
        generics,
        vis,
        attrs,
        name,
        return_type,
    } = input;

    // Get the WebDriver URL to use from the macro arguments, or use a sensible default
    let webdriver_url = args.webdriver_url.unwrap_or_else(|| "http://localhost:4444".to_string());

    // We create a wrapper function that handles errors and the Fantoccini client
    let output = quote! {
        #[::tokio::test]
        #vis async fn #name() {
            // The user's function
            #(#attrs)*
            async fn fn_internal#generics(#arg) -> #return_type {
                #block
            }
            // Only run the test if the environment variable is specified (avoids having to do exclusions for workspace `cargo test`)
            if ::std::env::var("PERSEUS_RUN_WASM_TESTS").is_ok() {
                let headless = ::std::env::var("PERSEUS_RUN_WASM_TESTS_HEADLESS").is_ok();
                // Set the capabilities of the client
                // If the user wants different capabilities, they should break out of this macro and use Fantoccini directly
                let mut capabilities = ::serde_json::Map::new();
                let firefox_opts;
                let chrome_opts;
                if headless {
                    firefox_opts = ::serde_json::json!({ "args": ["--headless"] });
                    chrome_opts = ::serde_json::json!({ "args": ["--headless"] });
                } else {
                    firefox_opts = ::serde_json::json!({ "args": [] });
                    chrome_opts = ::serde_json::json!({ "args": [] });
                }
                capabilities.insert("moz:firefoxOptions".to_string(), firefox_opts);
                capabilities.insert("goog:chromeOptions".to_string(), chrome_opts);

                let mut client = ::fantoccini::ClientBuilder::native()
                    .capabilities(capabilities)
                    .connect(&#webdriver_url).await.expect("failed to connect to WebDriver");
                let output = fn_internal(&mut client).await;
                // Close the client no matter what
                client.close().await.expect("failed to close Fantoccini client");
                // Panic if the test failed
                if let Err(err) = output {
                    panic!("test failed: '{}'", err.to_string())
                }
            }
        }
    };

    output
}
