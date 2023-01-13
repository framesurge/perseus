# Global state

Like any state management system worth its salt, Perseus also has a global state system, which is managed analogously to templates, through the [`GlobalStateCreator`](=state/struct.GlobalStateCreator@perseus) API. As with templates, you can generate build-time global state, request-time global state, and you can even amalgamate the two! Here's an example of all three working together:

```
{{#include ../../../examples/core/global_state/src/global_state.rs}}
```

Note the definition of a reactive state type `AppState` (which does not have to be `Clone`, unlike page state types), and the lack of `StateGeneratorInfo`: since there are no paths or helper state in the global state system, all you get is a locale (since some apps will need locale-specific global state). As with other state generator functions, these are `async` and can either be fallible or infallible, at your choice. Due to the lack of revalidation or incremental generation support (neither of which are planned) in global state, the build state generator can never be run at request-time, and therefore returns unblamed errors. To learn more about state generator error handling, see [here](:state/build).

Notice also the use of a `get_global_state_creator()` function that returns an instance of `GlobalStateCreator`. This is analogous to the `get_template()` functions you may be used to.

## Using global state

Since it's not provided as an argument to your views, you can access your global state through the [reactor](:fundamentals/reactor), as in this example:

```
{{#include ../../../examples/core/global_state/src/templates/index.rs}}
```

The `.get_global_state::<T>()` method is used, where `T` is the reactive version of the global state type. Providing the wrong type here will lead to a panic, and you can use `.try_get_global_state()` instead if you wish. These functions return a reference to the reactive version of your global state, with the lifetime of the page they were executed in, allowing interpolation.

Note that the global state has its own version of the page state store, and is cached in a reactive fashion, meaning updates to it on one page will be preserved on other pages. This makes the global state extremely helpful for storing things like user volume preferences in a music app, which might be changed from any song page.

## Pitfalls of request-time global state

Absolutely critically, if you are using only request-time global state, and you use the `.get_global_state()` method *anywhere* in your app that uses build state, you will experience panics at build-time. This is because Perseus does not attempt to prevent pages from accessing global state at build-time, even if it won't exist until request-time: policing this is your responsibility. Further, even if you set sensible defaults at build-time and override these at request-time, any pages that only use build state will never see the request-time state until the client-side, which can lead to hydration errors. In general, be cautious when using request-time global state, as improper usage of it abounds, and it can be highly error-prone. If in doubt, avoid it.
