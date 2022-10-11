# Hydration

In the examples of `Cargo.toml` files shown thus far, we've enabled a feature called `hydrate` on Perseus. This feature controls _hydration_, which is a way of making your app more performant. To explain it, we'll need to go a little in-depth.

Perseus uses server-side rendering of some kind for almost everything. In fact, unless you explicitly make something only run in the browser, Perseus will try to prerender it on the server first, either at build-time (faster, so Perseus does this for everything it can) or at request-time. This prerendering process yields a series of HTML and JSON files that make up the markup and state of your app. When a page is requested by a user, Perseus can serve them these files, and then the app shell (the Wasm code that runs everything in a Perseus app in the browser) will bring everything to life.

Those prerendered files can be imagined as solid iron, but, to make your app work, we need molten iron. In the real world, you need a lot of heat to turn iron into a liquid, and you need a lot of code to turn simple markup into interactive buttons and text in a browser! So, let's go through the metaphor a bit more: the build process and server are the miners that fetch all the iron out of the mines of the template code you write. Then, that iron is sent to the user's browser, and the app shell does _something_ to get molten iron that can be used to run your app.

Without hydration, the app shell will kindly thank the server for sending it the solid iron, and will then proceed to mine more of its own. In other words, the app shell will completely ignore the prerendered files that the server has sent (displaying them only until it's ready, which is why Perseus apps still work without JavaScript!).

But with hydration, the app shell can intelligently melt the iron that it's been given, it can _hydrate_ the simple markdown. Using hydration is generally much faster than not using hydration, though it's also very hard to implement! Hydration is done by Sycamore, and it's still experimental right now, so it's opt-in with Perseus. You can use the `hydrate` feature flag to enable it in any Perseus app, though you should be aware that there's a chance that things may break in very strange ways! If this happens, try disabling hydration.

## Performance Costs of Disabling Hydration

Not using hydration will impact your site's performance when the user first loads a page (moving around within the app is no problem), because the browser has to do a little more work, but it also has to completely re-display your site. The difference shouldn't be visible to users at all unless they try to scroll as soon as your site loads (as in within less than half a second on modern machines), in which case they'll be thrown back to the top of the site. However, Lighthouse doesn't seem to notice any differences, so your scores there won't change!

Notably, to make hydration better for the community, you should file any bugs about hydration on [the Sycamore repository](https://github.com/sycamore-rs/sycamore).
