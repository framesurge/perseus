# Reactive State

Since v0.3.3, Perseus added support for *reactive state*. Up until now, all our templates have generated state to create one or more pages, and then they've simply used that state to render some stuff. However, in reality, we'll have much more complex data models that involve user interaction. For example, we might have inputs on a page that might change aspects of the view displayed to the user. This used to be done with Sycamore's reactivity system on its own, but Perseus now provides a mechanism to make the state you provide to templates automatically reactive. That means every single property becomes reactive.

Just annotate a state `struct` with `#[perseus::make_rx(RxName)]`, where `RxName` is the name of the new reactive `struct` (e.g. `IndexState` might become `IndexStateRx`). This macro wraps every single property in your `struct` in a `Signal` and produces a new reactive version that way, implementing `perseus::state::MakeRx` on the original to provide a method `.make_rx()` that can be used to convert from the unreactive version to the reactive one (there's also the reverse through `perseus::state::MakeUnrx`, which is implemented on the new, reactive version). If you have fields on your `struct` that are themselves `struct`s, you'll need to nest that reactivity, which you can do by adding `#[rx::nested("field", FieldRxName)]` just underneath the `#[make_rx(...)]` macro, providing it the name of the field and the type of the reactive version (which you'd generated with `#[make_rx(...)]`). Notably, `#[make_rx(...)]` automatically derives `Serialize`, `Deserialize`, and `Clone` on your `struct` (so don't derive them yourself!).

*Note: Sycamore has a proposal to support fine-grained reactivity like this through observers, which will supersede this when they're released, and they'll make all this even faster! Right now, everything has to be cloned unfortunately.*

Once you've got some reactive versions of your state `struct`s ready, you should generate the unreactive versions as usual, but then set the first argument on your template function to the reactive version. This requires Perseus to convert between the unreactive and reactive versions in the background, which you can enable by changing `#[template(...)]` to `#[template_rx(...)]` and removing the Sycamore `#[component]` annotation (this is added automatically by `#[template_rx(...)]`). Behind the scenes, you've just enabled the world's most powerful state platform, and not only will your state be made reactive for you, it will be added to the *page state store*, a global store that enables Perseus to cache the state of a page. So, if your users start filling out forms on page 1 and then go to page 2, and then come back to page 1, their forms will be just how they left them. (Not sure about you, but it feels to us like it's about time this was the default on the web!)

You may be wondering what the benefits of having a reactive state are though. Well, the intention is this: every possible state your page can be in should be representable in your state. That means that, whenever you'd usually declare a new variable in a `Signal` to handle some state, you can move it into your template's state and handle it there instead, making things cleaner and taking advantage of Perseus' state caching system.

## Example

This can all be a bit hard to imagine, so here's how it looks in practice with a simple state involving a `username` that the user can type in, and then it'll be displayed back to them. You can see the source [here](https://github.com/arctic-hen7/perseus/blob/main/examples/rx_state/src/index.rs). Note that this example also uses [global state](:reference/state/global), which is documented in the next chapter, but you can ignore everything except that first `p` and `input` for now.

```rust
{{#include ../../../../examples/rx_state/src/index.rs}}
```

The only unergonomic thing here is that we have to `.clone()` the `username` so that we can both `bind:value` to it and display it. Note that this will be made unnecessary with Sycamore's new reactive primitives (which will be released soon).

## Accessing Another Page's State

Because every template that uses this pattern will have its state added to a special *page state store*, you can actually access the state of another page quite easily. However, you must be careful doing this, because the other page's state will only be available if it's been loaded by the user. On the server, every page is loaded in its own little silo to prevent corruption, so no other page will ever have been 'loaded'. As for in the browser, you might design an app in which it's only possible to get to a certain page by going through another, but you still can't assume that that page has been loaded, because [state freezing](:reference/state/freezing) can let a user pick up from any page in your app, and such special rendering flows will be shattered.

All that said, you can access another page's state like so (see [here](https://github.com/arctic-hen7/perseus/blob/main/examples/rx_state/src/about.rs)):

```rust
{{#include ../../../../examples/rx_state/src/about.rs}}
```
