# The Index View

In most Perseus apps, you can just focus on building your app's templates, and leave the boilerplate entirely to the Perseus engine, but sometimes that isn't quite sufficient, like if you want to use one stylesheet across your entire app. In traditional architectures, these are the kinds of modifications you might make to an `index.html` file that a framework inserts itself into, and you can do exactly this with Perseus! If you provide an `index.html` file in the root of your project (not inside `src/`), Perseus will insert itself into that!

However, if you're using Perseus, you probably don't want to be writing HTML right? You're supposed to be using Sycamore! Well, that's completely true, and so Perseus supports creating an index view with Sycamore code! You can do this like so:

```rust
{{#include ../../../examples/core/index_view/src/main.rs}}
```

Note that you can also use `.index_view_str()` to provide an arbitrary HTML string to use instead of Sycamore code.

It's also important to remember that whatever you put in your index view will persist across *all* the pages of your app! There is no way to change this, as Perseus literally injects itself into this, using it as a super-template for all your other templates!

## Requirements for the Index View

Perseus' index view is very versatile, but there are a few things you HAVE to include, or Perseus moves into undefined behavior, and almost anything could happen! This mostly translates to your app just spitting out several hundred errors when it tries to build though, because none of the tactics Perseus uses to insert itself into your app will work anymore.

1. You need a `<head>`. This can be empty, but it needs to be present in the form `<head></head>` (no self-closing tags allowed). The reason for this is that Perseus uses these tags as markers for inserting components of the magical metadata that makes your app work.
2. You need a `<body>`. This needs to be defined as `<body></body>`, for similar reasons to the `<head>`.
3. You need a `<div id="root"></div>`. Literally, you need that *exact* string in your index view, or Perseus won't be able to find your app at all! Now, yes we could parse the HTML fully and find this by ID, or we could just use string replacement and reduce dependencies and build time. Importantly, you can't use this directly is you use `.index_view()` and provide Sycamore code, as Sycamore will add some extra information that stuffs things up. Instead, you should use `perseus::PerseusRoot`, which is specially designed to be a drop-in entrypoint for Perseus. It should go without saying that you need to put this in the `<body>` of your app.

*Note: you don't need the typical `<!DOCTYPE html>`in your index view, since that's all Perseus targets, so it's added automatically. If, for some magical reason, you need to override this, you can do so with a [control plugin](:reference/plugins/control).*
