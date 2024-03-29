# Static Exporting

Thus far, we've used `perseus serve` to build and serve Perseus apps, but there is an alternative way that offers better performance in some cases. Namely, if your app doesn't need any rendering strategies that can't be run at build time (so if you're only using _build state_ and/or _build paths_ or neither), you can export your app to a set of purely static files that can be served by almost any hosting provider. You can do this by running `perseus export`, which will create a new directory `.perseus/dist/exported/`, the contents of which can be served on a system like [GitHub Pages](https:://pages.github.com). Your app should behave in the exact same way with exporting as with normal serving. If this isn't the case, please [open an issue](https://github.com/framesurge/perseus/issues/new/choose).

There is only one known difference between the behavior of your exported site and your normally served site, and that's regarding [static aliases](:reference/static-content). In a normal serving scenario, any static aliases that conflicted with a Perseus page or internal asset would be ignored, but, in an exporting context, **any static aliases that conflict with Perseus pages will override them**! If you suspect this might be happening to you, try exporting without those aliases and make sure the URL of your alias file doesn't already exist (in which case it would be a Perseus component).

## File Extensions

One slight hiccup with Perseus' static exporting system comes with regards to the `.html` file extension. Perseus' server expects that pages shouldn't have such extensions (hence `/about` rather than `/about.html`), but, when statically generated, they must have these extensions in the filesystem. So, if you don't want these extensions for your users (and if you want consistent behavior between exporting and serving), it's up to whatever system you're hosting your files with to strip these extensions. Many systems do this automatically, though some (like Python's `http.server`) do not.

Note that, in development, you can easily serve your app with `perseus export -s`, which will spin up a local server automatically!
