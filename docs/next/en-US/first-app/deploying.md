# Deploying!

Congratulations on making it through the tutorial, it's time to deploy your app! First, though, we haven't actually run it yet, so we may as well make sure it all compiles. Remember, you an always do this quickly with `perseus check`, which should give all ticks if everything's working. If not, you've probably just mistyped a variable name or something, which happens to us all. If you're having trouble, let us know [in a GitHub discussion], or [on Discord], and we'll be happy to help! (And remember, there are no stupid questions or dumb bugs.)

When you're ready to actually run your app, you can run `perseus serve`, which will prepare everything to be run for development. When it's ready, you'll be able to see your brand-new app at <http://localhost:8080>, in all its *Hello World!* glory! If you try clicking on the link to the *About* page, you should find that the page doesn't seem to change from the browser's perspective, it just instantly updates: this is Perseus' router in action. Press the back button in your browser to pop back to the landing page, and, again, it should be near-instant (Perseus has *cached* the index page, and can return to it with no network requests needed).

<details>

<summary>I'm throttling my network connection, and Perseus seems extremely slow...</summary>

A lot of DevTools in browsers have the option to throttle your network connection, to emulate how long it would take to load a real app. If you do this with Perseus, however, it will probably take around a full minute to even load your app. You'll see content very quickly because of Perseus' preloading system, but the `bundle.wasm` file will take forever. This is because, in development, Wasm bundles are *huge*. What will optimize and compress down to the size of a small cat photo can start as a muilti-megabyte behemoth, and this is why it's usually not a good idea to throttle Perseus apps to test their load-speed. If you wait for the Wasm bundle to load though, and *then* throttle, you'll get a better idea of real-world performance (if your browser supports this).

</details>

If that's all working, you might want to try going to <http://localhost:8080/foo>, which is a non-existent page. You should see a lovely *Page not found* message, and that's error handling in action!

## Deploying

But enough development shenanigans, we want to deploy this thing! To deploy a Perseus app, you'll need to make sure you've defined your [error views](:first-app/error-handling), because Perseus won't let you use the default implied error views in production.

When you're ready, just run this command:

```
perseus deploy
```

It's literally that easy. And, because Rust is a really nice programming language, something that works in development is basically guaranteed to work in production.

Note that this command will take a rather long time, especially on older machines, because it's applying aggressive optimizations to everything to keep bundle sizes down and page loads speedy, while also trying to keep your app as fast as possible. All these optimizations are configurable, but the defaults are tuned to be sensible, and the `deploy` command does pretty much everything automatically. Usually, there's no post-processing to be done at all.

When it's done, this command wil produce a `pkg/` folder in the root of your project that you can send to any server under the sun. Just tell it to run the `pkg/server` binary, and your app will run beautifully! (But make sure you don't tamper with the contents of this folder, because Perseus needs everything to be in just the right place, otherwise we get one of those crash-and-burn-in-production scenarios.) In fact, try running that binary right now, and you should see your app on <http://localhost:8080> once more!

Obviously, you probably want to host your app in production on a different address, like `0.0.0.0` (network-speak for "host this everywhere so everyone who comes to my server can find it"), and perhaps on port `80`. Note that Perseus doesn't handle HTTPS at all, and you'll need to do this with a reverse proxy or the like (which comes built-in to most servers these days). You can set the host and port with the `PERSEUS_HOST` and `PERSEUS_PORT` environment variables.

## Export deployment

However, there's actually a simpler way of deploying this app in particular. Because we aren't using any features that need a server (e.g. we're generating state at build-time, not request-time, so all the server is doing is just passing over files that it generated when we built the app), we can *export* our app. You can try this for development with `perseus export -s` (the `-s` tells Perseus to spin up a file server automatically to serve your app for you). In production, use `perseus deploy -e` to make `pkg/` contain a series of static files. If you have `python` installed on your computer, you can serve this with `python -m http.server -d pkg/`. The nice thing about exported apps is that they can be sent to places like [GitHub Pages], which will host your app for free. In fact, this whole website is exported (because it's all static documentation), and hosted on exactly that service!

## Closing words

With all that, you've successfully built and deployed your first ever Perseus app: well done! If you're liking Perseus so far, you can check out the rest of the documentation to learn about its features in greater detail, and [the examples] will be your friends in writing real-world Perseus code: they have examples of every single Perseus feature. If you think you've found a bug, please let us know by [opening an issue], or, if you'd like to contribute a feature, you can [open a pull request]. If you're having trouble, you can [open a GitHub discussion] or [as on our Discord], and our friendly community will be happy to help yout out! Also, make sure to take a look at [the Sycamore docs] to learn more about the library that underlies all of Perseus.

Best of luck in your journey, and, if you [defeat Medusa](https://en.wikipedia.org/wiki/Perseus), let us know!
