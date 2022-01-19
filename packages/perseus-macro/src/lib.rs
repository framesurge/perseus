/*!
 * Perseus is a blazingly fast frontend web development framework built in Rust with support for major rendering strategies,
 * reactivity without a virtual DOM, and extreme customizability. It wraps the lower-level capabilities of [Sycamore](https://github.com/sycamore-rs/sycamore)
 * and provides a NextJS-like API!
 *
 * - ✨ Supports static generation (serving only static resources)
 * - ✨ Supports server-side rendering (serving dynamic resources)
 * - ✨ Supports revalidation after time and/or with custom logic (updating rendered pages)
 * - ✨ Supports incremental regeneration (build on demand)
 * - ✨ Open build matrix (use any rendering strategy with anything else, mostly)
 * - ✨ CLI harness that lets you build apps with ease and confidence
 *
 * This is the documentation for the Perseus macros, but there's also [a CLI](https://arctic-hen7.github.io/perseus/cli.html),
 * [the core package](https://crates.io/crates/perseus), and other [integrations](https://arctic-hen7.github.io/perseus)
 * to make serving apps on other platforms easier!
 *
 * # Resources
 *
 * These docs will help you as a reference, but [the book](https://arctic-hen7.github.io/perseus) should
 * be your first port of call for learning about how to use Perseus and how it works.
 *
 * - [The Book](https://arctic-hen7.github.io/perseus)
 * - [GitHub repository](https://github.com/arctic-hen7/perseus)
 * - [Crate page](https://crates.io/crates/perseus)
 * - [Gitter chat](https://gitter.im/perseus-framework/community)
 * - [Discord server channel](https://discord.com/channels/820400041332179004/883168134331256892) (for Sycamore-related stuff)
 */

mod autoserde;
mod head;
mod rx_state;
mod template;
mod test;

use darling::FromMeta;
use proc_macro::TokenStream;
use syn::ItemStruct;

/// Automatically serializes/deserializes properties for a template. Perseus handles your templates' properties as `String`s under the
/// hood for both simplicity and to avoid bundle size increases from excessive monomorphization. This macro aims to prevent the need for
/// manually serializing and deserializing everything! This takes the type of function that it's working on, which must be one of the
/// following:
///
/// - `build_state` (serializes return type)
/// - `request_state` (serializes return type)
/// - `set_headers` (deserializes parameter)
/// - `amalgamate_states` (serializes return type, you'll still need to deserializes from `States` manually)
#[proc_macro_attribute]
pub fn autoserde(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as autoserde::AutoserdeFn);
    let attr_args = syn::parse_macro_input!(args as syn::AttributeArgs);
    // Parse macro arguments with `darling`
    let args = match autoserde::AutoserdeArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    autoserde::autoserde_impl(parsed, args).into()
}

/// Labels a Sycamore component as a Perseus template, turning it into something that can be easily inserted into the `.template()`
/// function, avoiding the need for you to manually serialize/deserialize things. This should be provided the name of the Sycamore component (same as given
/// to Sycamore's `#[component()]`, but without the `<G>`).
#[proc_macro_attribute]
pub fn template(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as template::TemplateFn);
    let arg = syn::parse_macro_input!(args as syn::Ident);

    template::template_impl(parsed, arg).into()
}

/// Exactly the same as `#[template]`, but this expects your state to be reactive (use `#[make_rx]` to make it thus). This will automatically deserialize state and make it reactive,
/// allowing you to use an MVC pattern easily in Perseus. As the second argument, you'll need to provide the name of your unreactive state `struct` (this is unergonomic,
/// but the compiler isn't smart enough to infer it yet).
///
/// Additionally, this macro will add the reactive state to the global state store, and will fetch it from there, allowing template state to persists between page changes. Additionally,
/// that state can be accessed by other templates if necessary.
#[proc_macro_attribute]
pub fn template_with_rx_state(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as template::TemplateFn);
    let attr_args = syn::parse_macro_input!(args as syn::AttributeArgs);

    template::template_with_rx_state_impl(parsed, attr_args).into()
}

/// Labels a function as a Perseus head function, which is very similar to a template, but
/// for the HTML metadata in the document `<head>`.
#[proc_macro_attribute]
pub fn head(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as head::HeadFn);

    head::head_impl(parsed).into()
}

/// Marks the given function as a Perseus test. Functions marked with this attribute must have the following signature:
/// `async fn foo(client: &mut fantoccini::Client) -> Result<>`.
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

/// Processes the given `struct` to create a reactive version by wrapping each field in a `Signal`. This will generate a new `struct` with the given name and implement a `.make_rx()`
/// method on the original that allows turning an instance of the unreactive `struct` into an instance of the reactive one.
///
/// This macro automatically derives `serde::Serialize` and `serde::Deserialize` on the original `struct`, so do NOT add these yourself, or errors will occur. Note that you can still
/// use Serde helper macros (e.g. `#[serde(rename = "testField")]`) as usual. `Clone` will also be derived on both the original and the new `struct`, so do NOT try to derive it yourself.
///
/// If one of your fields is itself a `struct`, by default it will just be wrapped in a `Signal`, but you can also enable nested fine-grained reactivity by adding the
/// `#[rx::nested("field_name", FieldTypeRx)]` helper attribute to the `struct` (not the field, that isn't supported by Rust yet), where `field_name` is the name of the field you want
/// to use ensted reactivity on, and `FieldTypeRx` is the wrapper type that will be expected. This should be created by using this macro on the original `struct` type.
///
/// Note that this will be deprecated or significantly altered by Sycamore's new observables system (when it's released). For that reason, this doesn't support more advanced
/// features like leaving some fields unreactive, this is an all-or-nothing solution for now.
///
/// # Examples
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// # use perseus_macro::make_rx; // You might import this from `perseus`
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
