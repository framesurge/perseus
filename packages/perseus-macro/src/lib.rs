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
mod head;
mod rx_state;
mod state_fns;
mod template;
mod template_rx;
mod test;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use state_fns::StateFnType;
use syn::{ItemStruct, Path};

/// Annotates functions used for generating state at build time to support
/// automatic serialization/deserialization of app state and client/server
/// division. This supersedes the old `autoserde` macro for build state
/// functions.
#[proc_macro_attribute]
pub fn build_state(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as state_fns::StateFn);

    state_fns::state_fn_impl(parsed, StateFnType::BuildState).into()
}

/// Annotates functions used for generating paths at build time to support
/// automatic serialization/deserialization of app state and client/server
/// division. This supersedes the old `autoserde` macro for build paths
/// functions.
#[proc_macro_attribute]
pub fn build_paths(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as state_fns::StateFn);

    state_fns::state_fn_impl(parsed, StateFnType::BuildPaths).into()
}

/// Annotates functions used for generating global state at build time to
/// support automatic serialization/deserialization of app state and
/// client/server division. This supersedes the old `autoserde` macro for global
/// build state functions.
#[proc_macro_attribute]
pub fn global_build_state(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as state_fns::StateFn);

    state_fns::state_fn_impl(parsed, StateFnType::GlobalBuildState).into()
}

/// Annotates functions used for generating state at request time to support
/// automatic serialization/deserialization of app state and client/server
/// division. This supersedes the old `autoserde` macro for request state
/// functions.
#[proc_macro_attribute]
pub fn request_state(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as state_fns::StateFn);

    state_fns::state_fn_impl(parsed, StateFnType::RequestState).into()
}

/// Annotates functions used for generating state at build time to support
/// automatic serialization/deserialization of app state and client/server
/// division. This supersedes the old `autoserde` macro for build state
/// functions.
#[proc_macro_attribute]
pub fn set_headers(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as state_fns::StateFn);

    state_fns::state_fn_impl(parsed, StateFnType::SetHeaders).into()
}

/// Annotates functions used for amalgamating build-time and request-time states
/// to support automatic serialization/deserialization of app state and
/// client/server division. This supersedes the old `autoserde` macro for state
/// amalgamation functions.
#[proc_macro_attribute]
pub fn amalgamate_states(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as state_fns::StateFn);

    state_fns::state_fn_impl(parsed, StateFnType::AmalgamateStates).into()
}

/// Annotates functions used for checking if a template should revalidate and
/// request-time states to support automatic serialization/deserialization
/// of app state and client/server division. This supersedes the old `autoserde`
/// macro for revalidation determination functions.
#[proc_macro_attribute]
pub fn should_revalidate(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as state_fns::StateFn);

    state_fns::state_fn_impl(parsed, StateFnType::ShouldRevalidate).into()
}

/// Labels a Sycamore component as a Perseus template, turning it into something
/// that can be easily inserted into the `.template()` function, avoiding the
/// need for you to manually serialize/deserialize things. This should be
/// provided the name of the Sycamore component (same as given to Sycamore's
/// `#[component()]`, but without the `<G>`).
#[proc_macro_attribute]
pub fn template(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as template::TemplateFn);

    template::template_impl(parsed).into()
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
#[proc_macro_attribute]
pub fn template_rx(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as template_rx::TemplateFn);
    template_rx::template_impl(parsed).into()
}

/// Labels a function as a Perseus head function, which is very similar to a
/// template, but for the HTML metadata in the document `<head>`.
#[proc_macro_attribute]
pub fn head(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as head::HeadFn);

    head::head_impl(parsed).into()
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
/// This macro automatically derives `serde::Serialize` and `serde::Deserialize`
/// on the original `struct`, so do NOT add these yourself, or errors will
/// occur. Note that you can still use Serde helper macros (e.g. `#[serde(rename
/// = "testField")]`) as usual. `Clone` will also be derived on both the
/// original and the new `struct`, so do NOT try to derive it yourself.
///
/// If one of your fields is itself a `struct`, by default it will just be
/// wrapped in a `Signal`, but you can also enable nested fine-grained
/// reactivity by adding the `#[rx::nested("field_name", FieldTypeRx)]` helper
/// attribute to the `struct` (not the field, that isn't supported by Rust yet),
/// where `field_name` is the name of the field you want to use instead
/// reactivity on, and `FieldTypeRx` is the wrapper type that will be expected.
/// This should be created by using this macro on the original `struct` type.
///
/// Note that this will be deprecated or significantly altered by Sycamore's new
/// observables system (when it's released). For that reason, this doesn't
/// support more advanced features like leaving some fields unreactive, this is
/// an all-or-nothing solution for now.
///
/// # Examples
///
/// ```rust,ignore
/// use serde::{Serialize, Deserialize};
/// use perseus::make_rx;
/// // We need this trait in scope to manually invoke `.make_rx()`
/// use perseus::state::MakeRx;
///
/// #[make_rx(TestRx)]
/// // Notice that we don't need to derive `Serialize`,`Deserialize`, or `Clone`, the macro does it for us
/// #[rx::nested("nested", NestedRx)]
/// struct Test {
///     #[serde(rename = "foo_test")]
///     foo: String,
///     bar: u16,
///     // This will get simple reactivity
///     baz: Baz,
///     // This will get fine-grained reactivity
///     // We use the unreactive type in the declaration, and tell the macro what the reactive type is in the annotation above
///     nested: Nested
/// }
/// // On unreactive types, we'll need to derive `Serialize` and `Deserialize` as usual
/// #[derive(Serialize, Deserialize, Clone)]
/// struct Baz {
///     test: String
/// }
/// #[perseus_macro::make_rx(NestedRx)]
/// struct Nested {
///     test: String
/// }
///
/// let new = Test {
///     foo: "foo".to_string(),
///     bar: 5,
///     baz: Baz {
///         // We won't be able to `.set()` this
///         test: "test".to_string()
///     },
///     nested: Nested {
///         // We will be able to `.set()` this
///         test: "nested".to_string()
///     }
/// }.make_rx();
/// // Simple reactivity
/// new.bar.set(6);
/// // Simple reactivity on a `struct`
/// new.baz.set(Baz {
///     test: "updated".to_string()
/// });
/// // Nested reactivity on a `struct`
/// new.nested.test.set("updated".to_string());
/// // Our own derivations still remain
/// let _new_2 = new.clone();
/// ```
#[proc_macro_attribute]
pub fn make_rx(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as ItemStruct);
    let name = syn::parse_macro_input!(args as syn::Ident);

    rx_state::make_rx_impl(parsed, name).into()
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
