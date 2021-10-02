# Incremental Generation

Arguable the most powerful strategy in Perseus is *incremental generation*, which is an extension of *build paths* such that any path in the template's root path domain (more info on that concept [here](../templates/intro.md)) will result in calling the *build state* strategy while the server is running.

A perfect example of this would be an retail site with thousands of products, all using the `product` template. If we built all these with *build paths*, and they all require fetching information from a database, builds could take a very long time. Instead, it's far more efficient to use *incremental generation*, which will allow any path under `/product` to call the *build state* strategy, which you can then use to render the product when it's first requested. This is on-demand building. But how is this different from the *request state* strategy? It caches the pages after they've been built the first time, meaning **you build once on-demand, and then it's static generation from there**. In other words, this strategy provides support for rendering thousands, millions, or even billions of pages from a single template while maintaining static generation times of less than a second!

Also, this strategy is fully compatible with *build paths*, meaning you could pre-render you most common pages at build-time, and have the rest built on-demand and then cached.

## Usage

This is the simplest strategy in Perseus to enable, needing only one line of code. Here's the example from earlier (which you can find [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/src/templates/post.rs)) that uses *incremental generation* together with *build paths* (and of course *build state*, which is mandatory for *incremental generation* to work):

```rust,no_run,no_playground
{{#include ../../../../examples/showcase/src/templates/post.rs}}
```

All we need to do is run `.incremental_generation()` on the `Template`, and it's ready.

Note that this example throws a *404 Not Found* error if we go to `/post/tests`, which is considered an illegal URL. This is a demonstration of preventing certain pages from working with this strategy, and such filtering should be done in the *build state* strategy.
