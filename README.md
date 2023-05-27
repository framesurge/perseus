<h1 align="center">Perseus</h1>

[![Book](https://img.shields.io/badge/Book-framesurge.sh-informational?style=for-the-badge)](https://framesurge.sh/perseus/en-US/docs)
[![API Docs](https://img.shields.io/docsrs/perseus?label=API%20Docs&style=for-the-badge)](https://docs.rs/perseus)
[![Crate Page](https://img.shields.io/crates/v/perseus?style=for-the-badge)](https://crates.io/crates/perseus)
[![Top Language](https://img.shields.io/github/languages/top/framesurge/perseus?style=for-the-badge)]()
[![Discord Chat](https://img.shields.io/discord/820400041332179004?label=Discord&style=for-the-badge)](https://discord.gg/PgwPn7dKEk)

> **WARNING:** This branch is highly experimental, and it's advised that Rust programmers work with the `main` branch or published v0.4.x versions of Perseus at the moment!

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

Here's a taste of Perseus (see [the _tiny_ example](https://github.com/framesurge/perseus/tree/main/examples/comprehensive/tiny) for more):

```rust,ignore
use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(
            Template::build("index")
                .view(|cx| {
                    view! { cx,
                        p { "Hello World!" }
                    }
                })
                .build()
        )
}
```

Check out [the book](https://framesurge.sh/perseus/en-US/docs) to learn how to turn that into your next app!

## Quick start

If you want to start working with Perseus right away, run the following commands and you'll have a basic app ready in no time! (Or, more accurately, after Cargo compiles everything...)

``` shell
cargo install perseus-cli
perseus new my-app
cd my-app/
perseus serve -w
```

Then, hop over to <http://localhost:8080> and see a placeholder app, in all its glory! If you change some code, that'll automatically update, reloading the browser all by itself. (This rebuilding might take a while though, see [here](https://framesurge.sh/perseus/en-US/docs/next/reference/compilation-times) for how to speed things up.)

## Aim

Support every major rendering strategy and provide developers the ability to efficiently create super-fast apps with Rust and a fantastic developer experience!

## Motivation

There is a sore lack of Rust frameworks for frontend development that support more than just SPAs and client-side rendering, and so Perseus was born. We need something like NextJS for Wasm. But why stop there?

## Contributing

We appreciate all kinds of contributions, check out our [contributing guidelines](https://github.com/framesurge/perseus/blob/main/CONTRIBUTING.md) for more information! Also, please be sure to follow our [code of conduct](https://github.com/framesurge/perseus/blob/main/CODE_OF_CONDUCT.md).

You can also chat about Perseus on [our channel on Sycamore's Discord server](https://discord.com/invite/GNqWYWNTdp).

Perseus wouldn't be posible without the hard work of all these wonderful people!

<a href="https://github.com/framesurge/perseus/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=framesurge/perseus" />
</a>

## License

See [`LICENSE`](https://github.com/framesurge/perseus/blob/main/LICENSE).
