# Hello World!

Let's get started with Perseus!

_To follow along here, you'll want to be familiar with Rust, which you can learn more about [here](https://rust-lang.org). You should also have it and `cargo` installed._

To begin, create a new folder for your project, let's call it `my-perseus-app`. Now, create a `Cargo.toml` file in that folder. This tells Rust which packages you want to use in your project and some other metadata. Put the following inside:

```toml
{{#include ../../../examples/comprehensive/tiny/Cargo.toml.example}}
```

<details>
<summary>What are those dependencies doing?</summary>

-   `perseus` -- the core module for Perseus
-   [`sycamore`](https://github.com/sycamore-rs/sycamore) -- the amazing system on which Perseus is built, this allows you to write reactive web apps in Rust

Note that we've set these dependencies up so that they'll automatically update _patch versions_, which means we'll get bug fixes automatically, but we won't get any updates that will break our app!

</details>

Now, create an `index.html` file at the root of your project and put the following inside:

```html
{{#include ../../../examples/comprehensive/tiny/index.html}}
```

<details>
<summary>Don't I need an `index.html` file?</summary>

With versions of Perseus before v0.3.4, an `index.html` file was required for Perseus to know how to display in your users' browsers, however, this is no longer required, as Perseus now has a default *index view* built in, with the option to provide your own through either `index.html` or Sycamore code!

For the requirements of any index views you create, see below.

</details>

Now, create a new directory called `src` and add a new file inside called `lib.rs`. Put the following inside:

```rust
{{#include ../../../examples/comprehensive/tiny/src/lib.rs}}
```

<details>
<summary>How does that work?</summary>

First, we import some things that'll be useful:

-   `perseus::{Html, PerseusApp, Template}` -- the `Html` trait, which lets your code be generic so that it can be rendered on either the server or in a browser (you'll see this throughout Sycamore code written for Perseus); the `PerseusApp` `struct`, which is how you represent a Perseus app; the `Template` `struct`, which represents a *template* in Perseus (which can create pages, as you'll soon learn -- this is the fundamental unit of Perseus)
-   `sycamore::view` -- Sycamore's `view!` macro, which lets you write HTML-like code in Rust

Perseus used to use a macro called `define_app!` to define your app, though this has since been deprecated and replaced with a more modern builder `struct`, which has methods that you can use to add extra features to your app (like internationalization). This is `PerseusApp`, and here, we're just adding one template with the `.template()` call (which you'll run each time you want to add a new template to your app). Here, we create a very simple template called `index`, a special template name that will bind this template to the root of your app, this will be the landing page. We then define the view code for this template with the `.template()` method on the `Template` `struct`, to which we provide a simple closure that returns a Sycamore `view!`, which just renders an HTML paragraph element (`<p>Hello World!</p>` in usual HTML markup). Usually, we'd provide a fully-fledged function here that can do many more things (like access global state stores), but for now we'll keep things nice and simple.

In most apps, the main things you'll define on `PerseusApp` are `Template`s, though, when you move to production, you'll also want to define `ErrorPages`, which tell Perseus what to do if your app reaches a nonexistent page (a 404 not found error) or similar. For rapid development though, Perseus provides a series of prebuilt error pages (but if you try to use these implicitly in production, you'll get an error message).

Also notice that we define this `PerseusApp` in a function called `main`, but you can call this anything you like, as long as you put `#[perseus::main]` before it, which turns it into something Perseus can expect (specifically, a special function named `__perseus_entrypoint`).

</details>

Now install the Perseus CLI with `cargo install perseus-cli` (you'll need `wasm-pack` to let Perseus build your app, use `cargo install wasm-pack` to install it) to make your life way easier, and deploy your app to <http://localhost:8080> by running `perseus serve` inside the root of your project! This will take a while the first time, because it's got to fetch all your dependencies and build your app.

<details>
<summary>Why do I need a CLI?</summary>

Perseus is a _very_ complex system, and, if you had to write all that complexity yourself, that _Hello World!_ example would be more like 1200 lines of code than 12! The CLI lets you abstract away all that complexity into a directory that you might have noticed appear called `.perseus/`. If you take a look inside, you'll actually find three crates (Rust packages): one for your app, another for the server that serves your app, and another for the builder that builds your app. These are what actually run your app, and they import the code you've written. They interface with the `PerseusApp` you define to make all this work.

When you run `perseus serve`, the `.perseus/` directory is created and added to your `.gitignore`, and then three stages occur in parallel (they're shown in your terminal):

-   _🔨 Generating your app_ -- here, your app is built to a series of static files in `.perseus/dist/static`, which makes your app lightning-fast (your app's pages are ready before it's even been deployed, which is called _static site generation_, or SSG)
-   _🏗️ Building your app to Wasm_ -- here, your app is built to [WebAssembly](https://webassembly.org), which is what lets a low-level programming language like Rust run in the browser
-   _📡 Building server_ -- here, Perseus builds its internal server based on your code, and prepares to serve your app (note that an app this simple can actually use [static exporting](:reference/exporting), but we'll deal with that later)

The first time you run this command, it can take quite a while to get everything ready, but after that it'll be really fast. And, if you haven't changed any code (_at all_) since you last ran it, you can run `perseus serve --no-build` to run the server basically instantaneously.

</details>

Once that's done, hop over to <http://localhost:8080> in any modern browser (not Internet Explorer...), and you should see _Hello World!_ printed on the screen! If you try going to <http://localhost:8080/about> or any other page, you should see a message that tells you the page wasn't found.

Congratulations! You've just created your first ever Perseus app! You can see the source code for this section [here](https://github.com/arctic-hen7/perseus/tree/main/examples/comprehensive/tiny).

## Moving Forward

The next section creates a slightly more realistic app with more than just one file, which will show you how a Perseus app is usually structured.

After that, you'll learn how different features of Perseus work, like _incremental generation_ (which lets you build pages on-demand at runtime)!

### Alternatives

If you've gone through this and you aren't that chuffed with Perseus, here are some similar projects in Rust:

-   [Sycamore](https://github.com/sycamore-rs/sycamore) (without Perseus) -- _A reactive library for creating web apps in Rust and WebAssembly._
-   [Yew](https://github.com/yewstack/yew) -- _Rust/Wasm framework for building client web apps._
-   [Seed](https://github.com/seed-rs/seed) -- _A Rust framework for creating web apps._
-   [Percy](https://github.com/chinedufn/percy) -- _Build frontend browser apps with Rust + WebAssembly. Supports server side rendering._
-   [MoonZoon](https://github.com/MoonZoon/MoonZoon) -- _Rust Fullstack Framework._
