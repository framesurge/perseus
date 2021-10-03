# Build State

The most commonly-used rendering strategy for Perseus is static generation, which renders your pages to static HTML files. These can then be served by the server with almost no additional processing, which makes for an extremely fast experience!

Note that, depending on other strategies used, Perseus may call this strategy at build-time or while the server is running, so you shouldn't depend on anything only present in a build environment (particularly if you're using the _incremental generation_ or _revalidation_ strategies).

_Note: Perseus currently still requires a server if you want to export to purely static files, though standalone exports may be added in a future release. In the meantime, check out [Zola](https://getzola.org), which does pure static generation fantastically._

## Usage

### Without _Build Paths_ or _Incremental Generation_

On its own, this strategy will simply generate properties for your template to turn it into a page, which would be perfect for something like a list of blog posts (just fetch the list from the filesystem, a database, etc.). Here's an example from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/src/templates/index.rs) for a simple greeting:

```rust
{{#include ../../../../examples/showcase/src/templates/index.rs}}
```

Note that Perseus passes around properties to pages as `String`s, so the function used for this strategy is expected to return a string. Note also the return type `StringResultWithCause`, which means you can specify an error as `(String, perseus::errors::ErrorCause)`, the later part of which can either be `Client(Option<u16>)` or `Server(Option<u16>)`. The `u16`s allow specifying a custom HTTP status code, otherwise the defaults are _400_ and _500_ respectively. This return type allows specifying who's responsible for an error. This is irrelevant if you use this strategy on its own or with _build paths_, but if you bring in _incremental generation_, this will be necessary.

### With _Build Paths_ or _Incremental Generation_

You may have noticed in the above example that the build state function takes a `path` parameter. This becomes useful once you bring the _build paths_ or _incremental generation_ strategies into play, which allow you to render many paths for a single template. In the following example (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/src/templates/post.rs)), all three strategies are used together to pre-render some blog posts at build-time, and allow the rest to be requested and rendered if they exist (here, any post will exist except one called `tests`):

```rust
{{#include ../../../../examples/showcase/src/templates/post.rs}}
```

When either of these additional strategies are used, _build state_ will be passed the path of the page to be rendered, which allows it to prepare unique properties for that page. In the above example, it just turns the URL into a title and renders that.

For further details on _build paths_ and _incremental generation_, see the following sections.
