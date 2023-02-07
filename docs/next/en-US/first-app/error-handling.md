# Error Handling

Now we come to the error handling of our app, which is an important part of Perseus. Basically, you've got to explain to Perseus how it should cope with errors that might occur, not in your code, but in its own. For example, let's say the user's internet connection fails: whose responsibility is that? Well, your code isn't manually fetching the next page, so it will probably be Perseus' problem. However, there's a famous story about the Australian parliament that applies quite nicely here: the chambers of parliament there are color-coded, with the House of Representatives being green and the Senate red. However, all emergency exit signs in Australia must, by law, be green. This would be a bit of an antipattern in the red Senate, so a law was specially passed to allow red exit signs in the Senate only. (Yes, this really happened.)

In the same manner, Perseus doesn't want to produce bright red error messages in Times New Roman if your website is bright orange in Comic Sans, so you're given full control over how to display errors. You provide `View`s to Perseus, and it renders them appropriately. For now though, we'll just do some pretty simple error handling to cover the basics: to learn more about how error handling works, and how advanced apps should handle it, see [this page](:fundamentals/error-views).

First, put the following in `src/error_views.rs`:

```rust
{{#include ../../../examples/core/basic/src/error_views.rs}}
```

This code might look intimidating, but it's actually very basic. All we're doing is defining a function `get_error_views()` that's responsible for generating our [`ErrorViews`](=prelude/struct.ErrorViews@perseus), which is the type that handles errors in Perseus. We provide a closure to `ErrorViews::new()` that takes four arguments: a Sycamore scope, the error itself, an [`ErrorContext`](=error_views/enum.ErrorContext@perseus), and an [`ErrorPosition`](=error_views/enum.ErrorPosition@perseus). Those last two are more complex, and you can read [this page](:fundamentals/error-views) to learn more about them, but the first two are what we'll concentrate on here.

The error type will always be [`ClientError`](=errors/enum.ClientError@perseus), which has a number of variants for all the different kinds of errors that can occur in Perseus. For now, all you need to know is that the main three are: `ClientError::ServerError`, which is used for errors that the server picked up on (e.g. a *404 Not Found*); `ClientError::Panic`, which is called just before the app terminates due to a panic; and `ClientError::FetchError`, which indicates either an internal server error or a failed network connection (usually the latter). There are several more variants, but we handle those here with a wildcard, labelling them all internal errors. With those variants explained, things are pretty self-explanatory, except perhaps for the fact that we return a tuple of two `Views`. The first one is for the document `<head>`, and the second is for the body.

The other thing to keep in mind with error views in Perseus is that they won't always take up the whole page (and this is what `ErrorPosition` is for telling you): sometimes the content can be prerendered fine, but the client can't be initialized for whatever reason, so the user can still see content, it's just not interactive. Because it would be a bit pointless to replace perfectly good, albeit uninteractive, content with an error message, Perseus renders a less intrusive popup error, which you can style with the `#__perseus_popup_error` CSS selector. In popup errors, whatever head you render for the error will be ignored, and the original head will be kept (because the page is still perfectly good, just, again, uninteractive).

<details>

<summary>What does uninteractive actually mean?</summary>

Great question! You can learn more about this in [the section on hydration](:fundamentals/hydration), but it basically means that the user can see the content, because it was *prerendered* on the server-side, but they can't interact with it: e.g. if they press a button, it won't do anything. Clicking links will still work, but they'll be handled by the browser, not by Perseus.

</details>

Finally, we handle different types of `ClientError::ServerError`s differently by their [HTTP status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status), which is the language HTTP (the protocol used for communicating between clients and servers) uses to describe errors. Anything starting with a 4 is a client error, and anything starting with a 5 is a server error (1 is informational, 2 is ok, and 3 indicates a redirect; you won't need to handle those). We also separately handle 404, just because it's so common.

With error handling done, it's about time to run this app!
