# `PerseusApp`

The core of Perseus is how it interacts with the code in `.perseus/`, which defines the engine that actually runs your app. Of course, to perform this interaction, you need to be able to tell the engine details about your app, like the templates you've written, the error pages you want to use, etc. All this is done through the `PerseusApp` `struct`, which acts as a bridge between your code and the engine, so this is essentially the core of Perseus, from the perspective of building apps with it.

The way you define `PerseusApp` in any Perseus app is by creating a function that returns an instance of it, with a type parameter (usually called `G`) of type `Html`, which gives Perseus the flexibility to render your app on both the server-side (as it needs to do for prerendering) and in a browser. You need to export this from the root of your app (`lib.rs`), and it's conventional to define it there too as a function called `main` or something similar. Notably, what you call this function is completely irrelevant, provided it has the `#[perseus::main]` attribute macro annotating it, which automatically tells Perseus to use it as your main function.

<details>
<summary>What does that attribute macro do?</summary>

Currently, `#[perseus::main]` just wraps your function in another one with the name `__perseus_entrypoint`, but this behavior could change at any time, so using this macro isn't optional! For example, in future it might modify your code in some crucial way, and such a modification to the macro would be considered a non-breaking change, which means your code could break in production. To be safe, use the macro (or pin Perseus to a specific minor version if you *really* hate it).

</details>

The smallest this can reasonably get is a fully self-contained app (taken from [here](https://github.com/framesurge/perseus/tree/main/examples/comprehensive/tiny/src/lib.rs)):

```rust
{{#include ../../../examples/comprehensive/tiny/src/lib.rs}}
```

In a more complex app though, this macro still remains very manageable (taken from [here](https://github.com/framesurge/perseus/tree/main/examples/core/state_generation/src/lib.rs)):

```rust
{{#include ../../../examples/core/state_generation/src/lib.rs}}
```

Note that, in theory, you could just define your entire app with `PerseusApp::new()`, and this would work, but you would have no pages and no functionality whatsoever. This might be a good way of benchmarking a server's performance with Perseus, perhaps??

## Configuration

To learn about all the functions this macro supports, see [here](https://docs.rs/perseus/latest/perseus/struct.PerseusApp.html).

Again, `PerseusApp` is just a bridge, so all the features you can use through it are documented elsewhere on this site.
