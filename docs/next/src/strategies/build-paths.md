# Build Paths

As touched on in the documentation on the *build state* strategy, Perseus can easily turn one template into many pages (e.g. one blog post template into many blog post pages) with the *build paths* strategy, which is a function that returns a `Vec<String>` of paths to build.

Note that it's often unwise to use this strategy to render all your blog posts or the like, but only render the top give most commonly accessed or the like, if any at all. This is relevant mostly when you have a large number of pages to be generated. The *incremental generation* strategy is better suited for this, and it also allows you to never need to rebuild your site for new content (as long as the server can access the new content).

Note that, like *build state*, this strategy may be invoked at build-time or while the server is running if you use the *revalidation* strategy (*incremental generation* doesn't affect *build paths* though).

## Usage

Here's the same example as given in the previous section (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/src/templates/post.rs)), which uses *build paths* together with *build state* and *incremental generation*:

```rust,no_run,no_playground
{{#include ../../../../examples/showcase/src/templates/post.rs}}
```

Note the return type of the `get_build_paths` function, which returns a vector of `String`s on success or a `String` error.
