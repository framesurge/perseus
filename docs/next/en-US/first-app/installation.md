# Installation

Before you get to coding your first Perseus app, you'll need to install the Perseus command-line interface (CLI) first, which you'll use to manage your app. The reason for this is that Perseus is a *framework*, not a library: you don't import Perseus into your code and use it, Perseus imports your code into itself. In fact, in the old days, you used to write a library that another crate would literally import!

To install the Perseus CLI, first make sure you have Rust installed (preferably with [`rustup`](https://rustup.rs)), and then run this command:

```sh
cargo install perseus-cli --version 0.4.0-beta.17
```

Once that's done, you can go ahead and create your first app! Although this would usually be done with the `perseus new` command, which spins up a scaffold for you, in this tutorial we'll do things manually so we can go through each line of code step by step. First, create a new Rust project:

```sh
cargo new my-app
cd my-app
```

This will create a new directory called `my-app/` that's equipped for a binary project (i.e. something you can run, rather than a library, which other code uses). First of all, create `.cargo/config.toml` in the root of your project, with the following contents:

```toml
[build]
rustflags = [ "--cfg", "engine" ]
```

This will make sure your IDE builds your app correctly. Without this, you'll have red squiggly lines all over the place, because Perseus needs to be explicitly told if it's working on the engine-side (e.g. a server) or the browser-side, which are very different environments! Also, setting things up explicitly like this lets you change `engine` to `client` in that file when you want your IDE to help you out with working on browser-only code. 

Next, put the following in your app's `Cargo.toml`:

```toml
{{#include ../../../examples/core/basic/Cargo.toml.example}}
```

The main things to pay attention to here are the dependencies, which are laid out differently from most Rust apps. Perseus is built in two parts: the *engine-side*, which is responsible for prerendering your pages, serving content, exporting your app, etc.; and the *client-side*, which runs inside a user's browser to make Perseus interactive, handling routing, interactivity, etc. The engine-side of your app will build to whatever target you compile it for, like `x86_64-unknown-linux-gnu`, which you would have on an OS like Ubuntu. This means Rust will translate your code into machine code that computers with that kind of processor and OS can understand (if you were running on an M1 Mac, the target would be quite different). The browser has its own sepaarate target, which ensures that you don't have to compile your code for every possible device that a user might view it on --- the browser takes care of all that, and runs Wasm, which is its own special language that Rust can translate itself into.

That all means that there are some features that don't belong in the browser (like building your app), and others that don't belong in the engine (like managing routing), so Perseus *target-gates* these, using Rust's `#[cfg(..)]` macro to make sure that certain things are only compiled at the right time. This reduces compilation times, and also slims down the bundles for both the engine and the browser (because they contain no unnecessary code). Sometimes, you'll want to do this in your own code as well, like if you have some function that should only run on the browser-side. Remember how we set up that `rustflags` key in `.cargo/config.toml`? Well, that's so you can use it just like this! If you want code to only be compiled for the browser, you put `#[cfg(client)]` on top of it, and you can use `#[cfg(engine)]` to do the same for the engine. You'll usually see this in Rust code, but your `Cargo.toml` can use it too for declaring dependencies that will only be used on one particular target. Here, we're making sure to bring in `perseus` everywhere, but `perseus-warp` (our server integration) should only be used on the engine-side. When you bring in a new dependency, think about whether it has to be available on the browser-side, because it often doesn't. For example, you could bring in the `regex` crate to automatically highlight any technical terms in a documentation site, but you can actually do that solely on the engine-side if you handle all that in the state generation process (which we'll get to). This avoids bringing the `regex` crate into the browser, which keeps your `.wasm` bundle nice and slim. A smaller Wasm bundle means it can be transferred over the network more quickly, which means faster page loads.

As for the actual dependencies themselves, here's what each one is for:

- `perseus`: this framework
- `sycamore`: the library that Perseus builds on, which you'll use to write views
- `serde` and `serde_json`: serialization/deserialization libraries you'll need to help Perseus transmit your pages over the internet
- `tokio` (engine-only): an `async` runtime used by Perseus, which your code is responsible for instantiating (but 99% of the time, you'll do that with a helper macro); we select certain features to improve performance and reduce compilation times
- `perseus-axum`: a server integration that will serve your pages to users

A basic Perseus app won't have any client-side dependencies, and you can omit that empty section if you like, but it's included here for completeness.
