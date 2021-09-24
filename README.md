<h1 align="center">Perseus</h1>

[![Book](https://img.shields.io/badge/Book-arctic--hen7.github.io-informational?style=for-the-badge)](https://arctic-hen7.github.io/perseus)
[![API Docs](https://img.shields.io/docsrs/perseus?label=API%20Docs&style=for-the-badge)](https://docs.rs/perseus)
[![Crate Page](https://img.shields.io/crates/v/perseus?style=for-the-badge)](https://crates.io/crates/perseus)
[![Top Language](https://img.shields.io/github/languages/top/arctic-hen7/perseus?style=for-the-badge)]()
[![Discord Chat](https://img.shields.io/discord/820400041332179004?label=Discord&style=for-the-badge)](https://discord.gg/PgwPn7dKEk)

Perseus is a blazingly fast frontend web development framework built in Rust with support for major rendering strategies, reactivity without a virtual DOM, and extreme customizability. It wraps the lower-level capabilities of [Sycamore](https://github.com/sycamore-rs/sycamore)!

-   📕 Supports static generation (serving only static resources)
-   🗼 Supports server-side rendering (serving dynamic resources)
-   🔧 Supports revalidation after time and/or with custom logic (updating rendered pages)
-   🛠️ Supports incremental regeneration (build on demand)
-   🏭 Open build matrix (use any rendering strategy with anything else)
-   🖥️ CLI harness that lets you build apps with ease and confidence
-   🌐 Full i18n support out-of-the-box with [Fluent](https://projectfluent.org)

## Usage

Here's a taste of Perseus (see [the _tiny_ example](https://github.com/arctic-hen7/perseus/tree/main/examples/tiny) for more):

```rust
use perseus::{define_app, ErrorPages, Template};
use std::rc::Rc;
use sycamore::template;
define_app! {
    templates: [
        Template::<G>::new("index").template(Rc::new(|_| {
            template! {
                p { "Hello World!" }
            }
        }))
    ],
    error_pages: ErrorPages::new(Rc::new(|url, status, err, _| {
        template! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    }))
}
```

Check out [the book](https://arctic-hen7.github.io/perseus) to learn how to turn that into your next app!

## Aim

Support every major rendering strategy and provide developers the ability to efficiently create super-fast apps with Rust and a fantastic developer experience!

## Motivation

There is a sore lack of Rust frameworks for frontend development that support more than just SPAs and client-side rendering, and so Perseus was born. We need something like NextJS for Wasm.

## Roadmap

### Pre-stable

These tasks still need to be done before Perseus can be pushed to v1.0.0.

-   [x] Create a custom CLI as a harness for apps without ridiculous amounts of configuration needed

*   [x] Support i18n out of the box
*   [x] Implement custom router
*   [x] Allow direct modification of the document head
*   [x] Improve SEO and initial load performance
*   [ ] Support custom template hierarchies
*   [ ] Pre-built integrations
    -   [x] Actix Web
    -   [ ] AWS Lambda

### Beyond

These tasks will be done after Perseus is stable.

-   [ ] Integrations for other platforms

## Contributing

We appreciate all kinds of contributions, check out our [contributing guidelines](./CONTRIBUTING.md) for more information! Also, please be sure to follow our [code of conduct](./CODE_OF_CONDUCT.md).

You can also chat about Perseus at our [Gitter link](https://gitter.im/perseus-framework/community), or (for Sycamore-related stuff) on [our channel on Sycamore's Discord server](https://discord.com/channels/820400041332179004/883168134331256892).

## License

See [`LICENSE`](./LICENSE).

[book]: https://arctic-hen7.github.io/perseus
[crate]: https://crates.io/crates/perseus
[docs]: https://docs.rs/perseus
[contrib]: ./CONTRIBUTING.md
