# Hello World!

Let's get started with Perseus!

*To follow along here, you'll want to be familiar with Rust, which you can learn more about [here](https://rust-lang.org). You should also have it and `cargo` installed.*

To begin, create a new folder for your project, let's call it `my-perseus-app`. Now, create a `Cargo.toml` file in that folder. This tells Rust which packages you want to use in your project and some other metadata. Put the following inside:

```toml
{{#include ../../../examples/tiny/Cargo.toml.example}}
```

<details>
<summary>What are those dependencies doing?</summary>

- `perseus` -- the core module for Perseus
- [`sycamore`](https://github.com/sycamore-rs/sycamore) -- the amazing system on which Perseus is built, this allows you to write reactive web apps in Rust

Note that we've set these dependencies up so that they'll automatically update *patch versions*, which means we'll get bug fixes automatically, but we won't get any updates that will break our app!

</details>

Now, create an `index.html` file at the root of your project and put the following inside:

```html
{{#include ../../../examples/tiny/index.html}}
```

<details>
<summary>Why do I need an HTML file?</summary>

Perseus aims to be as versatile as possible, and so it allows you to include your own `index.html` file, in which you can import things like fonts, analytics, etc.

This file MUST contain at least the following:

- `<div id="root"></div>`, which is where your app will be rendered, this must be a `<div>` with no other attributes except the `id`, and that spacing (that way parsing is lightweight and fast)
- A `<head>`, which is where HTML metadata goes (even if you don't have any metadata, Perseus still needs it)

Note also that we don't have to import anything to make Perseus run here, the server will do that automatically for us!

</details>

Now, create a new directory called `src` and add a new file inside called `lib.rs`. Put the following inside:

```rust,no_run,no_playground
{{#include ../../../examples/tiny/src/lib.rs}}
```

<details>
<summary>How does that work?</summary>

First, we import some things that'll be useful:

- `perseus::{define_app, ErrorPages, Template}` -- the -`define_app!` macro, which tells Perseus how your app works; the `ErrorPages` `struct`, which lets you tell Perseus how to handle errors (like *404 Not Found* if the user goes to a nonexistent page); and the `Template` `struct`, which is how Perseus manages pages in your app
- `std::rc::Rc` -- a [reference-counted smart pointer](https://doc.rust-lang.org/std/rc/struct.Rc.html) (you don't *have* to understand these to use Perseus, but reading that link would be helpful)
- `sycamore::template` -- Sycamore's [`template!` macro], which lets you write HTML-like code in Rust

Then, we use the `define_app!` macro to declare the different aspects of the app, starting with the *templates*. We only have one template, which we've called `index` (a special name that makes it render at the root of your app), and then we define how that should look, creating a paragraph (`p`) containing the text `Hello World!`. Perseus does all kinds of clever stuff with this under the hood, and we put it in an `Rc` to enable that.

Finally, we tell Perseus what to do if something in your app fails, like if the user goes to a page that doesn't exist. This requires creating a new instance of `ErrorPages`, which is a `struct` that lets you define a separate error page for every [HTTP status code](https://httpstatuses.com), as well as a fallback. Here, we've just defined the fallback. That page is given the URL that caused the error, the HTTP status code, and the actual error message, all of which we display with a Sycamore `template!`, with seamless interpolation.

</details>

Now install the Perseus CLI with `cargo install perseus-cli` (you'll need `wasm-pack` to let Perseus build your app, use `cargo install wasm-pack` to install it) to make your life way easier, and deploy your app to <http://localhost:8080> by running `perseus serve` inside the root of your project! This will take a while the first time, because it's got to fetch all your dependencies and build your app.

<details>
<summary>Why do I need a CLI?</summary>

Perseus is a *very* complex system, and, if you had to write all that complexity yourself, that *Hello World!* example would be more like 1700 lines of code than 17! The CLI lets you abstract away all that complexity into a directory that you might have noticed appear called `.perseus/`. If you take a look inside, you'll actually find two crates (Rust packages): one for your app, and another for the server that serves your app. These are what actually run your app, and they import the code you've written. The `define_app!` macro defines a series of functions and constants at compile-time that make this possible.

When you run `perseus serve`, the `.perseus/` directory is created and added to your `.gitignore`, and then three stages occur in parallel (they're shown in your terminal):

- *🔨 Generating your app* -- here, your app is built to a series of static files in `.perseus/dist/static`, which makes your app lightning-fast (your app's pages are ready before it's even been deployed, which is called *static site generation*, or SSG)
- *🏗️ Building your app to Wasm* -- here, your app is built to [WebAssembly](), which is what lets a low-level programming language like Rust run in the browser
- *📡 Building server* -- here, Perseus builds its internal server based on your code, and prepares to serve your app

The first time you run this command, it can take quite a while to get everything ready, but after that it'll be really fast. And, if you haven't changed any code (*at all*) since you last ran it, you can run `perseus serve --no-build` to run the server basically instantaneously.

</details>

Once that's done, hop over to <http://localhost:8080> in any modern browser (not Internet Explorer...), and you should see *Hello World!* printed on the screen! If you try going to <http://localhost:8080/about> or any other page, you should see a message that tells you the page wasn't found.

Congratulations! You've just created your first ever Perseus app! You can see the source code for this section [here](https://github.com/arctic-hen7/perseus/tree/main/examples/tiny).

## Moving Forward

The next section creates a slightly more realistic app with more than just one file, which will show you how a Perseus app is usually structured.

After that, you'll learn how different features of Perseus work, like *incremental generation* (which lets you build pages on-demand at runtime)!

### Alternatives

If you've gone through this and you aren't that chuffed with Perseus, here are some similar projects in Rust:

- [Sycamore](https://github.com/sycamore-rs/sycamore) (without Perseus) -- *A reactive library for creating web apps in Rust and WebAssembly.*
- [Yew](https://github.com/yewstack/yew) -- *Rust/Wasm framework for building client web apps.*
- [Seed](https://github.com/seed-rs/seed) -- *A Rust framework for creating web apps.*
- [Percy](https://github.com/chinedufn/percy) -- *Build frontend browser apps with Rust + WebAssembly. Supports server side rendering.*
- [MoonZoon](https://github.com/MoonZoon/MoonZoon) -- *Rust Fullstack Framework.*
