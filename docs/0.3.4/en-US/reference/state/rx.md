# Reactive State

In v0.3.4, Perseus added support for *reactive state*, which we talked about a bit in the tutorials at the beginning of the documentation. If you've come from a Perseus version before v0.3.4-rc.1, this system will be quite new to you, as it adds a whole new platform on which templates can interact with their state. Originally, you could generate state, and then it would be done, and the template would receive it as is. Now, that state can be made *reactive* by wrapping all its fields inside `Signal`s, and it will then be added to a global store of page state. The platform this is built on allows a whole new level of state mechanics in Perseus, including [global state](:reference/state/global) and even [hot state reloading](:reference/state/hsr) (a world first to our knowledge)!

In essence, Perseus now provides a way to make your state automatically reactive, which enables some *really* cool new features!

To use this new platform, just annotate a state `struct` with `#[perseus::make_rx(RxName)]`, where `RxName` is the name of the new reactive `struct` (e.g. `IndexState` might become `IndexStateRx`). This macro wraps every single property in your `struct` in a `Signal` and produces a new reactive version that way, implementing `perseus::state::MakeRx` on the original to provide a method `.make_rx()` that can be used to convert from the unreactive version to the reactive one (there's also the reverse through `perseus::state::MakeUnrx`, which is implemented on the new, reactive version). If you have fields on your `struct` that are themselves `struct`s, you'll need to nest that reactivity, which you can do by adding `#[rx::nested("field", FieldRxName)]` just underneath the `#[make_rx(...)]` macro, providing it the name of the field and the type of the reactive version (which you'd generated with `#[make_rx(...)]`). Notably, `#[make_rx(...)]` automatically derives `Serialize`, `Deserialize`, and `Clone` on your `struct` (so don't derive them yourself!).

*Note: Sycamore has a proposal to support fine-grained reactivity like this through observers, which will supersede this when they're released, and they'll make all this even faster! Right now, everything has to be cloned unfortunately.*

Once you've got some reactive versions of your state `struct`s ready, you should generate the unreactive versions as usual in functions like `get_build_state()`, but then set the first argument on your template function to the reactive version (e.g. `IndexStateRx` rather than `IndexState`). This requires Perseus to convert between the unreactive and reactive versions in the background, which you can enable by changing the old `#[template(...)]` (used in the old documentation/tutorials) to `#[template_rx]` and removing the Sycamore `#[component]` annotation (this is added automatically by `#[template_rx]`). Behind the scenes, you've just enabled the world's most powerful state platform, and not only will your state be made reactive for you, it will be added to the *page state store*, a global store that enables Perseus to cache the state of a page. So, if your users start filling out forms on page 1 and then go to page 2, and then come back to page 1, their forms will be just how they left them. (Not sure about you, but it feels to us like it's about time this was the default on the web!)

*Side note: if you think this behavior is horrific, you can still use the old `#[template(...)] macro, and we have no plans to deprecate it. Perseus' original unreactive state system worked very well, and there are still plenty of use cases where you may not want all this newfangled reactive state nonsense (like completely static blogs).*

You may be wondering what the benefits of having a reactive state are. Well, the intention is this: every possible state your page can be in should be representable in your state. That means that, whenever you'd usually declare a new variable in a `Signal` to handle some state, you can move it into your template's state and handle it there instead, making things cleaner and taking advantage of Perseus' state caching system. If your entire app doesn't use any of this though, you can still trivially use the old state platform if you want to.

## Example

This can all be a bit hard to imagine, so here's how it looks in practice with a simple state involving a `username` that the user can type in, and then it'll be displayed back to them. You can see the source [here](https://github.com/arctic-hen7/perseus/blob/main/examples/core/rx_state/src/templates/index.rs).

```rust
{{#include ../../../../examples/core/rx_state/src/templates/index.rs}}
```

The only particularly unergonomic thing here is that we have to `.clone()` the `username` so that we can both `bind:value` to it and display it. Note that this will be made unnecessary with Sycamore's new reactive primitives (which will be released soon).

## Accessing Another Page's State

Because every template that uses this pattern will have its state added to a special *page state store*, you can actually access the state of another page quite easily. However, you must be careful doing this, because the other page's state will only be available if it's been loaded by the user. On the server, every page is loaded in its own little silo to prevent corruption, so no other page will ever have been 'loaded'. As for in the browser, you might design an app in which it's only possible to get to a certain page by going through another, but you still can't assume that that page has been loaded, because [state freezing](:reference/state/freezing) can let a user pick up from any page in your app, and such special rendering flows will be shattered.

All that said, you can access another page's state like so (see [here](https://github.com/arctic-hen7/perseus/blob/main/examples/core/rx_state/src/templates/about.rs)):

```rust
{{#include ../../../../examples/core/rx_state/src/templates/about.rs}}
```
