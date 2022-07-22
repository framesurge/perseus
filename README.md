<h1 align="center">Perseus</h1>

[![Book](https://img.shields.io/badge/Book-arctic--hen7.github.io-informational?style=for-the-badge)](https://arctic-hen7.github.io/perseus/en-US/docs)
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

## What's it like?

Here's a taste of Perseus (see [the _tiny_ example](https://github.com/arctic-hen7/perseus/tree/main/examples/comprehensive/tiny) for more):

```rust,ignore
use perseus::{Html, PerseusApp, Template};
use sycamore::view;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(|| {
        Template::new("index").template(|cx, _| {
            view! { cx,
                p { "Hello World!" }
            }
        })
    })
}
```

Check out [the book](https://arctic-hen7.github.io/perseus/en-US/docs) to learn how to turn that into your next app!

## Quick start

If you want to start working with Perseus right away, run the following commands and you'll have a basic app ready in no time! (Or, more accurately, after Cargo compiles everything...)

``` shell
cargo install perseus-cli --version 0.4.0-beta.5
perseus new my-app
cd my-app/
perseus serve -w
```

Then, hop over to <http://localhost:8080> and see a placeholder app, in all its glory! If you change some code, that'll automatically update, reloading the browser all by itself. (This rebuilding might take a while though, see [here](https://arctic-hen7.github.io/perseus/en-US/docs/next/reference/compilation-times) for how to speed things up.)

Check out our [getting started tutorial](https://arctic-hen7.github.io/perseus/en-US/docs/next/getting-started/installation) for more, or head over to out [core principles](https://arctic-hen7.github.io/perseus/en-US/docs/next/core-principles) page, which explains the basics of how Perseus works. Enjoy!

## Aim

Support every major rendering strategy and provide developers the ability to efficiently create super-fast apps with Rust and a fantastic developer experience!

## Motivation

There is a sore lack of Rust frameworks for frontend development that support more than just SPAs and client-side rendering, and so Perseus was born. We need something like NextJS for Wasm. But why stop there?

## Contributing

We appreciate all kinds of contributions, check out our [contributing guidelines](https://github.com/arctic-hen7/perseus/blob/main/CONTRIBUTING.md) for more information! Also, please be sure to follow our [code of conduct](https://github.com/arctic-hen7/perseus/blob/main/CODE_OF_CONDUCT.md).

You can also chat about Perseus on [our channel on Sycamore's Discord server](https://discord.com/invite/GNqWYWNTdp).

## License

See [`LICENSE`](https://github.com/arctic-hen7/perseus/blob/main/LICENSE).
