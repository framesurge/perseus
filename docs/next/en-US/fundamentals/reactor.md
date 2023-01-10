# The reactor

Whenever you're working with Perseus' internals, whether it's to determine the current locale, hook into the router state, preload a page, or forcibly evict a large page from the state store, you'll need to get familiar with the [`Reactor`](=prelude/struct.Reactor@perseus). This is a multi-platform (i.e. available on the engine-side *and* the browser-side) type responsible for creating a unified environment for Perseus renders. Almost everything Perseus does in the background on the browser-side is event-driven (e.g. navigate to page X when the user presses this button, display error Y when this invalid thing is done), and the reactor is the side of all this.

The other thing the reactor does is manage all reactivity in Perseus. See, reactivity, *reactor*? Nifty, eh?

Accessing the reactor is very simple, as it's provided through Sycamore's context system, and it has a method for extracting itself therefrom:

```
Reactor::<G>::from_cx(cx)
```

Note the presence of the `G` type parameter, which is provided because the reactor behaves differently on the engine-side and the client-side. It also needs to know whether or not it's hydrating, because it's responsible for rendering. Note that putting in a type other than `G` here will lead to the `Reactor` not being found *sometimes*, and being found at other times. This can lead to headache-inducing errors that seem to make almost no sense.

It is also important to be aware of the fact that Perseus aligns the `G` parameter with the rendering environment, such that being on the client-side is guaranteed to lead to a `DomNode`/`HydrateNode` (depending on the `hydrate` feature flag), and being on the engine-side is guaranteed to lead to an `SsrNode`. Trying to manually violate this pattern, say by trying to render a page to a string on the client-side through Perseus, will lead to panics, which Perseus uses to prevent undefined behavior. If you want to do server-side rendering in the user's browser, you should do it directly through Sycamore's functions, and you *must not* use capsules, because those will *definitely* panic if they're rendered in weird circumstances like those.
