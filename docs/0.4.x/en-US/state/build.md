# Build-time state

The most commonly used part of the Perseus state platform is the *build-time state* system, which generates state when you run `perseus build` (or one of the other commands that invokes it, like `serve`, `export`, etc.). There are two main parts to the build state system: *build paths*, and *build state* itself.

## Build state

The fundamental idea of build state is very simple: you define an asynchronous function that does whatever it wants, and eventually comes back with some state, and then Perseus runs that function during the build process, and saves that state to an internal file (in the magical depths of `dist/`) for later use. This also allows Perseus to prerender any pages using build state to HTML as soon as their state is ready, meaning they can be served almost instantly when they're requested.

Using build state is very simple, here's an example.

```rust
{{#include ../../../examples/core/state_generation/src/templates/build_state.rs}}
```

The main features of this file are the definition of a state type (see [here](:first-app/generating-pages) for a refresher on this) `PageState` that's reactive, and the use of `.build_state_fn()` on the `Template` definition. A function is then provided to this that meets a certain set of criteria: it has to be `async`, it can return an error or be infallible, and it has to return something that makes sense as a state type (i.e. something that you've derived either `ReactiveState` or `UnreactiveState` on).

Now let's drill into what's happening in that `get_build_state` function. Obviously the logic is very simple, it's just returning a static string, but this could be entirely arbtirary. You could fetch from a database here, you could perform complex calculations, heck, you could deploy an army of carrier pigeons and wait for sensor events that tell you when a certain portion have returned if you really want!

The important thing to understand about build state more generally is the signature of this function. It's `async`, as explained earlier, and has access to a [Tokio](https://tokio.rs) 1.x runtime. It takes a single argument (unused here) of the type [`StateGeneratorInfo`](=prelude/struct.StateGeneratorInfo@perseus), which is actually extremely simple, it's just an organizational type. All it stores is the path to the page being built (e.g. if you're building state for `/post/foo` in the `post` template, this would be `foo`), the locale being built for, and any [helper state](:state/helper) you might have defined (which is what the generic is for). Here, we haven't defined any helper state, so the generic is set to the unit type `()`.

### Error handling

As mentioned, functions that generate build state can either be *infallible* (meaning they can't return errors, and they just return the state type), or *fallible* (meaning they return a `Result`). As much as possible, you should avoid `panic!`ing in any state generation functions, especially in apps using features like revalidation or incremental generation, since, if any of those panics occur on the client-side, the server will have to manage panicking threads. This is fine, and your app will be unaffected for other users, but it's terrible practice and will lead to the user whose query caused the panic waiting forever for a page to load that never will. Not ideal! Instead, try to gracefully return errors wherever you can, so that Perseus can convert those into nice [error views](:fundamentals/error-views).

But, you might be thinking, doesn't this function only run at build-time? Well, that's the reason for that [`BlamedError`](=prelude/struct.BlamedError@perseus) type wrapping our error type here. Now, you can return any error from any Perseus state generation function, as long as it implements `std::error::Error`, but some of them will require you to wrap it in this `BlamedError<E>` type, where `E` is your error. This is a special type that annotates your error with an [`ErrorBlame`](=prelude/enum.ErrorBlame@perseus), which says who was responsible for the error: either the client, or the server.

But, again, doesn't this function only run at build-time? How can the client possibly be responsible for any failure there? They can't be, of course, but, if you're using a feature like [revalidation](:state/revalidation), where build-time state is updated at request-time, it is entirely possible that this function will be executed at request-time, and a user could be responsible for any errors then. In this example, that can't happen, but Perseus doesn't know that.

Conveniently, `BlamedError` has a number of helpful conversions available to it that allow you to automatically convert from your error type into it. For instance, if you were to return some error `err`, you could do so with `return Err(err.into())`, or `result?` (since the `?` operator automatically attempts a conversion). Such automatic conversions will implicitly blame the server for the error (which is usually what you want). The one time this can become annoying is when you have a complex error type with multiple subtypes (e.g. `MyError` has variants for `MyFooError` and `MyBarError`). Even if you've implemented `From<MyFooError> for MyError`, you still can't use `?` as usual, because there are *two* conversions that need to take place: the one into `MyError`, and then another into `BlamedError<MyError>`. Until Rust supports custom implementations of `?` (which will alleviate this problem entirely), the best you can do is `my_foo_result.map_err(MyError::from)?`, which will handle both conversions relatively briefly.

This same error handling behavior applies for the vast majority of Perseus state generation functions.

## Build paths

Build paths are a very simple strategy that allows you to return a list of pages that should be rendered within your template. For example, if you return a list containing `foo`, `bar`, and `baz` for the `test` template, you'll get pages `/test/foo`, `/test/bar`, and `/test/baz`. If you also returned `foo/bar/baz`, you'd get another page `/test/foo/bar/baz`, which is perfectly acceptable.

<details>

<summary>Can two templates have conflicting paths?</summary>

No, because templates can only render paths within their own path. However, it's possible to have a situation where you have, say, one template called `foo` that renders a page `bar`, leading to `/foo/bar`, and another template actually named `foo/bar` (since templates can be at lower paths). This would be a problem, and only one page would be resolved (depending on the order of the build process, which is parallelized, this could be either). If the `foo` template is using incremental generation though, don't worry, since exact paths are always given priority over incremental ones. 99.9% of the time, you will not have to worry about routing conflicts like this.

</details>

Here's an example of using build paths:

```rust
{{#include ../../../examples/core/state_generation/src/templates/build_paths.rs}}
```

This may look slightly more intimidating than the previous example, but all that's been added is a new `get_build_paths` function, which is basically identical in terms of error handling, etc. to `get_build_state`, except it will *never* be run at request-time, so it can return a normal error, which will be blamed on the server. It also returns an organizational type [`BuildPaths`](=prelude/struct.BuildPaths@perseus), which has two parameters: the first is a list of paths, and the second is an `extra` property, which relates to [helper build state](:state/helper). Since we're not using it here, we just use `()`, converted into what Perseus expects with `.into()` (you can read more about this [here](:state/helper)).

The main thing about build paths is that list, which contains an empty string (which will, since the name of this template has been set to `build_paths`, for demonstration purposes, render at `/build_paths`, since the empty string indicates the page at the root of the template), and several other paths. One of these is nested, showing that that's possible, and another contains a space, which Perseus will automatically handle URL encoding/decoding of (since browsers don't like special characters like those, and use a thing called [percent encoding](https://developer.mozilla.org/en-US/docs/Glossary/percent-encoding) to work around them).

*Note: support for other special characters, especially non-ASCII characters, is currently untested in Perseus. If you have problems with this, please report them, or let us know if things work as expected for you!*

Note that here we've used `std::convert::Infallible` as the error type to show that you can return errors, but, since this function really is infallible, we could have just returned `BuildPaths` directly. As with other state generation functions, the build paths function is asynchronous, meaning you can do more complex work without disrupting the rest of the build process.

Remember that any paths you don't generate under the template will resolve to *404 Not Found* errors (e.g. `/build_paths/tests`, here, because it's not in that list). Also, if you don't explicitly specify the empty string here, the template will have no root page.

Finally, notice how `get_build_paths` works here: we're using the `path` property of [`StateGeneratorInfo`](=prelude/struct.StateGeneratorInfo@perseus) to produce a `PageState` that is path-dependent. 
