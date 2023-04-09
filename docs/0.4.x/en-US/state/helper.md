# Helper state

For a long time, the Perseus state platform consisted only of what you've read about so far, but there was a problem with this, one that's quite subtle. Let's say you have a blog where posts can be organized into series, and then there's a `series` template that lists each series in order. How would you write the state generation code for the series template? (Assuming it can all be done at build-time, for simplicity.)

Well, you might think, we can iterate over all the blog posts in the build paths logic, and read their series metadata properties to collate a list of all the series, so that's the first part done. (Right on!) And then for the actual build state generation, you'd just need to find all the blog posts that are a part of the given series. But how can we do that?

The best way is to iterate through all the blog posts again, which means, since the builds for all the series pages are done in parallel, if you have ten series, you're iterating through all those posts and reading every single one of them *eleven* times (+1 for the build paths logic). This is totally unreasonable, especially if your blog posts are on a server, rather than a local directory, and this could massively slow down build times. What would be good is if we could somehow only iterate through everything once, and just store a map of which posts are in what series that we can share through all the actual build state generations.

Because the only solutions to this problem are ugly workarounds, we decided to implement this as a first-class feature in Perseus: helper state! This is what that generic on [`StateGeneratorInfo`](=prelude/struct.StateGeneratorInfo@perseus) is all about: it denotes the type of your helper state.

Importantly, helper state isn't really like any of the other state systems in Perseus, because it's not available to the views you create, and it never gets to the client: it's just a helper for the rest of your state generation. Internally, Perseus calls this *extra state*, but helper state has come to be its name outside the codebase.

Here's an example of using helper/extra state:

```rust
{{#include ../../../examples/core/helper_build_state/src/templates/index.rs}}
```

Here, we've defined a special extra type called `HelperState` (but it can be called anything you like), and then we've used that for the `extra` parameter of [`BuildPaths`](=prelude/struct.BuildPaths@perseus). This allows the build paths function, which is executed once, to pass on useful information to the build state systems, potentially reducing the volume of computations that need to be performed. Note the use of `.into()` on the `HelperState` to convert it into a `Box`ed form that Perseus is more comfortable with internally. In fact, it's only when we call `.get_extra()` on the [`StateGeneratorInfo`](=prelude/struct.StateGeneratorinfo@perseus) provided to the `get_build_state` function that Perseus performs the conversions necessary to retrieve our helper state type (which means specifying the generic incorrectly can lead to panics at build-time, but these would be caught before your app went live, don't worry). Finally, the `.0` is just used to access the `String` inside `HelperState`.

That's pretty much all there is to helper state, and it's available at all stages of the state generation process, right up to [request-time state](:state/request). If there are any parts of request-time state that you can do at build-time, this is the best way to do them if you're not using [state amalgamation](:state/amalgamation).
