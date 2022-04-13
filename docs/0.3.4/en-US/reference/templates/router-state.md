# Listening to the Router

Given that Perseus loads new pages without reloading the browser tab, users will have no way to know that their clicking on a link actually did anything, which can be extremely annoying for your users, and may even dissaude them from using your site! Usually, this is circumvented by how quickly Perseus can load a new page, but, if a user happens to be on a particularly slow connection, it could take several seconds.

To avoid this, many modern frameworks support a loading bar at the top of the page to show that something is actully happening. Some sites prefer a more intrusive full page overlay with a loading indicator. No matter what approach you choose, Perseus gets out of your way and lets you build it, by using *router state*. This is a Sycamore `ReadSignal` that you can get access to in your templates and then use to listen for events on the router.

## Usage

This example (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/core/router_state/src/templates/index.rs)) shows using router state to create a simple indicator of the router's current state, though this could easily be extended into a progress bar, loading indicator, or the like.

```rust
{{#include ../../../../examples/core/router_state/src/templates/index.rs}}
```

The first step in using router state is accessing it, which can be done through Sycamore's [context API](https://sycamore-rs.netlify.app/docs/advanced/contexts). Specifically, we access the context of type `perseus::templates::RenderCtx` (which also includes other information), which has a field `router_state`, which contains an instance of `perseus::templates::RouterState`. Then, we can use the `.get_load_state()` method on that to get a `ReadSignal<perseus::templates::RouterLoadState>` (Sycamore-speak for a read-only piece of state). Next, we use Sycamore's `create_memo` to create some derived state (so it will update whenever the router's loading state does) that just turns the router's state into a string to render for the user.

As you can see, there are three mutually exclusive states the router can be in: `Loaded`, `Loading`, and `Server`. The first two of these have an attached `String` that indicates either the name of the template that has been loaded (in the first case) or the name of the template that is about to be loaded (in the second case). In the third state, you shouldn't do anything, because no router actually exists, as the page is being rendered on the server. Note that anything rendered in the `Server` state will be visible for a brief moment in the browser before the page is made interactive, which can cause ugly UI flashes. 

As noted in the comments in this code, if you were to load this page and click the link to the `/about` page (which has the template name `about`), you would momentarily see `Loading about.` before the page loaded. During this time (i.e. when the router is in the `Loading` state), you may want to render some kind of progress bar or overlay to indicate to the user that a new page is being loaded.
