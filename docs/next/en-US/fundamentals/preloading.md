# Preloading

One superpower of Perseus is its caching system, which takes any pages the user has already been to, figures out the minimum amount of information necessary to restore them without any network requests, and stores that, ensuring that pressing the back button leads to an instant response. Sometimes, however, you want this to work in the other direction too: if you are fairly confident of which page a user will go to next, you can *preload* it to make sure they get the content immediately.

Now, usually you would do preloading through the browser, which will fetch resources intelligently to minimize load times, but, again, Perseus knows better than the browser in a lot of cases. To render a new page, all it needs is the page's state and its document metadata, which actually come from a special internal link (behind `/.perseus/page`). Preloading this through the browser is finicky, and it doesn't allow Perseus to do some pre-parsing to keep things speedy, so Perseus provides its own imperative preloading interface.

There are two ways of using this interface: there's the easy way, and the fine-grained way. The easy way is to use the `.preload()` method on the `Reactor`, which spawns a future for you and panics on errors that you caused (like a misspelled route), while silently failing on errors from the server. Alternately, you could use the `.try_preload()` method, which lets you handle the errors, and forces you to manage the asynchronicity yourself. If you want more control over the error handling (which applies especially if you're preloading a route that you haven't hardcoded), then you should use this method instead.

Here's an example of using preloading:

```rust
{{#include ../../../examples/core/preload/src/templates/index.rs}}
```

(Don't worry about the weird links at the bottom, they're just for showing how preloading works with internationalization.)

When that `.preload()` call is hit, Perseus will continue going with execution, meaning the main thread isn't blocked, while simultaneously loading the preloading route (the `about` page) in the background. This means that, when the user clicks on the link to the about page, they'll see it immediately (and we mean literally instantaneously).

As the comments in the above example mention, however, you can't preload across locales, that would lead to errors. This is because Perseus can only manage one set of translations in memory at once, deliberately so (since translations can be *extremely* heavy).
