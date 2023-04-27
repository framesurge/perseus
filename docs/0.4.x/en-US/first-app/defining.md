# Defining a Perseus App

Once you've got all your dependencies installed, it's time to create the entrypoint to your Perseus app. In most Rust programs. you'll have a `main.rs` file that contains some `fn main() { .. }` that executes your code, and Perseus is no exception. However, remember that Perseus has two parts: the engine-side and the client-side, so you actually need *two* `main()` functions, one for each. Now, don't put anything in `src/main.rs` just yet, because, as we'll see later, there's actually a much more convenient way of handling all this.

Remember, you can tell Rust to only compile some code on the engine-side by putting `#[cfg(engine)]` over it, and you can use `#[cfg(client)]` to do the same for the browser. So, our code in `main.rs` should logically look something like this:

```rust
#[cfg(engine)]
fn main() {
    // Engine code here
}

#[cfg(client)]
fn main() {
    // Browser code here
}
```

Now, this actually isn't too far off, except that running WebAssembly is a little different than you might think. Currently, there isn't really a good concept of a 'binary' Wasm program, you'll always be coding a library that some JavaScript imports and runs. In the case of Perseus apps, we use a `main.rs` file because it makes more logical sense, since Perseus handles all that nasty JS stuff behind the scenes. From your point of view, you're just writing a normal binary. However, there is something special that the client-side function has to do: it has to return a `Result<(), JsValue>`, where `JsValue` is a special type that represents *stuff* in JS-land. You can use Perseus' [`ClientReturn`](=type.ClientReturn@perseus) type alias for this, but note that Perseus actually *can't* return an error from its invocation: all errors are gracefully handled, even panics (although they will eventually propagate up as an unhandled exception in the calling JS, which is why any panics in Perseus will appear as two messages in your browser console rather than one).

Further, Perseus makes the engine and client code pretty convenient with two features (which are enabled by default): `dflt-engine`, and `client-helpers`. The first of these gives us the [`run_dftl_engine()`](=engine/fn.run_dflt_engine@perseus) function, which takes an [`EngineOperation`](=engine/enum.EngineOperation@perseus) derived from the [`get_op()`](=engine/fn.get_op@perseus) function (which just parses environment variables passed through by the CLI), a function that returns a [`PerseusApp`](=prelude/struct.PerseusAppBase) (which we'll get to), and some function to run your server.

As for the client-side, Perseus provides `run_client()`, which just takes a function that returns a `PerseusApp`.

So what is this `PerseusApp`, you might ask? This `struct` forms the bridge between Perseus' internals, and your own code, because it's how you tell Perseus what your app looks like. In fact, because the vast majority of engine and client `main()` functions are so formulaic, Perseus provides a convenient macro, [`#[perseus::main(..)]`](=attr.main@perseus), which you can use to annotate a *single* `main()` function that returns a `PerseusApp`, and that macro will then do the rest automatically. Most of time, this is what you want, but you can always take a look at [the source code]() of that macro if you want to drill deeper into customizing your app (again, you will probably *never* need to do this, even if you're creating an insanely advanced app).

So, our actual `src/main.rs` file would look something like this (theory over, *now* we start coding):

```rust
{{#include ../../../examples/core/basic/src/main.rs}}
```

First off, we declare a module called `templates`, which will correspond to the `src/templates/` folder, which we'll use to store the code for all our templates. Go ahead and create that folder now, with an empty `mod.rs` file inside. The next thing is to import the Perseus `prelude` module, which just collates everything you'll need to run a Perseus app, which helps to avoid having to manually import a million different things. Most of your Perseus files will begin with `use perseus::prelude::*;`, and then `use sycamore::prelude::*;`

Then we get to that special `main()` function. As you can see, it returns a `PerseusApp`, which takes a generic `G`: this is a special part of Sycamore that lets is say "let this function work with any rendering backend that implements `Html`", because Sycamore can actually go way beyond the web! This generic restricts us to using `SsrNode` (for prerendering), `DomNode` (for rendering to the Document Object Model in the browser), or `HydrateNode` (the same as `DomNode`, but for when we're [hydrating](:fundamentals/hydration)).

You'll also notice that we've provided an argument to the `#[perseus::main(..)]` attribute macro: that's the function that will start up our server! If you want to add things like custom API routes, etc., then you can set this function manually, and then use one of the Perseus server integrations to work with the code you've written (see [this example](https://github.com/framesurge/perseus/tree/main/examples/core/custom_server) for more), but here we're just using the default server from the `perseus-axum` package. If you fancy [Warp](https://github.com/seanmonstar/warp), you can use `perseus-axum`, and [Actix Web](https://github.com/actix/actix-web) fans can use the `perseus-actix-web` package!

## Your `PerseusApp`

Now we get to the fun stuff: actually defining your app! The first step is to invoke `PerseusApp::new()`, which is what you'll nearly always want, unless you're in an environment with very special characteristics (e.g. a serverless function with a read-only filesystem), or if you want to manage your translations in a non-standard way for internationalization. Again, 99% of the time, `PerseusApp::new()` is fine.

The next thing we do is declare our templates, which we'll create in a moment. Generally, in Perseus, you'll have an `src/templates/` folder that contains all your templates, and each template will export a `get_template()` function that you call from here. However, if you're from JS-land, where you might be used to something called *filesystem routing* (in which the nesting of a file implies the route it will be hosted at), Perseus has no such thing. If you want to store the about page at `index.rs` and the index page at `about.rs`, have fun!

The next thing we do is specify some [`ErrorViews`](=error_views/struct.ErrorViews@perseus), which are responsible for doing all the error handling in our app. We'll cover this in more detail in [the error handling section](:first-app/error-handling), but just know for now that Perseus has a very strict error handling system, and, unlike a lot of other frameworks, there is no such thing as an unhandled error in Perseus: *everything* is handled (even panics, though they're a bit special).

Of course, you usually just want to dive straight into your app, so you can leave the `.error_views()` bit out if you like, and Perseus will provide some sensible defaults while you're still in development. However, if you try to deploy your app with those defaults, you'll get errors. 

(Note that you might see `ErrorViews::unlocalized_development_defaults()` hanging around a lot in the examples, which basically tells Perseus to force-use those 'sensible defaults' in production as well. This is very convenient for examples about how to use Perseus, but it's almost certainly a bad idea in your own code, especially if you want your app available in multiple languages!)

With all that explained, it's time to create some pages!
