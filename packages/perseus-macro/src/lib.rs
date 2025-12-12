#![doc = include_str!("../README.proj.md")]
/*!
## Features

- `live-reload` -- enables reloading the browser automatically when you make changes to your app
- `hsr` -- enables *hot state reloading*, which reloads the state of your app right before you made code changes in development, allowing you to pick up where you left off

## Packages

This is the API documentation for the `perseus-macro` package, which manages Perseus' procedural macros. Note that Perseus mostly uses [the book](https://framesurge.sh/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/framesurge/perseus/tree/main/examples).
*/

mod auto_scope;
mod entrypoint;
mod rx_state;
mod test;

use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, Path, Signature};

use crate::rx_state::ReactiveStateDeriveInput;

/// A helper macro for templates that use reactive state. This macro is now
/// largely optional in Sycamore 0.9+, as the reactivity system no longer
/// requires explicit scope parameters or lifetime annotations.
///
/// Templates that take no state, or templates that take unreactive state, can
/// be provided as normal functions to the methods `.view()` and
/// `.view_with_unreactive_state()` respectively, on Perseus' `Template` type.
///
/// This macro primarily exists for consistency and validation. It ensures your
/// template function has the correct signature when using reactive state.
///
/// **Example usage:**
///
/// ```text
/// #[auto_scope]
/// fn my_page(state: &MyStateRx) -> View {
///     // Your template code here
/// }
/// ```
///
/// **What it does:**
/// - Validates that the function signature is correct for a reactive template
/// - Ensures the state parameter is a reference
/// - Provides clear error messages if the signature is incorrect
///
/// In Sycamore 0.9.2, the reactivity system is much simpler:
/// - No scope parameter needed (signals are now `'static` and `Copy`)
/// - No generic type parameters (View no longer needs `<G: Html>`)
/// - No explicit lifetime annotations required
///
/// If you dislike macros, you can write the function signature directly
/// without this macro - it's purely a convenience and validation tool.
///
/// *Note: this can also be used for capsules that take reactive state, it's not
/// just limited to templates.*
#[proc_macro_attribute]
pub fn auto_scope(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as auto_scope::TemplateFn);
    auto_scope::template_impl(parsed).into()
}

/// Marks the given function as a Perseus test. Functions marked with this
/// attribute must have the following signature: `async fn foo(client: &mut
/// fantoccini::Client) -> Result<>`.
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as test::TestFn);
    let attr_args = syn::parse_macro_input!(args with syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated);
    // Convert Meta to NestedMeta for darling
    let nested_meta: Vec<_> = attr_args
        .into_iter()
        .map(|meta| darling::ast::NestedMeta::Meta(meta))
        .collect();
    // Parse macro arguments with `darling`
    let args = match test::TestArgs::from_list(&nested_meta) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    test::test_impl(parsed, args).into()
}

/// Marks the given function as the universal entrypoint into your app. This is
/// designed for simple use-cases, and the annotated function should return
/// a `PerseusApp`. This will expand into separate `main()` functions for both
/// the browser and engine sides.
///
/// This should take an argument for the function that will produce your server.
/// In most apps using this macro (which is designed for simple use-cases), this
/// will just be something like `perseus_axum::dflt_server` (with `perseus-warp`
/// as a dependency with the `dflt-server` feature enabled).
///
/// Note that the `dflt-engine` and `client-helpers` features must be enabled on
/// `perseus` for this to work. (These are enabled by default.)
///
/// Note further that you'll need to have `wasm-bindgen` as a dependency to use
/// this.
#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as entrypoint::MainFn);
    let args = syn::parse_macro_input!(args as Path);

    entrypoint::main_impl(parsed, args).into()
}

/// This is identical to `#[main]`, except it doesn't require a server
/// integration, because it sets your app up for exporting only. This is useful
/// for apps not using server-requiring features (like incremental static
/// generation and revalidation) that want to avoid bringing in another
/// dependency on the server-side.
#[proc_macro_attribute]
pub fn main_export(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as entrypoint::MainFn);

    entrypoint::main_export_impl(parsed).into()
}

/// Marks the given function as the browser entrypoint into your app. This is
/// designed for more complex apps that need to manually distinguish between the
/// engine and browser entrypoints.
///
/// If you just want to run some simple customizations, you should probably use
/// `perseus::run_client` to use the default client logic after you've made your
/// modifications. `perseus::ClientReturn` should be your return type no matter
/// what.
///
/// Note that any generics on the annotated function will not be preserved. You
/// should put the `PerseusApp` generator in a separate function.
///
/// Note further that you'll need to have `wasm-bindgen` as a dependency to use
/// this.
#[proc_macro_attribute]
pub fn browser_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as entrypoint::MainFn);

    entrypoint::browser_main_impl(parsed).into()
}

/// Marks the given function as the engine entrypoint into your app. This is
/// designed for more complex apps that need to manually distinguish between the
/// engine and browser entrypoints.
///
/// If you just want to run some simple customizations, you should probably use
/// `perseus::run_dflt_engine` with `perseus::builder::get_op` to use the
/// default client logic after you've made your modifications. You'll also want
/// to return an exit code from this function (use `std::process:exit(..)`).
///
/// Note that the `dflt-engine` and `client-helpers` features must be enabled on
/// `perseus` for this to work. (These are enabled by default.)
///
/// Note further that you'll need to have `tokio` as a dependency to use this.
///
/// Finally, note that any generics on the annotated function will not be
/// preserved. You should put the `PerseusApp` generator in a separate function.
#[proc_macro_attribute]
pub fn engine_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as entrypoint::EngineMainFn);

    entrypoint::engine_main_impl(parsed).into()
}

/// Processes the given `struct` to create a reactive version by wrapping each
/// field in a `Signal`. This will generate a new `struct` with the given name
/// and implement a `.make_rx()` method on the original that allows turning an
/// instance of the unreactive `struct` into an instance of the reactive one.
///
/// If one of your fields is itself a `struct`, by default it will just be
/// wrapped in a `Signal`, but you can also enable nested fine-grained
/// reactivity by adding the `#[rx(nested)]` helper macro to the field.
/// Fields that have nested reactivity should also use this derive macro.
#[proc_macro_derive(ReactiveState, attributes(rx))]
pub fn reactive_state(input: TokenStream) -> TokenStream {
    let input = match ReactiveStateDeriveInput::from_derive_input(&syn::parse_macro_input!(
        input as DeriveInput
    )) {
        Ok(input) => input,
        Err(err) => return err.write_errors().into(),
    };

    rx_state::make_rx_impl(input).into()
}

/// A convenience macro that makes sure the given function is only defined on
/// the engine-side, creating an empty function on the browser-side. Perseus
/// implicitly expects most of your state generation functions to be defined in
/// this way (though you certainly don't have to use this macro).
///
/// Note that this will convert `async` functions to non-`async` functions on
/// the browser-side (your function will be left alone on the engine-side).
#[proc_macro_attribute]
pub fn engine_only_fn(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_2: proc_macro2::TokenStream = input.clone().into();
    let ItemFn {
        vis,
        sig: Signature { ident, .. },
        ..
    } = parse_macro_input!(input as ItemFn);

    quote! {
        #[cfg(client)]
        #vis fn #ident () {}
        // On the engine-side, the function is unmodified
        #[cfg(engine)]
        #input_2
    }
    .into()
}

/// A convenience macro that makes sure the given function is only defined on
/// the browser-side, creating an empty function on the engine-side. Perseus
/// implicitly expects your browser-side state modification functions to be
/// defined in this way (though you certainly don't have to use this macro).
///
/// Note that this will convert `async` functions to non-`async` functions on
/// the engine-side (your function will be left alone on the browser-side).
#[proc_macro_attribute]
pub fn browser_only_fn(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_2: proc_macro2::TokenStream = input.clone().into();
    let ItemFn {
        vis,
        sig: Signature { ident, .. },
        ..
    } = parse_macro_input!(input as ItemFn);

    quote! {
        #[cfg(engine)]
        #vis fn #ident () {}
        // One the browser-side, the function is unmodified
        #[cfg(client)]
        #input_2
    }
    .into()
}

#[proc_macro_derive(UnreactiveState)]
pub fn unreactive_state(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // This is a marker trait, so we barely have to do anything here
    quote! {
        impl ::perseus::state::UnreactiveState for #name {}
    }
    .into()
}
