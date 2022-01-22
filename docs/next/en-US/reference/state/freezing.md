# State Freezing

If you use the reactive and global state systems to their full potential, your entire app can be represented as its state. So what if you could make all that state unreactive again, serialize it to a string, and keep it for later? Well, you'd be able to let your users pick up at the *exact* same place they were when they come back later. Imagine you're in the middle of filling out some forms and then your computer crashes. You boot back up and go to the website you were on. If it's built with Perseus and state freezing occurred just before the crash, you're right back to where you were. Same page, same inputs, same everything.

Specifically, Perseus achieves this by serializing the global state and the page state store, along with the route that the user's currently on. You can invoke this easily by running `.freeze()` on the render context, which you can access with `perseus::get_render_ctx!()`. Best of all, if state hasn't been used yet (e.g. a page ahsn't been visited), it won't be cached, because it doesn't need to be. That also applies to global state, meaning the size of your frozen output is minimized.

## Example

You can easily imperatively instruct your app to freeze itself like so (see [here](https://github.com/arctic-hen7/perseus/tree/main/examples/rx_state/src/index.rs)):

```rust
{{#include ../../../../examples/rx_state/src/index.rs}}
```

## Thawing

Recovering your app's state from a frozen state is called *thawing* in Perseus (basically like hydration for state, but remember that hydration is for views and thawing is for state!), and it's completely automatic. Once you give Perseus a way to get your frozen state, it'll check when your app loads up and thaw as necessary.

*The process of telling Perseus how to get frozen state is still in development, but it'll be ready for v0.3.3!*

TODO
