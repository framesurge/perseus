# Your Second App

This section will cover building a more realistic app than the _Hello World!_ section, with proper structuring and multiple templates.

If learning by reading isn't really your thing, or you'd like a reference, you can see all the code in [this repository](https://github.com/arctic-hen7/perseus/tree/main/examples/core/basic)!

## Setup

Much like the _Hello World!_ app, we'll start off by creating a new directory for the project, maybe `my-second-perseus-app` (or you could exercise imagination...). Then, we'll create a new `Cargo.toml` file and fill it with the following:

```toml
{{#include ../../../examples/core/basic/Cargo.toml.example}}
```

The only difference between this and the last `Cargo.toml` we created is two new dependencies:

-   [`serde`](https://serde.rs) -- a really useful Rust library for serializing/deserializing data
-   [`serde_json`](https://github.com/serde-rs/json) -- Serde's integration for JSON, which lets us pass around properties for more advanced pages in Perseus (you may not explicitly use this, but you'll need it as a dependency for some Perseus macros)

## `lib.rs`

As in every Perseus app, `lib.rs` is how we communicate with the CLI and tell it how our app works. Put the following content in `src/lib.rs`:

```rust
{{#include ../../../examples/core/basic/src/lib.rs}}
```

This code is quite different from your first app, so let's go through how it works.

First, we define two other modules in our code: `error_pages` (at `src/error_pages.rs`) and `templates` (at `src/templates`). Don't worry, we'll create those in a moment. The rest of the code creates a new app with two templates, both from the `templates` module. Specifically, we provide the `.template()` function with another function that produces our template, which allows us to keep each template's code in a separate file.

We're also using `.error_pages()` here to tell Perseus how to handle errors in our app (like a nonexistent page), and we'll put these in the `error_pages` module. Note that you don't have to do this in development, Perseus has a set of defaults that it can use, but you can't use those in production, so you will have to create some error pages at some stage.

## Error Handling

Before we get to the cool part of building the actual pages of the app, we should set up error pages, which we'll do in `src/error_pages.rs`:

```rust
{{#include ../../../examples/core/basic/src/error_pages.rs}}
```

The first thing to note here is the import of [`Html`](https://docs.rs/sycamore/0.7/sycamore/generic_node/trait.Html.html), which we define as a type parameter on the `get_error_pages` function. This makes sure that we can compile these views on the client or the server as long as they're targeting HTML (Sycamore can also target other templating formats for completely different systems, like MacOS desktop apps).

In this function, we also define a different error page for a 404 error, which will occur when a user tries to go to a page that doesn't exist. The fallback page (which we initialize `ErrorPages` with) is the same as last time, and will be called for any errors other than a _404 Not Found_. Note that the error pages we define here are extremely similar to Perseus' defaults, and, in a real app, you'd probably create something much more fancy!

## `index.rs`

It's time to create the first page for this app! But first, we need to make sure that import in `src/lib.rs` of `mod templates;` works, which requires us to create a new file `src/templates/mod.rs`, which declares `src/templates` as a module in your crate with its own code (this is how folders work in rust projects). Add the following to that file:

```rust
{{#include ../../../examples/core/basic/src/templates/mod.rs}}
```

It's common practice to have a file (or even a folder) for each _template_, which is slightly different to a page (explained in more detail later), and this app has two pages: a landing page (index) and an about page.

Let's begin with the landing page. Create a new file `src/templates/index.rs` and put the following inside:

```rust
{{#include ../../../examples/core/basic/src/templates/index.rs}}
```

This code is _much_ more complex than the _Hello World!_ example, so let's go through it carefully.

First, we import a whole ton of stuff:

-   `perseus`
    -   `RenderFnResultWithCause` -- see below for an explanation of this
    -   `Template` -- as before
    -   `Html` -- as before (this is from Sycamore, but is re-exported by Perseus for convenience)
    -   `http::header::{HeaderMap, HeaderName}` -- some types for adding HTTP headers to our page
    -   `SsrNode` -- Sycamore's representation of a node that will only be rendered on the server (this comes from the `sycamore` crate, but Perseus re-exports it for convenience)
-   `serde`
    -   `Serialize` -- a trait for `struct`s that can be turned into a string (like JSON)
    -   `Deserialize` -- a trait for `struct`s that can be *de*serialized from a string (like JSON)
-   `sycamore`
    -   `component` -- a macro that turns a function into a Sycamore component
    -   `view` -- the `view!` macro, same as before
    -   `View` -- the output of the `view!` macro

Then we define a number of different functions and a `struct`, each of which gets a section now.

### `IndexPageState`

This `struct` represents the *state* of the index page. As mentioned in the explanation of Perseus' [core principles](:core-principles), Perseus is fundamentally a system for turning a state and a view into a user interface. In this case, our index page will display a greeting generated at build time. That means that our view code will be generic over the greeting that we generate (which you can see in our `view! {...}` code under `index_page()`). Perseus lets us generate state in [several ways](:reference/strategies), and in this case we'll use the simplest: [build state](:reference/strategies/build-state), which just runs a custom function when we build our app that generates an instance of our state, which is represented by this `struct`.

Note that we also use the `#[perseus::make_rx(IndexPageStateRx)]` macro here to make our state *reactive*. What this macro does is takes in our `struct` and produces a new version of it called `IndexPageStateRx` that has the exact same fields, but with each one wrapped in a Sycamore `Signal`, which makes each field reactive. This means that our page can mutate its state in one place and every other place that uses that state will automatically update through Sycamore's reactivity system! You can read more about the ideas behind this [here](:reference/state/rx).

### `index_page()`

This is the actual component that will render a user interface for your page. Perseus lets you provide a *template function* like this as a simple Rust function that takes in your page's state and produces a Sycamore `View<G>` (again, `G` is ambient here because of the proc macro). However, there's *a lot* of work that goes on behind the scenes to make your state reactive, register it with Perseus, manage [global state](:reference/state/global), and set up a Sycamore component that's usable by the rest of the Perseus code. This is all done with one of two attribute macros: `#[perseus::template(...)]` or `#[perseus::template_rx]`. In previous versions of Perseus, you'd use the former, which would give you an unreactive instance of your state (in our case, `IndexPageState`). However, since v0.3.4, it's recommended to use the latter, which gives you a reactive version (in our case, `IndexPageStateRx`) and can manage more advanced features of Perseus' [reactive state platform](:reference/state/rx), so that's what we use here. Also, `template_rx` manages things like Sycamore components internally for you, minimising the amount of code you actually have to write.

Note that `index_page()` takes `IndexPageStateRx` as an argument, which it can then access in the `view!`. This is Sycamore's interpolation system, which you can read about [here](https://sycamore-rs.netlify.app/docs/basics/template), but all you need to know is that it's basically seamless and works exactly as you'd expect (remember though that, because we're using the `template_rx` macro, we have to use a reactive state `struct`, which we generate with the `make_rx` macro, so all our fields are wrapped in `Signal`s, which is why we use `.get()`).

The only other thing we do here is define an `<a>` (an HTML link) to `/about`. This link, and any others you define, will automatically be detected by Sycamore's systems, which will pass them to Perseus' routing logic, which means your users **never leave the page**. In this way, Perseus only pulls in the content that needs to change, and gives your users the feeling of a lightning-fast and weightless app.

_Note: external links will automatically be excluded from this, and you can exclude manually by adding `rel="external"` if you need._

### `head()`

This function is very similar to `index_page()`, except that it isn't a fully fledged Sycamore component, it just returns a `view! {}` instead, and it only takes an unreactive version of your state. What this is used for is to define the content of the `<head>`, which is metadata for your website, like its `<title>`. As you can see, this is given the properties that `index_page()` takes, but we aren't using them for anything in this example. The `#[perseus::head]` macro tells Perseus to do some boilerplate work behind the scenes that's very similar to that done with `index_page`, but specialized for the `<head>`.

What's really important to note about this function is that it only renders to an `SsrNode`, which means you cannot use reactivity in here! Whatever is rendered the first time will be turned into a `String` and then statically interpolated into the document's `<head>`. That also means that this only runs on the server (if you want to change it on the client, you'll need to do that manually).

The difference between metadata defined here and metadata defined in your `index.html` file is that the latter will apply to every page, while this will only apply to the template. So, this is more useful for things like titles, while you might use `index.html` to import stylesheets or analytics.

If you inspect the source code of the HTML in your browser, you'll find a big comment in the `<head>` that says `<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->`, that separates the stuff that should remain the same on every page from the stuff that should update for each page.

### `get_template()`

This function is what we call in `lib.rs`, and it combines everything else in this file to produce an actual Perseus `Template` to be used. Note the name of the template as `index`, which Perseus interprets as special, which causes this template to be rendered at `/` (the landing page).

Perseus' templating system is extremely versatile, and here we're using it to define our page itself through `.template()`, and to define a function that will modify the document `<head>` (which allows us to add a title) with `.head()`. Notably, we also use the _build state_ rendering strategy, which tells Perseus to call the `get_build_state()` function when your app builds to get some state (an instance of `IndexPageState` to be precise). More on that in a moment.

#### `.template()`

This function is what Perseus will call when it wants to render your template (which it does more often than you might think). If you've used the `#[perseus::template_rx]` or `#[perseus::template(...)]` macro on `index_page()`, you can provide `index_page` directly here, but it can be useful to understand what that macro is doing.

Behind the scenes, that macro transforms your `index_page()` function to take properties as an `Option<String>` instead of as `IndexPageState`, because Perseus actually passes your properties around internally as `String`s. At first, this might seem weird, but it avoids a few common problems that would increase your final Wasm binary size and make your website take a very long time to load. Interestingly, it's actually also more performant to use `String`s everywhere, because we need to perform that conversion anyway when we send your properties to a user's browser. In addition, the `#[perseus::template_rx]` macro will manage setting up and interacting with the more complex features of Perseus' [reactive state platform](:reference/state/rx).

If that all went over your head, don't worry, that's just what Perseus does behind the scenes, and what you used to have to do by hand! The `#[perseus::template(...)]`/`#[perseus::template_rx]` macros do all that for you.

#### `.head()`

This is just the equivalent of `.template()` for the `head()` function, and it does basically the exact same thing. The only particular thing of note here is that the properties this expects are again as an `Option<String>`, and those are deserialized automatically by the `#[perseus::head]` macro that we used on `head()` earlier.

### `get_build_props()`

This function is part of Perseus' secret sauce (actually _open_ sauce), and it will be called when the CLI builds your app to create properties that the template will take (it expects a string, hence the serialization). Here, we just hard-code a greeting in to be used, but the real power of this comes when you start using the fact that this function is `async`. You might query a database to get a list of blog posts, or pull in a Markdown documentation page and parse it, the possibilities are endless!

This function returns a rather special type, `RenderFnResultWithCause<IndexPageState>`, which declares that your function will return `IndexPageState` if it succeeds, and a special error if it fails. That error can be anything you want (it's a `Box<dyn std::error::Error + Send + Sync>` internally), but it will also have a blame assigned to it that records whether it was the server or the client that caused the error, which will impact the final HTTP status code (e.g. 404, 500, etc.). You can use the `blame_err!` macro to create these errors easily, but any time you use `?` in functions that return this type will simply use the default of blaming the server and returning an HTTP status code of _500 Internal Server Error_.

It may seem a little pointless to blame the client in the build process, but the reason this can happen is because, in more advanced uses of Perseus (particularly [incremental generation](:reference/strategies/incremental)), this function could be called as a result of a client's request with parameters that it provides, which could be invalid. Essentially, know that it's a thing that's important in more complex use-cases of Perseus.

That `#[perseus::autoserde(build_state)]` is also something you'll see quite a lot of (but not in older versions of Perseus). It's a convenience macro that automatically serializes the return of your function to a `String` for Perseus to use internally, which is basically just the opposite of the annotation we used earlier on `index_page()`. You don't technically need this, but it eliminates some boilerplate code that you don't need to bother writing yourself.

## `about.rs`

Okay! We're past the hump, and now it's time to define the (much simpler) `/about` page. Create `src/templates/about.rs` and put the following inside:

```rust
{{#include ../../../examples/core/basic/src/templates/about.rs}}
```

This is basically exactly the same as `index.rs`, except we don't have any state to deal with, and we don't need to generate anything special at build time (but Perseus will still render this page to static HTML at build time, ready to be served to your users).

## Running It

`perseus export -sw`

That's all. Every time you build a Perseus app, that's all you need to do.

*Note: because this app is very simple and doesn't use any server-requiring features, we can use [static exporting](:reference/exporting). For some more complex apps, you'll need to use `perseus serve -w` to spin up a full server.*

Once this is finished, your app will be live at <http://localhost:8080>! Note that if you don't like that, you can change the host/port with the `PERSEUS_HOST`/`PERSEUS_PORT` environment variables (e.g. you'd want to set the host to `0.0.0.0` if you want other people on your network to be able to access your site).

Hop over to <http://localhost:8080> in any modern browser and you should see your greeting `Hello World!` above a link to the about page! If you click that link, you'll be taken to a page that just says `About.`, but notice how your browser seemingly never navigates to a new page (the tab doesn't show a loading icon)? That's Perseus' _app shell_ in action, which intercepts navigation to other pages and makes it occur seamlessly, only fetching the bare minimum to make the new page load. The same behavior will occur if you use your browser's forward/backward buttons.

You can also try changing some of the code for your app (like the greeting generated), and you'll see that your app will automatically rebuild. When it's done, your browser will reload the new version of your app (even keeping as much old state as it can, meaning you can keep working without losing your place, see [here](:reference/state/hsr) for the details)! under the hood, this process is similar to the *hot module reloading* that most JavaScript frameworks perform, but it's actually even more advanced and resilient.

<details>
<summary>Why a 'modern browser'?</summary>

### Browser Compatibility

Perseus is compatible with any browser that supports Wasm, which is most modern browsers like Firefox and Chrome. However, legacy browsers like Internet Explorer will not work with any Perseus app, unless you _polyfill_ support for WebAssembly.

*Note: technically, it's possible to 'compile' Wasm into JavaScript, and we're looking into possibly supporting this inside Perseus for sites that need to target very old browsers. At the moment though, this is not supported through Perseus.*

</details>

## Moving Forward

Congratulations! You're now well on your way to building highly performant web apps in Rust! The remaining sections of this book are more reference-style, and won't guide you through building an app, but they'll focus instead on specific features of Perseus that can be used to make extremely powerful systems.

So go forth, and build!
