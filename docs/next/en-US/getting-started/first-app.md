# Your First App

With a basic app scaffold all set up, it's time to get into the nitty-gritty of building a Perseus app. Somewhat humorously, the hardest part to wrap your head around as a beginner is probably going to be the `Cargo.toml` we're about to set up. It should look something like the following.

```toml
{{#include ../../../examples/comprehensive/tiny/Cargo.toml.example}}
```

<details>
<summary>Excuse me??</summary>

The first section is still pretty straightforward, just defining the name and version of your app's package. The line after that, `edition = "2021"`, tells Rust to use a specific version of itself. There's also a 2018 version and a 2015 version, though Perseus won't work with either of those, as it needs some features only introduced in 2021. That version also includes some comfort features that will make your life easier at times.

Now we'll address the interesting dependencies setup. Essentially, we've got three dependency sections. The reason for this is because Perseus runs partly on a server, and partially in a browser. The former is called the *engine*, which is responsible for building and serving your app. The latter is just called the browser, which is where Perseus makes your app work for your users.

These two environments couldn't really be more different, and, while Perseus tries to minimize the complexities of managing both from your perspective, there are *many* Rust packages that won't run in the browser yet. By having separate dependency sections for each environment, we can decide explicitly which packages we want to be available where.

The first section is the usual one, pulling in dependencies that we want everywhere. Both `perseus` and `sycamore` are needed in the browser and on the server-side, so we put them here. Most of the packages you bring in yourself will go here too. As a general rule of thumb, put stuff here unless you're getting errors or trying to optimize your app (which we have a whole section on).

The second is `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`, which looks scary, but is actually pretty simple when you think about it. It defines the `dependencies` section only on the `target` (operating system) that matches the condition `cfg(not(target_arch = "wasm32"))` --- the target that's not `wasm32`, which is Rust's way of talking about the browser. This section contains engine-only dependencies. In other words, the code that runs in the browser will have no clude these even exist. We put two things in here: `tokio` and `perseus-warp`. The first is an asynchronous runtime that Perseus uses in the background (this means we can do stuff like compile three parts of your app at the same time, which speeds up builds). The second is one of those integration crates we were talking about earlier, with the `dflt-server` feature enabled, which makes it expose an extra function that just makes us a server that we don't have to think about. Unless you're writing custom API routes, this is all you need.

The third section is exactly the same as the previous, just without that `not(...)`, so this one defines dependencies that we use in the browser only. We've put `wasm-bindgen` here, which we could compile on the server, but it would be a little pointless, since Perseus only uses it behind the scenes in making your app work in the browser. (This is needed for a macro that a Perseus macro defines, which is why you have to import it yourself.)

</details>

Next, we can get on with the app's actual code! Head over to `src/main.rs` and put the following inside:

```rust
{{#include ../../../examples/comprehensive/tiny/src/main.rs}}
```

This is your entire first app! Note that most Perseus apps won't actually look like this, we've condensed everything into 17 lines of code for simplicity.

<details>
<summary>So that means something, does it?</summary>

We've started off with some simple imports that we need, which we'll talk about as we get to them. The really important thing here is the `main()` function, which is annotated with the `#[perseus::main()]` *proc macro* (these are nifty things in Rust that let you define something, like a function, and then let the macro modify it). This macro isn't necessary, but it's very good for small apps, because there's actually fair bit of stuff happening behind the scenes here.

We also give that macro an argument, `perseus_integration::dflt_server`. You should change this to whatever integration you're using (we set up `perseus_warp` earlier). Every integration has a feature called `dflt-server` (which we enabled earlier in `Cargo.toml`) that exposes a function called `dflt_server` (notice how the packages use hyphens and the code uses underscores --- this is a Rust convention).

As you might have inferred, the argument we provide to the `#[perseus::main()]` macro is the function it will use to create a server for our app! You can provide something like `dflt_server` here if you don't want to think about that much more, or you can define an expansive API and use that here instead! (Note that there's actually a much better way to do this, which is addressed much later on.)

This function also takes a *generic*, or *type parameter*, called `G`. We use this to return a [`PerseusApp`](=type.PerseusApp@perseus) (which is the construct that contains all the information about our app) that uses `G`. This is essentially saying that we want to return a `PerseusApp` for something that implements the `Html` trait, which we imported earlier. This is Sycamore's way of expressing that this function can either return something designed for the browser, or for the engine. Specifically, the engine uses `SsrNode` (server-side rendering), and the browser uses `DomNode`/`HydrateNode`. Don't worry though, you don't need to understand these distinctions just yet.

The body of the function is where the magic happens: we define a new `PerseusApp` with our one template and some error pages. The template is called `index`, which is a special name that means it will be hosted at the index of our site --- it will be the landing page. The code for that template is a `view! { .. }`, which comes from Sycamore, and it's how we write things that the user can see. If you've used HTML before, this is the Rust version of that. It might look a bit daunting at first, but most people tend to warm to it fairly well after using it a little.

All this `view! { .. }` defines is a `p`, which is equivalent to the HTML `<p></p>`, a paragraph element. This is where we can put text for our site. The contents are the universal phrase, `Hello World!`.

You might be scratching your head about that `cx` though. Understandable. This is the *reactive scope* of the view, which is something complicated that you would need to understand much more about if you were using normal Sycamore. In Perseus, all you really need to know for the basics is that this is a thing that you need to give to every `view! { .. }`, and that your templates always take it as an argument. If you want to know what this actually does, you can read more about it [here](https://sycamore-rs.netlify.app/docs/basics/reactivity).

The last thing to note is the `ErrorPages`, which are an innovation of Perseus that force you to write fallback pages for situations like the user going to a page that doesn't exist (the infamous 404 error). You could leave these out in development, but when you go to production, Perseus will scream at you. The error pages we've defined here are dead simple: we're just using the universal fallback provided to `ErrorPages::new()`, which is used for everything, unless you provide specific error pages for errors like 404, 500, etc. This fallback page is told the URL the error occurred on, the HTTP status code, and the error itself.

</details>

With that all out of the way, add the following to `.gitignore`:

```gitignore
dist/
```

That just tells Git not to pay any attention to the build artifacts that Perseus is about to create. Now run this command:

```sh
perseus serve -w
```

Because this is the first time building your app, Cargo has to pull in a whole lot of stuff behind the scenes, so now would be a good time to fix yourself a beverage. Once it's done, you can see your app at <http://localhost:8080>, and you should be greeted pleasantly by your app! If you want to check out the error pages, go to <http://localhost:8080/blah>, or any other page that doesn't exist.

Now, try updating that `Hello World!` message to be a little more like the first of its kind: `Hello, world!` Once you save the file, the CLI will immediately get to work rebuilding your app, and your browser will reload automatically when it's done!

*Note: if you're finding the build process really slow, or if you're on older hardware, you should try switching to Rust's [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#rustup-and-the-role-of-rust-nightly) for a faster compiler experience.*

Now stop that command with `Ctrl+C` and run `perseus deploy` instead. This will take a very long time, but it will produce a `pkg/` directory that you could put on a real-world server, and it would be completely ready to serve your brand-new app! Because this app is so simple, you could even use `perseus deploy -e` instead to just produce a bunch of flat files that you could host from anywhere without needing a proper server.

All this has just scratched the surface of what's possible with Perseus, and there's so much more to learn! The next big things are about understanding some of the core principles behind Perseus, which should help you to understand why any of what you just did actually worked.
