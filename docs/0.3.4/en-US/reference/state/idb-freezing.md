# Freezing to IndexedDB

One of the most common places to store frozen state is inside the browser, which can be done with Perseus' inbuilt `IdbFrozenStateStore` system, which uses [IndexedDB](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API) to store as many frozen states as you want, allowing you to revert to not just the previous state, but the one before that, the one before that, etc.

To use this system, you'll need to enable the `idb-freezing` feature flag, and then you can use the system as per the below example.

# Example

The following code is taken from [here](https://github.com/framesurge/perseus/tree/main/examples/idb_freezing/src/idb.rs).

```rust
{{#include ../../../../examples/core/idb_freezing/src/templates/index.rs}}
```

This example is very contrived, but it illustrates the fundamentals of freezing and thawing to IndexedDB. You'll need to perform most of this logic in a `wasm_bindgen_futures::spawn_local()`, a function that spawns a future in the browser, because the IndexedDB API is asynchronous (so that costly DB operations don't block the main UI thread). The first button we have in this example has its `on:click` handler set to one of these futures, and it then freezes the state, initializes the database (which will either create it or open it if it already exists), and then calls `.set()` to set the new frozen state (which will remove previously stored frozen states in the background). The rest of the code here is just boilerplate for reporting successes or failures to the user.

Notably, the operations you'll perform through `IdbFrozenStateStore` are all fallible, they can all return an `Err`. These cases should be handled carefully, because there are a myriad number of causes (filesystem errors in the browser, invalid data, etc.). Perseus tries to shield you from these as much as possible, but you should be wary of potentially extremely strange errors when working with IndexedDB (they should be very rare though). If your app experiences an error, it's often worth retrying the operation once to see if it works the second time. If you're having trouble in local development, you should use your browser's developer tools to delete the `perseus` database.

As for thawing, the process is essentially the same, except in reverse, and it should be noted that the `.thaw()` method is fallible, while the `.freeze()` method is not. This is due to the potential issues of accepting a frozen state of unknown origin.

One thing that may seem strange here is that we get the render context outside the click handlers. The reason for this is that the render context is composed almost entirely of `Signal`s and the like, so once you have one instance, it will update. Further, we actually couldn't get the render context in the futures even if we tried, since once we go into the future, we decouple from Sycamore's rendering system, so the context no longer exists as far as it's concerned. We can work around this, but for simplicity it's best to just get the render context at the beginning and use it later.

It's also important to understand that we don't freeze straight away, but only when the user presses the button, since the result of `.freeze()` is an unreactive `String`, which won't update with changes to our app's state.
