# Installation and Setup

Perseus aims to be as easy to use as possible with the default settings, but you will need to install a few things first.

## Rust and Cargo

Make sure you've got Rust and Cargo. If you haven't yet, install them from [here](https://doc.rust-lang.org/stable/book/ch01-01-installation.html), and you might want to check out the [Rust book](https://doc.rust-lang.org/book) if you're unfamiliar with the language.

After you've done this, make sure everything works by running `cargo` in a terminal. You should get a help page!

## Build Tools

Perseus is built on top of Wasm (WebAssembly), which basically lets you use programming languages other than JavaScript to build websites/webapps. That tech is _really_ complicated, and you'll need two particular build tools to make your code work.

The first one is [`wasm-pack`](), which helps to compile your Rust code to WebAssembly (sort of like how you'd compile code normally, but specially for the browser). You can install it with this command:

```
cargo install wasm-pack
```

Now, you should be able to type `wasm-pack` in your terminal to get another help page!

The next tool is one you might be familiar with if you're coming from the JavaScript world: [Rollup](https://rollupjs.org). Rollup is a bundling tool for JavaScript, and it works really nicely with Wasm. If you loathe JavaScript with a passion, don't worry, the only JavaScript in Perseus just invokes your (infinitely superior) Rust code! You can install Rollup with the following command:

```
npm i -g rollup
```

or

```
yarn global add rollup
```

Both of those assume you have either `npm` or `yarn` installed. If not, you can check out other installation options [here](https://rollupjs.org/guide/en/#installation).

## Perseus CLI

Perseus is easiest to use with the official CLI, which will let you effortlessly develop apps and serve them locally. It's not quite designed for production serving yet, but development on that is actively underway!

You can install the CLI with the following command:

```
cargo install perseus-cli
```

And it should be available as `perseus` in your terminal! Type `perseus -v` to check that everything works (you should see a version number).

Note that the Perseus CLI will not work unless you've installed Cargo, `wasm-pack` and Rollup. If you've installed them at different paths, you can set the `PERSEUS_CARGO_PATH`/`PERSEUS_WASM_PACK_PATH`/`PERSEUS_ROLLUP_PATH` environment variables to define those. The default paths are `cargo`, `wasm-pack`, and `rollup` respectively.

## Setting Up a New Project

Okay! Now that you've got all the build tools and the CLI set up, you can create a new project with this command in the directory where you want your app:

```
cargo init --lib
```

You should now have an `src/` directory and a `Cargo.toml` file, which is what we'll edit first. You need quite a few dependencies for Perseus to work, which you can set up by adding the following to your `Cargo.toml` under the `[dependencies]` line:

```toml
# Perseus itself, which we (amazingly) need for a Perseus app
perseus = "0.1"
# Sycamore, the library Perseus depends on for lower-leve reactivity primitivity
sycamore = { version = "0.5", features = ["ssr"] }
sycamore-router = "0.5"
# Serde, which lets you work with representations of data, like JSON
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Now run the following command to install and build everything for the first time (this will take a little while):

```
cargo build
```

If that all works, then congratulations, you've just created the scaffold of a Perseus app! It's time to make your app do something now!
