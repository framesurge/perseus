#![doc = include_str!("../README.proj.md")]
/*!
## Features

- `live-reload` -- enables reloading the browser automatically when you make changes to your app
- `hsr` -- enables *hot state reloading*, which reloads the state of your app right before you made code changes in development, allowing you to pick up where you left off

## Packages

This is the API documentation for the `perseus-macro` package, which manages Perseus' procedural macros. Note that Perseus mostly uses [the book](https://arctic-hen7.github.io/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/arctic-hen7/perseus/tree/main/examples).
*/

mod entrypoint;
mod rx_state;
mod template;
mod test;

use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, Path, Signature};

use crate::rx_state::ReactiveStateDeriveInput;

// TODO(0.5.x) Remove this entirely
#[doc(hidden)]
#[proc_macro_attribute]
pub fn template_rx(_args: TokenStream, _input: TokenStream) -> TokenStream {
    quote! {
        compile_error!("the `template_rx` macro has been replaced by the `template` macro")
    }
    .into()
}

/// The new version of `#[template]` designed for reactive state. This can
/// interface automatically with global state, and will automatically provide
/// Sycamore `#[component]` annotations. To use this, you don't need to provide
/// anything other than an optional custom type parameter letter (by default,
/// `G` will be used). Unlike with the original macro, this will automatically
/// handle component names internally.
///
/// The first argument your template function can take is state generated for it
/// (e.g. by the *build state* strategy), but the reactive version (created with
/// `#[make_rx]` usually). From this, Perseus can infer the other required types
/// and automatically make your state reactive for you.
///
/// The second argument your template function can take is a global state
/// generated with the `GlobalStateCreator`. You should also provide the
/// reactive type here, and Perseus will do all the rest in the background.
///
/// Labels a function as a Perseus template, automatically managing its state
/// and integrating it into your app. Functions annotated with this macro
/// take at least one argument for Sycamore's reactive scope, and then a
/// possible other argument for some state they generate with a rendering
/// strategy (e.g. *build state*, generated when you build your app, see the
/// book for more). That state is expected to be reactive (see [`make_rx`]),
/// although, if you use `#[template(unreactive)]`, you can use any state that
/// has been annotated with [`UnreactiveState`] to make it clear to Perseus not
/// to expect something reactive.
///
/// Although you can make a Perseus app without using this macro, this isn't
/// recommended, since Perseus passes around state in your app as `String`s and
/// `dyn Any`s, meaning there is a large amount of overhead to actually using
/// the state you expect. This macro will automatically handle all that overhead
/// for you, making the process of building your app *significantly* smoother!
///
/// *Note: in previous versions of Perseus, there was a `template_rx` macro,
/// which has become this. The old unreactive `template` macro has become
/// `#[template(unreactive)]`. For those used to using Sycamore `#[component]`
/// annotation on their pages, this is no longer required. Note also that global
/// state is now accessed through the `.get_global_state()` method on Perseus'
/// `RenderCtx`.*
#[proc_macro_attribute]
pub fn template(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as template::TemplateFn);
    let is_reactive = args.to_string() != "unreactive";
    template::template_impl(parsed, is_reactive).into()
}

/// Marks the given function as a Perseus test. Functions marked with this
/// attribute must have the following signature: `async fn foo(client: &mut
/// fantoccini::Client) -> Result<>`.
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as test::TestFn);
    let attr_args = syn::parse_macro_input!(args as syn::AttributeArgs);
    // Parse macro arguments with `darling`
    let args = match test::TestArgs::from_list(&attr_args) {
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
/// will just be something like `perseus_warp::dflt_server` (with `perseus-warp`
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

/// Marks the annotated code as only to be run as part of the engine (the
/// server, the builder, the exporter, etc.). This resolves to a target-gate
/// that makes the annotated code run only on targets that are not `wasm32`.
#[proc_macro_attribute]
pub fn engine(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_2: proc_macro2::TokenStream = input.into();
    quote! {
        #[cfg(not(target_arch = "wasm32"))]
        #input_2
    }
    .into()
}

/// Marks the annotated code as only to be run in the browser. This is the
/// opposite of (and mutually exclusive with) `#[engine]`. This resolves to a
/// target-gate that makes the annotated code run only on targets that are
/// `wasm32`.
#[proc_macro_attribute]
pub fn browser(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_2: proc_macro2::TokenStream = input.into();
    quote! {
        #[cfg(target_arch = "wasm32")]
        #input_2
    }
    .into()
}

/// A convenience macro that makes sure the given function is only defined on
/// the engine-side, creating an empty function on the browser-side. Perseus
/// implicitly expects most of your state generation functions to be defined in
/// this way (though you certainly don't have to use this macro).
///
/// Note that this will convert `async` functions to non-`async` functions.
#[proc_macro_attribute]
pub fn engine_only_fn(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_2: proc_macro2::TokenStream = input.clone().into();
    let ItemFn {
        vis,
        sig: Signature { ident, .. },
        ..
    } = parse_macro_input!(input as ItemFn);

    quote! {
        #[cfg(target_arch = "wasm32")]
        #vis fn #ident () {}
        // One the engine-side, the function is unmodified
        #[cfg(not(target_arch = "wasm32"))]
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
