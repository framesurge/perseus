# Build State

The most commonly-used rendering strategy for Perseus is static generation, which renders your pages to static HTML files. These can then be served by the server with almost no additional processing, which makes for an extremely fast experience!

Note that, depending on other strategies used, Perseus may call this strategy at build-time or while the server is running, so you shouldn't depend on anything only present in a build environment (particularly if you're using the _incremental generation_ or _revalidation_ strategies).

_Note: if you want to export your app to purely static files, see [this section](../exporting.md), which will help you use Perseus without any server._

## Usage

### Without _Build Paths_ or _Incremental Generation_

On its own, this strategy will simply generate properties for your template to turn it into a page, which would be perfect for something like a list of blog posts (just fetch the list from the filesystem, a database, etc.). Here's an example from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/src/templates/index.rs) for a simple greeting:

```rust,no_run,no_playground
{{#include ../../../../examples/showcase/src/templates/index.rs}}
```

Note that Perseus passes around properties to pages as `String`s, so the function used for this strategy is expected to return a string. Note also the return type `RenderFnResultWithCause`, a Perseus type that represents the possibility of returning almost any kind of error, with an attached cause declaration that blames either the client or the server for the error. Most of the time, the server will be at fault (e.g. if serializing some obvious properties fails), and this is the default if you use `?` or `.into()` on another error type to run an automatic conversion. However, if you want to explicitly state a different cause (or provide a different HTTP status code), you can construct `GenericErrorWithCause`, as done in the below example (under the next subheading) if the path is `post/tests`. We set the error (a `Box<dyn std::error::Error>`) and then set the cause to be the client (they navigated to an illegal page) and tell the server to return a 404, which means our app will display something like _Page not found_.

### With _Build Paths_ or _Incremental Generation_

You may have noticed in the above example that the build state function takes a `path` parameter. This becomes useful once you bring the _build paths_ or _incremental generation_ strategies into play, which allow you to render many paths for a single template. In the following example (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/src/templates/post.rs)), all three strategies are used together to pre-render some blog posts at build-time, and allow the rest to be requested and rendered if they exist (here, any post will exist except one called `tests`):

```rust,no_run,no_playground
{{#include ../../../../examples/showcase/src/templates/post.rs}}
```

When either of these additional strategies are used, _build state_ will be passed the path of the page to be rendered, which allows it to prepare unique properties for that page. In the above example, it just turns the URL into a title and renders that.

For further details on _build paths_ and _incremental generation_, see the following sections.

## Common Pitfalls

When a user goes to your app from another website, Perseus will send all the data they need down straight away (in the [initial loads](../advanced/initial-loads.md) system), which involves setting any state you provide in a JavaScript global variable so that the browser can access it without needing to talk to the server again (which would slow things down). Unfortunately, JavaScript's concept of 'raw strings' (in which you don't need to escape anything) is quite a bit looser than Rust's, and so Perseus internally escapes any instances of backticks or `${` (JS interpolation syntax). This should all work fine, but, when your state is deserialized, it's not considered acceptable for it to contain *control characters*. In other words, anything like `\n`, `\t` or the like that have special meanings in strings must be escaped before being sent through Perseus! Note that this is a requirement imposed by the lower-level module [`serde_json`](https://github.com/serde-rs/json), not Perseus itself.
