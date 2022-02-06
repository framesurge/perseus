# Incremental Generation

Arguably the most powerful strategy in Perseus is _incremental generation_, which is an extension of _build paths_ such that any path in the template's root path domain (more info on that concept [here](:reference/templates/intro)) will result in calling the _build state_ strategy while the server is running.

A perfect example of this would be an retail site with thousands of products, all using the `product` template. If we built all these with _build paths_, and they all require fetching information from a database, builds could take a very long time. Instead, it's far more efficient to use _incremental generation_, which will allow any path under `/product` to call the _build state_ strategy, which you can then use to render the product when it's first requested. This is on-demand building. But how is this different from the _request state_ strategy? It caches the pages after they've been built the first time, meaning **you build once on-demand, and then it's static generation from there**. In other words, this strategy provides support for rendering thousands, millions, or even billions of pages from a single template while maintaining static generation times of less than a second!

Also, this strategy is fully compatible with _build paths_, meaning you could pre-render you most common pages at build-time, and have the rest built on-demand and then cached.

## Usage

This is the simplest strategy in Perseus to enable, needing only one line of code. Here's the example from earlier (which you can find [here](https://github.com/arctic-hen7/perseus/blob/main/examples/core/state_generation/src/templates/incremental_generation.rs)) that uses _incremental generation_ together with _build paths_ (and of course _build state_, which is mandatory for _incremental generation_ to work):

```rust
{{#include ../../../../examples/core/state_generation/src/templates/incremental_generation.rs}}
```

All we need to do is run `.incremental_generation()` on the `Template`, and it's ready.

Note that this example throws a _404 Not Found_ error if we go to `/incremental_generation/tests`, which is considered an illegal URL. This is a demonstration of preventing certain pages from working with this strategy, and such filtering should be done in the _build state_ strategy.
