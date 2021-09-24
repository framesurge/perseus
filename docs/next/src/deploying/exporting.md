# Static Exporting

The easiest way to deploy Perseus is as a set of static files, which is supported if your app uses only the _build state_ and _build paths_ strategies, or none at all. If you use _incremental generation_, _revalidation_, or _request state_ in any of your templates though, you can't export your app to static file, because these strategies require a custom server. For these cases, please continue to the rest of this section to learn how to deploy your more complex setup.

However, if your app only needs to run server-side computations at build-time, then you can export it to a set of static files without changing anything, simply by running `perseus export`. This will create a new directory called `.perseus/dist/exported`, the contents of which can be served on a system like [GitHub Pages](https:://pages.github.com). Your app should behave in the exact same way with exporting as with normal serving. If this isn't the case, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose).

There is only one known difference between the behavior of your exported site and your normally served site, and that's regarding [static aliases](../static-content.md). In a normal serving scenario, any static aliases that conflicted with a Perseus page or internal asset would be ignored, but, in an exporting context, **any static aliases that conflict with Perseus pages will override them**! If you suspect this might be happening to you, try exporting without those aliases and make sure the URL of your alias file doesn't already exist (in which case it would be a Perseus component).

## File Extensions

One slight hiccup with Perseus' static exporting system comes with regards to the `.html` file extension. Perseus' server expects that pages shouldn't have such extensions (hence `/about` rather than `/about.html`), but, when statically generated, they must have these extensions in the filesystem. So, if you don't want these extensions for your users (and if you want consistent behavior between exporting and serving), it's up to whatever system you're hosting your files with to strip these extensions. Many systems do this automatically, though some (like Python's `http.server`) do not.

One of the best systems for testing static exporting on your local machine is the [`serve`](https://github.com/versel/serve) JavaScript package, which can be run from the command-line without touching any JavaScript, and it handles this problem automatically. However, other solutions certainly exist if you don't want any JS polluting your system!
