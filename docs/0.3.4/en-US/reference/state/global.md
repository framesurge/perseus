# Global State

As you've seen, Perseus has full support for reactive state in templates, but what about state that's not associated with any template? The usual example is something like dark mode, which the user might manually disable. In most JavaScript frameworks, you'd bring in some bloated state management system to handle this, but Perseus has global state built in. To declare it, you create a `GlobalStateCreator`, which will be used to generate some state, and then that'll be made reactive and passed to your templates as their second argument (if they have one, and you'll have to use the `#[template_rx]` macro).

The essence of global state is that you can generate it at build-time (though with something like setting dark mode, you'll probably want to ignore whatever was set at build time until you know the browser's preferences) and access it seamlessly from any template. Just like usual [reactive state](:reference/state/rx), you can make it reactive with `#[make_rx(...)]`, and you essentially get app-wide MVC with just a few lines of code (and no extra dependencies, all this is completely built into Perseus).

<details>
<summary>How would I actually implement dark mode like this?</summary>

Above is a contrived example. In reality, dark mode is set in two ways: the preference that the browser exposes and the user's own manual setting (usually through a toggle switch in the header or similar). You obviously don't want to start in light mode and then flicker to dark mode once you know the user's preference, that would be awful, so it's far better to rely on a class in the HTML (e.g. if `dark` is set on the `<body>`, certain things should be styled in different ways) that you set based on a cookie that you've stored, falling back to the `prefers-color-scheme` if there's no cookie yet. Perseus is designed to load content and then make it interactive, so if you do this cookie-checking in your Wasm ode, you'll be too late to avoid that flicker, which is why it's better to either do this with a separate Wasm bundle, or with a quick bit of JS written directly into your `index.html` file. There are plenty of guides on how to achieve this online.

The role of dark mode comes in in styling that toggle switch mostly, and whenever it changes, you should toggle the `dark` class on the `<body>` or similar and update the global state. YOu could to the class toggling with a `create_effect` that listens for changes in the global state. So, in this case, global state actually isn't crucial, it just makes things cleaner and easier to set up. Unfortunately though, dark mode is irritating with any prerendering because you want to avoid that flicker. (That said, if you don't mind temporarily blinding your 3am users, do whatever you like!)

</details>

## Example

All the following examples are taken from [here](https://github.com/framesurge/perseus/blob/main/examples/global_state).

To being with, you'll need to set up a `GlobalStateCreator`, which will look something like this (it's supposed to be fairly similar to the process of generating state for a `Template`, but it currently only supports build-time state generation):

```rust
{{#include ../../../../examples/core/global_state/src/global_state.rs}}
```

Then, you can tell Perseus about that by adding it to `PerseusApp` like so:

```rust
{{#include ../../../../examples/core/global_state/src/lib.rs}}
```

Finally, you can use it like so (note the second argument to `index_page`):

```rust
{{#include ../../../../examples/core/global_state/src/templates/index.rs}}
```

## Potential Issues

Global state has a quirk that shouldn't be an issue for most, but that can be very helpful to know about if you start to dig into the internals of Perseus. Global state is passed down from the server as a window-level JS variable (as with template state), but it doesn't immediately get deserialized and registered, it's loaded lazily. So, if the user loads fifty templates that don't access global state, your app won't initialize the global state. But, the moment you take it as an argument to a template, it will be set up. This means that, while you can access the global state through the render context (with `perseus::get_render_ctx!()`), you shouldn't do this except in templates that already take the global state as an argument. It may seem tempting to assume that the user has already gone to another page which has set up global state, but no matter how the flow of your app works, you mustn't assume this because of [state freezing](:reference/state/freezing), which can break such flows. Basically, don't access the global state through the render context, you almost never need to and it may be wrong. Trust in `#[template_rx]`.
