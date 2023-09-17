# Error Views

If there's one thing that's guaranteed to happen in all software, it would be errors, and, when they do happen, your app needs to be prepared for them. Internally, there are quite a few failure scenarios that Perseus might encounter: for example, if the user loses their internet connection while Perseus is trying to fetch a page, it won't be able to complete that. This would be a failure in the *framework*, not your app. In these cases, as well as in some cases of failures caused by your code (e.g. a mistyped link address, a deliberate authentication failure, etc.), Perseus will render what we call *error views*. These are basically special Sycamore `View`s that your app holds onto until an error occurs, at which time it will automatically display them appropriately. 

One thing to make clear about error views is that *you don't invoke these, Perseus does*. What you are writing are a series of instructions to Perseus on what to do in each type of failure: you are **not** writing error handling for your own logic. If, for example, a user enters a password that's too short and submits a form, you would not display a Perseus error view, you would handle errors manually there. The reason for this is that there are simply too many cases of error handling in real-world apps for Perseus to be able to reasonably and flexibly handle them all.

By convention, error views are usually placed in an `src/error_views.rs` file in your project, though this could also be a folder if your error views are particularly elaborate.

## `ClientError`

All errors in Perseus, even ones that occur on the server, fall under the [`ClientError`](=errors/enum.ClientError@perseus) `enum`, which you'll typically `match` against to determine what to render. In terms of semantic versioning, this `enum` is considered stable, and new variants will not be added except in a breaking change. Let's go through those variants one by one.

### `ServerError`

A `ServerError` is probably the error type you'll come into contact with most frequently, because it denotes errors that have been propagated down by the server explicitly. This usually means something failed on its end, which could be anything from a *404 Not Found* error to a *500 Internal Server Error*. This variant has two properties: one for its [HTTP status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status), and another for an actual error message, which will be in English (for internationalized apps).

Because of the frequency of this error, it is very common to handle status variants independently. For example, the [error views](:fundamentals/error-views) example in the Perseus repository handles a 404 error independently (because people mistype URLs far more frequently than servers fail), and then handles anything starting with a 4 (client error) separately to anything starting with a 5 (server error).

One important difference between `ServerError` and all the other error types is that it is the only one that occurs *before* the client. This means it is the only error type that will be reflected in what the user sees, before hydration. For example, let's say the user mistyped a URL and ends up at a 404 error page. Because this error 'occurred' on the server (in that that was where Perseus first noticed that something had gone wrong), the HTML that goes to the client will show that error. If, however, something like a hydration error had occurred (which would *not* fall under `ServerError`), the server might have handled everything fine, and the user may be able to still see all the content they want, it just won't be interactive. In these cases, it's better for user experience not to completely delete all the content that has been served statically, just because it isn't interactive. In these cases, Perseus will display a popup error.

### `FetchError`

Fetch errors are fairly self-explanatory, and occur when there has been a failure in communicating with the server. There are a few sub-variants of this, governed by the [`FetchError`](=errors/enum.FetchError@perseus) type.

You may notice in the API documentation that there's a `FetchError::NotOk` variant, which you might expect to fire when the user hits a 404 in what's called a *subsequent load* (i.e. any load of another page in the app where the user has come from somewhere else in the app, rather than an outside website, like a search engine). However, Perseus actually performs route matching on the client-side, before it makes any requests, which is why broken links in Perseus will not lead to any network requests. For subsequent loads, anything other than a *200 OK* response indicates a server error.

### `PluginError`

Plugin errors are exactly what it says on the tin: any error that a plugin can produce. Because all plugin functions are implicitly fallible, any plugin could, at any time, return any error. Since there are a number of opportunities for plugins to run just before a Perseus app starts, they can produce errors, which will often be critical (if they occur before your app starts, Perseus will have to perform a full abort).

### `ThawError`

Thaw errors only apply to you if you're using the [state freezing](:state/freezing-thawing) system, and they usually arise from corrupted state. Very importantly, thaw errors are *sleeper errors*, meaning that you might thaw some frozen state that has an error in it, and then that error might only become apparent much later. This is because Perseus' thawing mechanism is gradual: state is only deserialized when a page needs it. So, if only one page has invalid state, the thaw error will occur much later.

However, in the vast majority of cases of corruption, the whole frozen app will be garbled, and Perseus will detect this error and return it immediately, avoiding sleepers.

### `PlatformError`

Platform errors are currently not really used in Perseus, but, in future, they will be used to make the render backend of Perseus generic so that it could be used beyond the browser. If you're running in a browser though, and you get this error, you can be fairly confident of a critical failure, and you shouldn't expect your error message to even display. An example of a cause for this kind of error would be the `window` object not being defined. To see examples of this error, try running a Perseus app in NodeJS, and see what happens...

### `PreloadError`

Preload errors occur when you try to preload something that's invalid, and they will nearly always be the result of failures in your own code (usually mistyping a template name or the like).

### `InvariantError`

Invariant errors are the most insidious kind of error in Perseus: they arise from internal invariants not being upheld within Perseus. For example, all Perseus apps are expected to define, within the document metadata, a global state. For apps that don't have a global state, they should explicitly declare the fact that they don't have one. However, if a hypothetical overzealous minifcation system decided to strip out the empty global state declaration, deciding it was useless, Perseus would be unable to function. The reason these sorts of things don't resolve to panics is that, unlike most Rust programs, where invariants are simply logical properties that we can't prove to the compiler, in Perseus there's a divide between the client and the server: the network. That means that we might be 100% certain that all invariants are upheld on the server-side, but, by the time we get to the client-side, we might be looking at a completely different file type.

In short, these errors are rare, but catastrophic, and usually cannot be recovered from.

However, there are some cases in which these errors *might* be caused by your code. The most obvious is if you try to fetch the wrong global state type. Let's say you registered `MyGlobalState`, but then you try to get `MyPageState` by accident. Because Perseus uses downcasting to manage state, this would lead to a runtime invariant error. However, in this case, it would be perfectly reasonable for a confused user to go to a different page instead, so we don't panic and fail the whole system.

### `Panic`

Finally, we come to panics, which Perseus takes a unique approach to. Rather than having you set a panic handler, Perseus does it for you, because there are several additional things it needs to do. First, it will print messages to the console telling the user what has happened, just in case everything goes pear-shaped after that (remember, we might be panicking because the `window` doesn't exist, in which case we sure aren't going to be able to display error messages). Then, it will display a popup error that you set the contents of through this handler. Generally, a panic handler should be quite apologetic, because there's literally no way to proceed from here, and the app is practically guaranteed to fail completely. You can also set custom panic handling logic, which will be executed once the message has been displayed, and this is usually a good time to do things like report the error to a crash analytics server. (Note that this is distinct from the `crash` plugin opportunity, which would occur in the case of a critical startup invariant error.)

If you're coming from non-web Rust, you might be thinking you can just set a `catch_unwind` and restart your app nicely, but Wasm uses `panic = "abort"` by default, meaning such handlers are meaningless. Also, while you could re-instantiate Perseus manually, it's not recommended at all, since panics usually indicate that something has gone critically wrong, and that's likely to repeat itself. In general, you should ask the user to restart the app manually by reloading the page, which should fix most spontaneous errors.

## Error positioning

One very unique part of Perseus' error handling system that's very important is its concept of *error positioning*, i.e. how an error appears. There are are three options for this, which correspond to the variants of the [`ErrorPostion`](=error_views/enum.ErrorPosition@perseus) `enum`. The first is that the error will take up the whole page, the second is that it will take up the whole of a widget (such errors will of course only be triggered by widgets), and the third is that the error will be confined to a popup.

It's important to check what kind of error position you have for two reasons: the first is so that you don't display a full-page view with `100vh` styling all over the place if it ends up being displayed in a tiny popup, and the second is because, in a popup error view, you won't have access to a router. This means that any links placed into a popup error will be handled with the browser's default behavior. While this is fine, it may not be what you expect, so be aware of this.

Perseus decides how to position an error based on some simple rules: if it's an *initial load* where the error occurred (i.e. the user has come to your app from the outside internet, e.g. from a search engine), then check if it's a `ClientError::ServerError`. If so, `Page`, if not, `Popup`. If it's a *subsequent load* (i.e. where the user has gone from one page to another inside your app), then this is delegated to you through the `subsequent_load_determinant_fn` function, which you can set on your [`ErrorViews`](=prelude/struct.ErrorViews@perseus). If you don't set this though, the same rules are applied automatically.

The reason behind these rules is based on user experience. Any error that is not a `ServerError` will have occurred on the client-side, right? So that means the server was fine, which in turn means that the server-side rendering was okay. That means there's prerendered content that's perfectly valid sitting in front of the user, and they can read it. They might not be able to interact with it, but, in nearly all cases, it's better that they can't interact with it than that they can't see it at all. There is nothing more infuriating than going to a news website, seeing the article prerendered in front of you, and then having that lovely content be replaced with an error message.

However, if you have some cases where you know for certain that the error should take up the whole page, because the prerendered content is bad in some way, then you can always style the popup error to be absolutely positioned and take up the whole page.

Popup errors will be rendered in a `<div>` with the HTML `id` `__perseus_popup_error`, which you can use to style it arbitrarily.

## Error context

Because Perseus is, for the millionth time, a very complex system, errors can occur in different places. For example, an error that occurs in a plugin before the app is started will mean that you can't have access to something like a translator, because it literally doesn't exist yet. Such errors are rare, but they can definitely happen, and this is why Perseus provides an [`ErrorContext`](=error_views/enum.ErrorContext@perseus). This has four variants, each corresponding to a different level of interactivity that Perseus has, which you can read about in detail [here](=error_views/enum.ErrorContext@perseus).

One important thing to understand about error contexts is how they interact with internationalized apps. Perseus will always make a best effort to give your error views a translator, to the point that, if the user goes to, say, `/bad-page`, Perseus will first resolve it to a localized version (e.g. `/en-US/bad-page`), and only *then* handle the routing error. However, in exported apps, the situation is very different, because error views can be managed beautifully on the client-side, but 404 errors in particular are handled by exporting the error page to a static file, usually called `404.html`, which your serving infrastructure is responsible for providing. Unfortunately, there are very few providers who support localized error views, and, in such cases, error views will always be non-internationalized. However, Perseus will do all it can to, on the client-side, provide a translator. In some cases, however, this may be simply impossible. If you're using internationalization, it's generally recommended to avoid exporting for this reason.

## Writing `ErrorViews`

When you actually write your error views, it is surprisingly simple. Just call `ErrorViews::new()` and provide a closure that takes four arguments: a Sycamore scope, the `ClientError` that occurred, an `ErrorContext`, and an `ErrorPosition`. Then, match them as you like and return a tuple of two `View`s: the first for the document metadata, and the second for the body of the error. Note that popup errors will have their head views ignored, as will widget errors. (In such cases, you can use `View::empty()` to just produce an empty view.) You can see a full example of using error views [here](https://github.com/framesurge/perseus/tree/main/examples/core/error_views).
