# Your Second App

Now it's time to build a more realistic app with Perseus, one that takes advantage of the state platform and uses a structure more similar to one you'd see in a real Perseus app. All the code for this tutorial is available [here](https://github.com/arctic-hen7/perseus/tree/main/examples/core/basic).

Note that this tutorial assumes you've already installed Rust and the Perseus CLI as per [these instructions](:getting-started/installation).

## Setup

We can create a new Perseus project by going to some directory and running `perseus new my-app`, which will create a folder called `my-app/` and set it up for Perseus. Then, you can pop in there and make sure `Cargo.toml` looks like the following:

```toml
{{#include ../../../examples/core/basic/Cargo.toml.example}}
```

This is almost identical to the [first app](:getting-started/first-app) we built, so we'll skip over further explanation here. Recall though that this structure of declaring engine-side and browser-side dependencies separately is a fairly standard pattern in Perseus.

Next, create some files and folders so that your project tree looks like this:

```
├── Cargo.toml
└── src
    ├── error_pages.rs
    ├── lib.rs
    └── templates
        ├── about.rs
        ├── index.rs
        └── mod.rs
```

Great, now you've got your basic directory setup for a Perseus project! For a quick foreshadowing, we'll be putting our app declaration in `src/lib.rs`, our error pages (for 404 etc.) in `src/error_pages.rs`, adn each of the files in `templates/` will correspond to a template (which could generate as many pages as it wants, see [here](:core-principles) for an explanation of this).

Finally, add the following to the top of `src/lib.rs` so that Cargo knows about this structure:

```rust
mod error_pages;
mod templates;
```

And then make `src/templates/mod.rs` look like the following to declare the files in there to Cargo:

```rust
{{#include ../../../examples/core/basic/src/templates/mod.rs}}
```

The reason these are `pub mod`s is so that we can access them from `lib.rs` easily.

## Index template

Let's jump right into the code of this app! We'll start with the index template, which will render the landing page of our site. The code for this is accordingly in `src/templates/index.rs`.

In `src/templates/index.rs`, dump the following code (replacing the auto-generated code):

```rust
{{#include ../../../examples/core/basic/src/templates/index.rs}}
```

This is much more complex than our first app, so let's dive into explanation. Note that the imports at the top of this file will be explained as we go.

The first thing then is `IndexPageState`, which is our first usage of Perseus' state platform. As explained [here](:core-principles), a page in Perseus is produced from a template and some state. In this template, we'll only be rendering one template, but it will use some state to demonstrate how we can execute arbitrary code when we build our app to create pages. In this case, our state is dead simple, containing only one property, `greeting`, which is a `String`.

Importantly, we've annotated that with `#[perseus::make_rx(IndexPageStateRx)]`, which will create a version of this `struct` that uses Sycamore's `Signal`s: a reactive version. If you're unfamiliar with Sycamore's reactivity system, you should read [this](https://sycamore-rs.netlify.app/docs/basics/reactivity) quickly before continuing.

Next, we create a function called `index_page`, which we annotate with `#[perseus::template_rx]`. That macro is used for declaring templates, and you can think of it like black box that makes things work.

<details>
<summary>Details??</summary>

What that macro actually does depends on the complexity of your template, but the core purpose is to make sure it gets the right state. Internally, Perseus passes around all state as serialized `String`s, since it needs to be sent over the network from the server. This macro performs deserialization for you, and registers the state with the app-wide state management system if it's the first load of it. If not, it will restore previous state, meaning, for example, that user inputs can retain their content even if the user goes to three other pages in your app before returning, with no extra code from you. The workings of these macros aren't too complex, but they are extremely unergonomic.

If you really viscerally hate macros though, then you *could* implement the under-the-hood stuff manually based on [this file](), but we seriously wouldn't recommend it. Also, that code could change at any time, which means any update could be a breaking change for you.

*Note: these macros are progressively becoming less and less important to Perseus. Eventually, we hope to reduce them to the absolute minimum necessary for functionality.*

</details>

This function takes two arguments: a Sycamore reactive scope and the reactive version of the state, which both share the same lifetime `'a`. Don't worry though, we won't have to worry about these lifetimes most of the time, Sycamore is very well-designed to make them stay out of our way! They're just there to make things much more ergonomic and speedy. (In older version,s you had to `.clone()` *everything*.)

Finally, we produce a Sycamore [`View`](=struct.View@sycamore), which will render content to the user's browser. Notably, this function is generic over a type parameter `G: Html`, which we use to make sure this code can be run on both the engine-side and the browser-side.

<details>
<summary>Wait up, why are my templates being rendered on the engine-side?</summary>

To improve performance *dramatically*, Perseus renders all pages on the engine-side before your app ever gets to users, creating fully-built HTML that can be sent down at a moment's notice, meaning users see pages quickly, and then they become interactive a moment later. Generally, this is agreed to be much better than users having to wait potentially several seconds to see anything on your site at all. 

As a result of this, the code in any template function must be able to run on both the browser-side and the server-side. But, you can always use `#[cfg(target_arch = "wasm32")]` to gate browser-only code, or `#[cfg(not(target_arch = "wasm32"))]` to gate engine-only code.

</details>

Inside this function, we use Sycamore's [`view!`](=macro.view@sycamore) macro to a create a view for the user, which will be displayed in their browser. We provide the reactive scope `cx`, and then we have just two items we're rendering. The second is a simple link to `about`, which is the same as `/about`, but without the absolutism that the route has to be at the top-level (instead, it will be relative to the rest of the site, which lets you serve your entire app inside another website trivially --- it's exactly what's done on this website!).

The first element is a paragraph that contains some dynamic content. Specifically, the value of that `greeting` property in our state. Notably, we're calling `.get()` on that, because, remember, we're using the reactive version, so it's not a `String` anymore, it's a `&'a Signal<String>`! Again, you don't need to worry about the lifetimes, Sycamore makes all that seamless for you.

Notably, we could actually change this value at runtime if we wanted by calling `.set()`, but we won't do that in this example.

The next function we define is `get_template()`, which is fairly straightforward. It just declares a [`Template`](=struct.Template@perseus) with the necessary properties. Specifically, we define the function that actually renders the template as `index_page`, and the other two we'll get to now.

The first of those is the `head()` function, which is annotated as `#[perseus::head]` (which has similar responsibilities to `#[perseus::template_rx]`). In HTML, the language for creating views on the web, there are two main components to every page: the `<body>` and the `<head>`, the latter of which defines certain metadata, like the title, and any stylesheets you need, for example. If `index_page()` creates the body, then `head()` creates the head in this example. Notably, because the head is rendered only ahead of time, it can't be reactive. For that reason, it's passed the unreactive version of the state, and, rather than being generic over `Html`, it uses [`SsrNode`](=struct.SsrNode@perseus), which is specialized for the engine-side.

Because this function will only ever run on the engine-side, `#[perseus::head]` implies a target-gate to the engine (i.e. `#[cfg(not(target_arch = "wasm32"))]` is implicit). This means you can use engine-side dependencies here without any extra gating.

Finally, `get_build_state()` is responsible for generating an instance of `IndexPageState` that the template will be rendered with ahead of time on the engine-side. In this example, this logic is very simple, just generating a static `greeting`, but, in more complex apps, this might fetch information from a database, or it could run more complex computations.

For example, this very website uses build-time state generation to fetch the content for each of these docs pages from Markdown, rendering then to HTML, making the experience of both writing and viewing these docs as smooth as possible!

Importantly, that function takes two parameters: the path of the page (only relevant if you're using *build paths* too) and the locale (only relevant if you're using internationalization). Crucially, we return a [`RenderFnResultWithCause`](=struct.RenderFnResultWithCause@perseus), which is basically a glorified `Result` that lets you return any error type you want. But, we need to do one more thing if we get an error in state generation: we need to know who's responsible. You're probably familiar with the 404 HTTP status code, meaning the page wasn't found, but there are actually dozens of these, all with different meanings (like 418, which indicates the server is a teapot incapable of brewing coffee). The 4xx codes are for when the client caused the problems, and the 5xx codes are for when the server caused the problem. For the Perseus server to know which of these to send, it needs to know who was responsible, which `RenderFnResultWithCause` lets you declare. For an example of how to return errors from here like this, see [here]().

<details>
<summary>But we're generating on the engine-side...</summary>

It may seem like the client could never be responsible, since we're generating state at build time. That's true, unless you're using *incremental generation*, which is another state generation strategy that means functions like `get_build_state()` could be executed on the engine-side while the server is running in production, and the `path` parameter can be arbitrary. In these cases, the client can most certainly cause an error.

If none of that makes sense, don't worry, you can learn more about it [here]().

</details>

## About template

With that done, we can build the second template of this app, which is much simpler! Add the following to `src/templates/about.rs`:

```rust
{{#include ../../../examples/core/basic/src/templates/about.rs}}
```

This is basically a simpler version of the index template, with no state, and this template only defines a simple view and some metadata in the head.

Importantly, this illustrates that templates that don't take state don't have to have a second argument for their nonexistent state, the `#[perseus::template_rx]` macro is smart enough to handle that (and even a third argument for global state).

## Error pages

Before we tie everything together, we've got to handle errors in this app! If the user goes to a page that doesn't exist right now, they'll be greeted with a stock error page designed for development debugging and fast iteration. In production, we won't even be allowed to build our app unless we set up some error handling.

Add the following to `src/error_pages.rs`:

```rust
{{#include ../../../examples/core/basic/src/error_pages.rs}}
```

This is a very simplistic error page setup, but it illustrates pretty well what error pages actually are in Perseus. Essentially, you define a new [`ErrorPages`](=struct.ErrorPages@perseus) instance that's again generic over `Html` so it can work on the engine-side and the browser-side. In that `::new()` function, you need to provide a fallback page, because you're unlikely to provide a different error page for every possible HTTP status code. If one occurs that you haven't explicitly handled for, this fallback page will be used. Then, we use `.add_page()` to add another page for the 404 HTTP status code (page not found).

Notably, an error page is defined with a closure that takes four arguments: a reactive scope, the URL the user was on when the error occurred (which they'll stay on while the error page is displayed), the HTTP status code, teh actual `String` error message, and a translator (but we aren't using i18n, so we don't need this).

## Tying it all together

Now, we can bring everything together in `src/lib.rs`:

```rust
{{#include ../../../examples/core/basic/src/lib.rs}}
```

**Important:** replace `perseus_integration` here with `perseus_warp`! We use `perseus_integration` as an internal glue crate, and all code in these docs is sourced directly from the examples.

This is quite similar to the first app we built, though with a few more complexities. As in that app, we declare a `main()` function annotated with `#[perseus::main(...)]` to declare the entrypoint of our app. In there, we define the function that will spin up our server, here just using the `dflt_server` of our chosen integration. In the `Cargo.toml` above, we used `perseus_warp`, but you could trivially use any integration you like (or whatever works with your existing servers, which Perseus can extend).

Then, on our [`PerseusApp`](=struct.PerseusApp@perseus), we define the two templates, and our error pages. Simple as that!

## Running it

```shell
perseus serve -w
```

This will compile your whole app (which might take a while for the first time), and then serve it at <http://localhost:8080>! If you take a look there, you should be greeted with *Hello World!* and a link to the about page, which should take you there without causing the browser to load a new page. This demonstrates how Perseus internally switches pages out with minimal requests to the server, using less bandwidth and enabling faster page transitions.

Now, try changing that *Hello World!* greeting to be the more historically accurate *Hello, world!* in `src/templates/index.rs`, and watch as the CLI automatically recompiles your app and reloads the browser so you can see your changes!

*Note: this simple change will probably take a fair while to recompile for. See [here](:reference/compilation-times) for how to optimize this.* 

Finally, try running `perseus deploy`, and you'll get a `pkg/` folder at the root of your project with a `server` executable, which you can run to serve your app in production! With any Perseus app, that `pkg/` folder can be sent to a server and hosted live!
