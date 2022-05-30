# State Freezing

If you use the reactive and global state systems to their full potential, your entire app can be represented as its state. So what if you could make all that state unreactive again, serialize it to a string, and keep it for later? Well, you'd be able to let your users pick up at the *exact* same place they were when they come back later. Imagine you're in the middle of filling out some forms and then your computer crashes. You boot back up and go to the website you were on. If it's built with Perseus and state freezing occurred just before the crash, you're right back to where you were. Same page, same inputs, same everything.

Specifically, Perseus achieves this by serializing the global state and the page state store, along with the route that the user's currently on. You can invoke this easily by running `.freeze()` on the render context, which you can access with `perseus::get_render_ctx!()`. Best of all, if state hasn't been used yet (e.g. a page hasn't been visited), it won't be cached, because it doesn't need to be. That also applies to global state, meaning the size of your frozen output is minimized (note that this isn't property-level granular yet, but that *might* be investigated in future).

## Example

You can easily imperatively instruct your app to freeze itself like so (see [here](https://github.com/arctic-hen7/perseus/tree/main/examples/core/freezing_and_thawing/src/templates/index.rs)):

```rust
{{#include ../../../../examples/core/freezing_and_thawing/src/templates/index.rs}}
```

## Thawing

Recovering your app's state from a frozen state is called *thawing* in Perseus (basically like hydration for state, but remember that hydration is for views and thawing is for state!), and it can occur gradually and automatically once you provide Perseus a frozen state to use, which you can do by calling `.thaw()` on the render context (which you can get with `perseus::get_render_ctx!()`). How you store and retrieve frozen state is entirely up to you. For example, you could store the user's last state in a database and then fetch that when the user logs in, or you could store it in IndexedDB and have even logging in be covered by it (if authentication tokens are part of your global state). Note that thawing will also return the user to the page they were on when the state was thawed.

One important thing to understand about thawing though is how Perseus decided what state to use for a template, because there can be up to three options. Every template that accepts state will have generated state that's provided to it from the generation proceses on the server, but there could also be a frozen state and an active state (some state that's already been made reactive). The server-generated state is always the lowest priority, and it will be used if no active or frozen state is available. However, deciding between frozen and active state is more complicated. If only one is available, it will of course be used, but it both are available, the choice is yours. You can represent this choice through the `ThawPrefs` `struct`, which must be provided to a call to `.thaw()` as the second argument. This has two fields, one for page state, and another for global state. For global state, you can set the `global_prefers_frozen` field to `true` if you want to override active global state with a frozen one. For page state, you'll use `PageThawPrefs`, which can be set to `IncludeAll` (all pages will prefer frozen state), `Include(Vec<String>)` (the listed pages will prefer frozen state, all others will prefer active state), or `Exclude(Vec<String>)` (the listed pages will prefer active state, all others will prefer frozen state). There's no `ExcludeAll` option because that would defeat the entire purpose of thawing.

It may at first be tempting to use `IncludeAll`, but this is an important UX decision that you should consider carefully. Using frozen state when active state isn't available is automatic, but preferring frozen state *over* active state translates to something like this: a user does some stuff, then state is thawed, everything they did at the start is gone and replaced with whatever they did in the previous session. This might be entirely reasonable in pages that can only be accessed after thawing is complete, but in pages that are accessible at all times, this could be extremely irritating to your users!

<details>
<summary>Thawing isn't working...</summary>

It may seem sometimes like thawing has completely failed, and this is usually for one of two reasons.

1. You're extracting the state of another page.
2. You're getting the global state without having it as the second argument to your template function. (In other words, you're getting it manually through `perseus::get_render_ctx!().global_state.borrow()`).

In the first case, the reasoning is simple. Statw thawing is a gradual process, so the state for a page won't be thawed until the user actually visits that page. This is why it's much better to use global state for state that needs to be shared between pages, and you should generally avoid extracting state from other pages.

In the second case, the reason is similar. When you get the global state directly in this way, you bypass the thawing process altogether, meaning thawed state won't show up. If you need to access the global state, you should do it by making it the second argument to your template function (as documented [here](:reference/state/global)).

*Note: in a future version of Perseus, thawing logic may be altered so that direct access does become possible, but it's currently not.*

</details>
