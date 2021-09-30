# Your Second App

This section will cover building a more realistic app than the _Hello World!_ section, with proper structuring and multiple templates.

If learning by reading isn't really your thing, or you'd like a reference, you can see all the code in [this repository](https://github.com/arctic-hen7/perseus/tree/main/examples/basic)!

## Setup

Much like the _Hello World!_ app, we'll start off by creating a new directory for the project, maybe `my-second-perseus-app` (or you could exercise imagination...). Then, we'll create a new `Cargo.toml` file and fill it with the following:

```toml
{{#include ../../../examples/basic/Cargo.toml.example}}
```

The only difference between this and the last `Cargo.toml` we created is two new dependencies:

-   [`serde`](https://serde.rs) -- a really useful Rust library for serializing/deserializing data
-   [`serde_json`](https://github.com/serde-rs/json) -- Serde's integration for JSON, which lets us pass around properties for more advanced pages in Perseus

The next thing to do is to create `index.html`, which is pretty much the same as last time:

```html
{{#include ../../../examples/basic/index.html}}
```

The only notable difference here is the absence of a `<title>`, which is because we'll be creating it inside Perseus! Any Perseus template can modify the `<head>` of the document, but anything you put into `index.html` will persist across all pages. We don't want to have conflicting titles, so we leave that property out of `index.html`.

## `lib.rs`

As in every Perseus app, `lib.rs` is how we communicate with the CLI and tell it how our app works. Put the following content in `src/lib.rs`:

```rust,no_playground,no_run
{{#include ../../../examples/basic/src/lib.rs}}
```

This code is quite different from your first app, so let's go through how it works.

First, we define two other modules in our code: `error_pages` (at `src/error_pages.rs`) and `templates` (at `src/templates`). Don't worry, we'll create those in a moment. The rest of the code creates a new app with two templates, which are expected to be in the `src/templates` directory. Note the use of `<G>` here, which is a Rust _type parameter_ (the `get_template` function can work for the browser or the server, so Rust needs to know which one it is). This parameter is _ambient_ to the `templates` key, which means you can use it without declaring it as long as you're inside `templates: {...}`. This will be set to `DomNode` for the browser and `SsrNode` for the server, but that all happens behind the scenes.

Also note that we're pulling in our error pages from another file as well (in a larger app you may even want to have a different file for each error page).

The last thing we do is new, we define `static_aliases` to map the URL `/test.txt` in our app to the file `static/test.txt`. This feature is detailed in more depth later, but it can be extremely useful, for example for defining your site's logo (or favicon), which browsers expect to be available at `/favicon.ico`. Create the `static/test.txt` file now (`static/` should NOT be inside `src/`!) and fill it with whatever you want.

## Error Handling

Before we get to the cool part of building the actual pages of the app, we should set up error pages again, which we'll do in `src/error_pages.rs`:

```rust,no_run,no_playground
{{#include ../../../examples/basic/src/error_pages.rs}}
```

This is a little more advanced than the last time we did this, and there are a few things we should note.

The first is the import of `GenericNode`, which we define as a type parameter on the `get_error_pages` function. As we said before, this means your error pages will work on the client or the server, and they're needed in both environments. If you're interested, this separation of browser and server elements is done by Sycamore, and you can learn more about it [here](https://docs.rs/sycamore/0.6/sycamore/generic_node/trait.GenericNode.html).

In this function, we also define a different error page for a 404 error, which will occur when a user tries to go to a page that doesn't exist. The fallback page (which we initialize `ErrorPages` with) is the same as last time, and will be called for any errors other than a _404 Not Found_.

## `index.rs`

It's time to create the first page for this app! But first, we need to make sure that import in `src/lib.rs` of `mod templates;` works, which requires us to create a new file `src/templates/mod.rs`, which declares `src/templates` as a module with its own code. Add the following to that file:

```rust,no_run,no_playground
{{#include ../../../examples/basic/src/templates/mod.rs}}
```

It's common practice to have a file for each _template_, which is slightly different to a page (explained in more detail later), and this app has two pages: a landing page (index) and an about page.

Let's begin with the landing page. Create a new file `src/templates/index.rs` and put the following inside:

```rust,no_run,no_playground
{{#include ../../../examples/basic/src/templates/index.rs}}
```

This code is _much_ more complex than the _Hello World!_ example, so let's go through it carefully.

First, we import a whole ton of stuff:

-   `perseus`
    -   `RenderFnResultWithCause` -- see below for an explanation of this
    -   `Template` -- as before
    -   `GenericNode` -- as before
-   `serde`
    -   `Serialize` -- a trait for `struct`s that can be turned into a string (like JSON)
    -   `Deserialize` -- a trait for `struct`s that can be *de*serialized from a string (like JSON)
-   `std::rc::Rc` -- same as before, you can read more about `Rc`s [here](https://doc.rust-lang.org/std/rc/struct.Rc.html)
-   `sycamore`
    -   `component` -- a macro that turns a function into a Sycamore component
    -   `template` -- the `template!` macro, same as before
    -   `Template as SycamoreTemplate` -- the output of the `template!` macro, aliased as `SycamoreTemplate` so it doesn't conflict with `perseus::Template`, which is very different

Then we define a number of different functions and a `struct`, each of which gets a section now.

### `IndexPageProps`

This `struct` represents the properties that the index page will take. In this case, we're building an index page that will display a greeting defined in this, specifically in the `greeting` property.

Any template can take arguments in Perseus, which should always be given inside a `struct`. For simplicity and performance, Perseus only ever passes your properties around as a `String`, so you'll need to serialize/deserialize them yourself (as in the functions below).

### `index_page()`

This is the actual component that your page is. Technically, you could just put this under `template_fn()`, but it's conventional to break it out independently. By annotating it with `#[component(IndexPage<G>)]`, we tell Sycamore to turn it into a complex `struct` that can be called inside `template!` (which we do in `template_fn()`).

Note that this takes `IndexPageProps` as an argument, which it can then access in the `template!`. This is Sycamore's interpolation system, which you can read about [here](https://sycamore-rs.netlify.app/docs/basics/template), but all you need to know is that it's basically seamless and works exactly as you'd expect.

The only other thing we do here is define an `<a>` (an HTML link) to `/about`. This link, and any others you define, will automatically be detected by Sycamore's systems, which will pass them to Perseus' routing logic, which means your users **never leave the page**. In this way, Perseus only pulls in the content that needs to change, and gives your users the feeling of a lightning-fast and weightless app.

_Note: external links will automatically be excluded from this, and you can exclude manually by adding `rel="external"` if you need._

### `get_template()`

This function is what we call in `lib.rs`, and it combines everything else in this file to produce an actual Perseus `Template` to be used. Note the name of the template as `index`, which Perseus interprets as special, which causes this template to be rendered at `/` (the landing page).

Perseus' templating system is extremely versatile, and here we're using it to define our page itself through `.template()`, and to define a function that will modify the document `<head>` (which allows us to add a title) with `.head()`. Notably, we also use the _build state_ rendering strategy, which tells Perseus to call the `get_build_props()` function when your app builds to get some state. More on that now.

#### `.template()`

The result of this function is what Perseus will call when it wants to render your template (which it does more than you might think), and it passes it the props that your template takes as an `Option<String>`. This might seem a bit weird, but there are reasons under the hood. All you need to know here is that if your template takes any properties, they **will** be here, and it's safe to `.unwrap()` them for deserialization.

#### `.head()`

This is very similar to `template_fn`, except it can't be reactive. In other words, anything you put in here is like a picture, it can't move (so no buttons, counters, etc.). This is because this modifies the document `<head>`, so you should put metadata, titles, etc. in here. Note that the function we return from here does take an argument (ignored with `_`), that's a string of the properties to your app, but we don't need it in this example. If this page was a generic template for blog posts, you might use this capability to render a different title for each blog post.

All this does though is set the `<title>`. If you inspect the source code of the HTML in your browser, you'll find a big comment in the `<head>` that says `<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->`, that separates the stuff that should remain the same on every page from the stuff that should update for each page.

### `get_build_props()`

This function is part of Perseus' secret sauce (actually _open_ sauce), and it will be called when the CLI builds your app to create properties that the template will take (it expects a string, hence the serialization). Here, we just hard-code a greeting in to be used, but the real power of this comes when you start using the fact that this function is `async`. You might query a database to get a list of blog posts, or pull in a Markdown documentation page and parse it, the possibilities are endless!

Note that this function returns a `RenderFnResultWithCause<String>`, which means that it returns a normal Rust `Result<String, E>`, where `E` is a `GenericErrorWithCause`, a Perseus type that combines an arbitrary error message with a declaration of who caused the error (either the client or the server). This becomes important when you combine this rendering strategy with others, which are explained in depth later in the book. Note that we use `?` in this example on errors from modules like `serde_json`, showing how versatile this type is. If you don't explicitly construct `GenericErrorWithCause`, blame for the error will be assigned to the server, resulting in a _500 Internal Server Error_ HTTP status code.

### `set_headers_fn()`

This function represents a very powerful feature of Perseus, the ability to set any [HTTP headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers) for a given template. In this case, any time the Perseus server successfully returns our template to the browser, it will call this function on the HTTP response just before it sends it, which will add our custom header `x-test`, setting it to the value `custom value`.

Note that this function has its own special return type, and that `HeaderMap` is distinct from other types, like a `HashMap`.

## `about.rs`

Okay! We're past the hump, and now it's time to define the (much simpler) `/about` page. Create `src/templates/about.rs` and put the following inside:

```rust,no_run,no_playground
{{#include ../../../examples/basic/src/templates/about.rs}}
```

This is basically exactly the same as `index.rs`, except we don't have any properties to deal with, and we don't need to generate anything special at build time (but Perseus will still render this page to static HTML, ready to be served to your users).

## Running It

`perseus serve`

That's all. Every time you build a Perseus app, that's all you need to do.

Once this is finished, your app will be live at <http://localhost:8080>! Note that if you don't like that, you can change the host/port with the `HOST`/`PORT` environment variables (e.g. you'd want to set the host to `0.0.0.0` if you want other people on your network to be able to access your site).

Hop over to <http://localhost:8080> in any modern browser and you should see your greeting `Hello World!` above a link to the about page! if you click that link, you'll be taken to a page that just says `About.`, but notice how your browser seemingly never navigates to a new page (the tab doesn't show a loading icon)? That's Perseus' _app shell_ in action, which intercepts navigation to other pages and makes it occur seamlessly, only fetching the bare minimum to make the new page load. The same behavior will occur if you use your browser's forward/backward buttons.

<details>
<summary>Why a 'modern browser'?</summary>

### Browser Compatibility

Perseus is compatible with any browser that supports Wasm, which is most modern browsers like Firefox and Chrome. However, legacy browsers like Internet Explorer will not work with any Perseus app, unless you _polyfill_ support for WebAssembly.

</details>

By the way, remember this little bit of code in `src/lib.rs`?

```rust,no_run,no_playground
{{#include ../../../examples/basic/src/lib.rs:12:14}}
```

If you navigate to <http://localhost:8080/test.txt>, you should see the contents on `static/test.txt`! You can also access them at <http://localhost:8080/.perseus/static/test.txt>

## Moving Forward

Congratulations! You're now well on your way to building highly performant web apps in Rust! The remaining sections of this book are more reference-style, and won't guide you through building an app, but they'll focus instead on specific features of Perseus that can be used to make extremely powerful systems.

So go forth, and build!
