# Incremental generation

One of the most powerful features of Perseus' state generation platform is the *incremental generation* system, which can be thought of as the request-time counterpart to the *build paths* strategy. Let's say you run an e-commerce website, and you have ten million products. Do you want to build ten million pages at build-time? Probably not!

A much better way of handling this would be to instead pre-render only your top 100 products or so at build-time (remember that Perseus builds are lightning fast after Rust compilation, so even that many is still light; this website generates several hundred documentation pages in less than half a second), and somehow render the others later, only when they're requested. This kind of 'on-demand' approach would be best if, when a user requested a page that wasn't prerendered at build-time, it's not just built for them, but also cached for future use, *as if* it had been built at build time. This kind of extension of the build process to just keep happening also allows you to add new products to your site in the future, and they'll be prerendered properly the first time somebody requests them (using [revalidation](:state/revalidation) on some kind of inventory page makes the most sense here).

All this is supported with literally one single line of code: `.incremental_generation()`. No arguments, no special functions, that's all you need, and Perseus will change its routing algorithm slightly to still match all the pages you render at build-time, but to also say "when a page under this template is requested that we don't know about yet, bear with it and try it out on the server anyway". The server will see if it's been prerendered in the past, and it'll provide it if it was, and otherwise it will run your `get_build_state` function, providing whatever path the user gave.

Of course, this could mean that somebody might go to the page `/product/faster-than-light-engine`, which might unfortunately still be in development, so that page shouldn't exist. And *this* is why we have `BlamedError<E>` in build state! So that you can say "if this page actually shouldn't exist, return an error that's blamed on the *client*, with HTTP status 404". This will be rendered by Perseus into a *404 Not Found* page automatically (but error views won't be cached, meaning that, if this product becomes available in the future, everything will work out).

Note that incremental generation is fully compatible with all other state generation methods, including request-time state generation and both forms of revalidation.

Here's an example of incremental generation:

```
{{#include ../../../examples/core/state_generation/src/templates/incremental_generation.rs}}
```

Note the use of build paths (you still have to generate *some* pages, otherwise incremental generation will be completely ignored and you'll just get an index page), and the conditional in `get_build_state` that checks for the illegal path `tests`, returning a `BlamedError` with blame `ErrorBlame::Client(Some(404))`, where `404` is the HTTP status code for a page not being found! Here, we're accompanying that with a `std::io::Error`, but you could use any error type you like.

Note that incrementally generated pages will be placed in the mutable store, which you shoudl keep in mind when deploying to read-only environments, such as serverless functions (see [here] for details).

<details>

<summary>How does Vercel handle that?</summary>

If you're from the JS world, you might be familiar with NextJS, which also supports incremental generation, but they offer a serverless function service that works with it seamlessly. Details about how this works are not public, but they seem to be using a colocated database setup to achieve this, or they may be using function-specific incremental caches (which would lead to lower performance, so this is unlikely).

You might wonder if Perseus could run in the same system. So have we, and this is an avenue we intend to explore in 2023.

</details>
