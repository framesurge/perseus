<h1 align="center">Perseus</h1>

[![Book](https://img.shields.io/badge/Book-arctic--hen7.github.io-informational?style=for-the-badge)](https://arctic-hen7.github.io/perseus)
[![API Docs](https://img.shields.io/docsrs/perseus?label=API%20Docs&style=for-the-badge)](https://docs.rs/perseus)
[![Crate Page](https://img.shields.io/crates/v/perseus?style=for-the-badge)](https://crates.io/crates/perseus)
[![Top Language](https://img.shields.io/github/languages/top/arctic-hen7/perseus?style=for-the-badge)]()
[![Discord Chat](https://img.shields.io/discord/820400041332179004?label=Discord&style=for-the-badge)](https://discord.gg/PgwPn7dKEk)

Perseus is a blazingly fast frontend web development framework built in Rust with support for generating page state at build-time, request-time, incrementally, or whatever you'd like! It supports reactivity using [Sycamore](https://github.com/sycamore-rs/sycamore), and builds on it to provide a fully-fledged framework for developing modern apps.

-   📕 Supports static generation (serving only static resources)
-   🗼 Supports server-side rendering (serving dynamic resources)
-   🔧 Supports revalidation after time and/or with custom logic (updating rendered pages)
-   🛠️ Supports incremental regeneration (build on demand)
-   🏭 Open build matrix (use any rendering strategy with anything else)
-   🖥️ CLI harness that lets you build apps with ease and confidence
-   🌐 Full i18n support out-of-the-box with [Fluent](https://projectfluent.org)
-   🏎 Lighthouse scores of 100 on desktop and over 95 on mobile
-   ⚡ Support for *hot state reloading* (reload your entire app's state after you make any code changes in development, Perseus is the only framework in the world that can do this, to our knowledge)

## Usage

Here's a taste of Perseus (see [the _tiny_ example](https://github.com/arctic-hen7/perseus/tree/main/examples/comprehensive/tiny) for more):

```rust
use perseus::{define_app, ErrorPages, Template};
use sycamore::view;
define_app! {
    templates: [
        Template::<G>::new("index").template(|_| view! {
            p { "Hello World!" }
        })
    ],
    error_pages: ErrorPages::new(|url, status, err, _| view! {
        p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
    })
}
```

Check out [the book](https://arctic-hen7.github.io/perseus/en-US) to learn how to turn that into your next app!

## Aim

Support every major rendering strategy and provide developers the ability to efficiently create super-fast apps with Rust and a fantastic developer experience!

## Motivation

There is a sore lack of Rust frameworks for frontend development that support more than just SPAs and client-side rendering, and so Perseus was born. We need something like NextJS for Wasm (but why stop there?).

## Contributing

We appreciate all kinds of contributions, check out our [contributing guidelines](https://github.com/arctic-hen7/perseus/blob/main/CONTRIBUTING.md) for more information! Also, please be sure to follow our [code of conduct](https://github.com/arctic-hen7/perseus/blob/main/CODE_OF_CONDUCT.md).

You can also chat about Perseus on [our channel on Sycamore's Discord server](https://discord.com/invite/GNqWYWNTdp).

## License

See [`LICENSE`](https://github.com/arctic-hen7/perseus/blob/main/LICENSE).
