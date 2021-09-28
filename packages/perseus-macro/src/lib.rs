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

mod test;

use darling::FromMeta;
use proc_macro::TokenStream;

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
