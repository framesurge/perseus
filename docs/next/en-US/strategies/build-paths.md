# Build Paths

As touched on in the documentation on the _build state_ strategy, Perseus can easily turn one template into many pages (e.g. one blog post template into many blog post pages) with the _build paths_ strategy, which is a function that returns a `Vec<String>` of paths to build.

Note that it's often unwise to use this strategy to render all your blog posts or the like, but only render the top give most commonly accessed or the like, if any at all. This is relevant mostly when you have a large number of pages to be generated. The _incremental generation_ strategy is better suited for this, and it also allows you to never need to rebuild your site for new content (as long as the server can access the new content).

Note that, like _build state_, this strategy may be invoked at build-time or while the server is running if you use the _revalidation_ strategy (_incremental generation_ doesn't affect _build paths_ though).

## Usage

Here's the same example as given in the previous section (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/i18n/templates/post.rs)), which uses _build paths_ together with _build state_:

```rust,no_run,no_playground
{{#include ../../../../examples/i18n/src/templates/post.rs}}
```

Note the return type of the `get_build_paths` function, which returns a `RenderFnResult<Vec<String>>`, which is just an alias for `Result<T, Box<dyn std::error::Error>>`, which means that you can return any error you want. If you need to explicitly `return Err(..)`, then you should use `.into()` to perform the conversion from your error type to this type automatically. Perseus will then format your errors nicely for you using [`fmterr`](https://github.com/arctic-hen7/fmterr).

Also note how this page renders the page `/docs` by specifying an empty string as one of the paths exported from `get_build_paths`.
