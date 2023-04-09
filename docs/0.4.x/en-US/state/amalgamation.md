# State amalgamation

There are quite a few cases when you're using the state generation platform where you might like to generate state at both build-time *and* request-time, and Perseus has several ways of handling this. Generally, the request-time state will just completely override the build-time state, which is a little pointless, since it doesn't have access to the build-time state, and therefore there would really be no point in even using build-time state. However, you can also specify a custom strategy for resolving the two states, which is called *state amalgamation*. To our knowledge, Perseus is currently the only framework in the world that supports this (for some reason, since it's really not that hard to implement).

Like [other state generation functions](:state/build), your state amalgamation function can be either fallible (with a [`BlamedError`](=prelude/struct.BlamedError@perseus)) or infallible, and it has access to a [`StateGeneratorInfo`](=prelude/struct.StateGeneratorInfo@perseus) instance. It's also asynchronous, and returns your state. The difference between it and other functions is that it also takes, as arguments, your build-time and request-time states (it does *not* take the HTTP request, so you'll have to extract any data from this that you want and put it into your request-time state). Here's an example of using it (albeit a rather contrived one):

```rust
{{#include ../../../examples/core/state_generation/src/templates/amalgamation.rs}}
```

Real-world examples of using state amalgamation are difficult to find, because no other framework supports this feature, although there have been requests for it to be supported in some very niche cases in the past. Since it involves very little code from Perseus, it is provided for those niche cases, and for cases where it would be generally useful as an alternative solution to a problem.

One particular case that can be useful is having an `enum` state with variants for build-time, request-time, and post-amalgamation. The build-time state can be used for anything that can be done that early, and then the request-time state performs authentication, while the amalgamation draws it all together, ensuring that only the necessary stuff is actually sent to the client. Unfortunately, doing this would require a manual implementation of the traits that `ReactiveState` would normally implement, since it doesn't yet support `enum`s (but it will in a future version).
