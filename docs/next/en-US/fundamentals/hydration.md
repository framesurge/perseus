# Hydration

One of the most important parts of Perseus is its prerendering system, which allows it to turn Rust code that renders HTML into actual HTML ahead of time, making sure that your users see content immediately. However, there's an art to going from prerendered content to a functional webpage, adn that art is *hydration*.

Internally, Perseus does *a heck of a lot* in the prerendering process, but it eventually comes down to Sycamore's `render_to_string()` function (or, rather, an internal re-implementation of it with some tweaks), which inserts some little numbers throughout your HTML called *hydration IDs*. These allow Sycamore to then *hydrate* your page after an *initial load*.

## Initial and subsequent loads

But, in order to really understand hydration, we need to understand a bit more about how Perseus handles routing. There are two distinct types of page loads in Perseus, which are handled *completely* differently.

Initial loads are the first kind, and these occur when a user comes to a page on your site from somewhere else on the internet. For example, if someone searches up your site on Google and then clicks on the provided link, this would be an initial load. What makes these special is that the user has *absolutely nothing* about your site, which means they need to be sent what we call the *app shell*, which consists of a `bundle.js` file, and a `bundle.wasm` file, which will take over the responsibility of routing from the browser, making sure that future renders will be much faster. We'll get to that though. However, if we were just to do this, and then leave Perseus to render your site on the client-side, this would be a *terrible* loading experience, because it takes a moment to download a Wasm bundle and run it. In the interim period, the user would be staring at a blank page, waiting --- not good!

So, Perseus bundles this together with an HTML file that's automatically generated. This will contain the prerendered version of whatever page the user has requested, meaning they can see some content *immediately*. The Wasm bundle can then take a moment to load in the background, and the code will be made interactive afterward. We'll get to what that means in a moment.

However, once the user has loaded all that once, they don't need to get the app shell again, because it definitionally remains the same for every page. Therefore, Perseus only needs to fetch the state of the next page the user loads (along with its head metadata), because Rust is so darn fast at rendering HTML (it's actually slower to fetch an HTML fragment and then hydrate it for subsequent loads!), so it takes over from the browser at handling routing and minimizes the number of network requests performed. This also allows Perseus to do application-level caching, which dramatically improves performance when a user goes back to a page they've already visited (by "improves performance", we mean it becomes *literally instantaneous*),

And that's the difference between initial loads and subsequent ones! In initial loads, there's a whole package, and in subsequent loads, there's just the state of whatever page is being rendered next.

## Hydration

As explained just now, Perseus *prerenders* your pages to HTML on the engine-side, as early as it can (usually at build-time, unless you're doing something that needs access to the user's request, like accessing cookies for check authentication). Unfortunately though, there's quite a gap between static HTML (which is literally a string) and an interactive page: that gap is *event handlers*. When a user pushes a button, the browser needs to know what to do, and that can't be encoded in stringified HTML.

<details>

<summary>Yes it can be!</summary>

Okay, if you're used to using vanilla JavaScript, you might be recalling how you could write something like this:

```html
<button onclick="console.log('Clicked!')">Click!</button>
```

In this case, the event handler is absolutely encoded in the static HTML, but this doesn't play well with Wasm at all just yet. One day, hopefully, we'll be able to encode event handlers properly in static HTML, but this may actually be slower than the approach we're about to describe.

You might also be familiar with a framework like [Qwik](https://qwik.builder.io), which uses something called *resumability*, which translates to basically bringing framework complexities back to the basics of string Javascript inside HTML. Unfortunately, this isn't yet possible with Wasm. Perseus does the next-best thing.

</details>

The solution to this problem is *hydration*, where a program goes through the HTML string and checks it against your code, adding event handlers as it needs. This is quite a complex process very often, and it's part of Sycamore, not Perseus, so the credit for this should go entirely to them!

If you're from JS-land, you might be used to hydration taking quite a while, during which time users can't click buttons or anything. Unlike JS, Wasm comes compiled, and can be executed piece-by-piece as the browser gets more of it from the server. This, plus the fact that Rust is just crazily fast, means Perseus (really Sycamore) does hydration practically instantly: the limiting factor is not hydration, but rather the size of the Wasm bundle, which influences how long it takes to download the whole thing, which is why it's important to optimize Wasm for size (which `perseus deploy` does automatically, and very aggressively).
