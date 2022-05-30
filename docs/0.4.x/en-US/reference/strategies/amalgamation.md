# State Amalgamation

In the introduction to this section, we mentioned that all these rendering strategies are compatible with one another, though we didn't explain how the two strategies that generate unique properties for a template can possible be compatible. That is, how can you use _build state_ and _request state_ in the same template? To our knowledge, Perseus is the only framework in the world (in any language) that supports using both, and it's made possible by _state amalgamation_, which lets you provide an arbitrary function that can merge conflicting states from these two strategies!

## Usage

Here's an example from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/core/state_generation/src/templates/amalgamation.rs):

```rust
{{#include ../../../../examples/core/state_generation/src/templates/amalgamation.rs}}
```

This example illustrates a very simple amalgamation, taking the states of both strategies to produce a new state that combines the two. Note that this also uses `RenderFnWithCause` as a return type (see the section on the [_build state_](:reference/strategies/build-state) strategy for more information). It will be passed an instance of `States`, which you can learn more about in the [API docs](https://docs.rs/perseus). As usual, serialization of your returned state is done with the `#[perseus::autoserde(amalgamate_states)]` macro, though the components of `States` will **not** be deserialized, and you'll have to do that manually. Note that the next major version of Perseus will deserialize the components of `States` automatically.
