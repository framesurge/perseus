# Freezing and thawing

One of the most unique, and most powerful features of the Perseus state platform is its system of *state freezing*. Imagine this: all your reactive (and unreactive) state types implement `Serialize` and `Deserialize`, right? We also have an internal cache of them that monitors all the updates that occur to the states of the last *N* pages a user has visited (by default, *N* is 25). So what if we iterated through all of those, serialized them to a string, and stored that? It would be a fullly stringified representation of the state of the app. And, if you build your app with all reactive components built into your state type (i.e. not using rogue `Signal`s that aren't a part of your page state), then you could restore your entire app perfectly from this string.

Since v0.3.5, that has been built into Perseus.

In fact, it's this feature that powers one of Perseus' most powerful development features: *hot state reloading* (HSR). In JS-land, there's *hot module reloading*, where the bundlers intelligently only swaps out the tiny little chunks of JS needed to update your app, allowing you, the developer, to stay in the same place while you're developing. If you're four states deep into debugging a login form, not having to be thrown back to the beginning every time you reposition a button is something you will *really* appreciate! However, this seems impossible in Wasm, because we don't have chunking yet. Perseus changes this by implementing state freezing/thawing at the framework level, allowing Perseus to automatically freeze your entire app's state, save it into the browser, reload the page to get the new code, and then instantly thaw your app, meaning the only times you will get thrown back to the beginning of that login form are when you change your app's data model.

## Understanding state freezing

State freezing can be slightly difficult to understand at an implementation level, because of the complexity of the internals of Perseus. Generally though, you can think of it like this: all your pages are literally having their states serialized to `String`s, and then those are all being combined with your global state (if you have one), and some other details, like the current route. This can then all be used by Perseus to *thaw* that string by deserializing everything and reconstituting it.

## The process of thawing

Critically, Perseus **does not** restore your state all at once, and this can be difficult to wrap your head around. The problem is that Perseus doesn't record any of your state types internally: it gets them from your view functions, and that means it can't thaw all your state at once, because it doesn't know what to deserialize your states into. For all it knows, your page states might by `u8`s! So, Perseus stores all the frozen state internally, and, each time the user goes to a new page, it checks if there's some frozen state known for that page, deserializing it if it can. If this fails, a popup error will be emitted, which can usually be solved by reloading the page to dispose of the corrupted frozen state. (Note that most accidental corruptions would break the very JSON structure of the thing, and would be caught immediately.) This also goes for the global state (frozen state is checked on the first `.get_global_state()` call to [`Reactor`](=prelude/struct.Reactor@perseus)).

Note that Perseus will also automatically navigate back to the route the user was on when their state was thawed.

You can control many aspects of thawing, including whether frozen state or new state is preferred, on a page-by-page basis using the [`ThawPrefs`](=state/struct.ThawPrefs@perseus), which you can read about at that link.

## Example

Here's a more complex example of using state freezing. There are two inputs, one for the global state, and one for the page state, which will be used to reactively set them, and then a button that freezes the whole app (using the `reactor.freeze()` method, which really is all you need to do!). For demonstration purposes, that's then synchronized to an input that takes in state that can be used to thaw the app, which is a slightly more complex (and fallible) process. Note the use of `#[cfg(client)]`, since state freezing/thawing can only take place on the client-side.

```rust
{{#include ../../../examples/core/freezing_and_thawing/src/templates/index.rs}}
```

## Storing frozen state

Freezing your app's state can be extremely powerful, and it's often very useful to simply store this frozen state in a database, allowing your users to return to exactly where they left off after they log back in, or something similar. However, there is also the option of storing the state in the browser itself through [IndexedDB], a database that can be used to store complex objects. Interfacing with IndexedDB is extremely complex in JS, let alone in Wasm (where we have to use `web-sys` bindings), so Perseus uses [`rexie`](https://docs.rs/rexie/latest/rexie) to provide a convenient wrapper when the `idb-freezing` feature flag is enabled. This is managed through the [`IdbFrozenStateStore`](=state/struct.IdbFrozenStateStore@perseus) type, which uses a named database. If you like, you can do this manually: this type is provided as a common convenience, and because it's used internally for HSR.

## Offline state replication

*Coming soon!*
