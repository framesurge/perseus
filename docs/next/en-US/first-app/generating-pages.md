# Generating Pages

Before we start generating pages for your app, it's important to understand how Perseus handles pages, because it's quite different to other frameworks. Let's use an example: imagine you have a personal website with a blog, and there are some posts that should each be hosted at `/post/<post-slug>`, where 'slug' refers to a machine-friendly version (i.e. no spaces, all lowercase, etc.). You might be used to declaring some kind of `enum Router`, or creating a `post/` folder in your code, but Perseus goes a different route (pun intended).

We use *templates* to generate *pages*.

Re-read that a couple of times, because it's the core idea that underlies Perseus' design. We use *templates* to generate *pages*.

A template is like a page with some holes. A `post` template might have all the styling, the header, the footer, etc., with a gap for the title, a gap for the content, maybe a gap for some tags, etc. Think of them like stencils. You can then generate *state*, which is a term Perseus uses for data that can fill in a template. Generally speaking, if you list the gaps in your template, and make a `struct` with a field for each of those gaps, that's what your state should look like. So, if we were making a blog, we would have a struct that perhaps looks something like this:

```rust
struct PostState {
    title: String,
    content: String,
    tags: Vec<String>,
}
```

When you plug *state* into a matching *template*, you can create a *page*. **Template + state = page**, in other words. Perseus has some convenient ways to do this: you'll usually declare an `async` function that produces a `Vec` of all the paths you want to generate (e.g. for a blog it might enumerate all the `.md` files in a directory), and then another function that goes through each of those paths one-by-one and generates their state (e.g. fora blog it might read the contents of each file). Once you've generated the state, Perseus does the boring work of fitting it all together, and it prerenders your pages to HTML at build-time so they can be served to clients as quickly as possible.

## A simple greeting

But for this tutorial, we're just getting started, so we'll use *build-time state* to produce a greeting that we can fit into our index template. In this case, our template will just produce one page: the landing page.

To do this, first add `pub mod index;` to your `src/templates/mod.rs` file, and then out the following in `src/templates/index.rs`:

```rust
{{#include ../../../examples/core/basic/src/templates/index.rs}}
```

This is much more complex than you might have been expecting! First, we import those `prelude` modules, as usual, and we also grab `serde`'s `Serialize` and `Deserialize` derive macros, because, when you think about it, Perseus needs to send whatever state you generate over the network to a user's browser, so it has to be able to turn your state into a string and back again.

The first major part of this file is the state definition: here we're creating a `struct IndexPageState` with one field `greeting: String`, and we've annotated that with what look like some pretty scary macros!

In fact, though, they're actually pretty simple. First, we want `Serialize` and `Deserialize`, as explained earlier, and then we want our state to be `Clone`able, mostly because Perseus sometimes needs to do this internally (but it doesn't happen regularly by any means). We also derive `ReactiveState`, which is a special Perseus macro that you can read more about [here](=derive.ReactiveState@perseus). Basically, it wraps all your fields in [`RcSignal`](=prelude/struct.RcSignal@sycamore)s, which make them *reactive*. Internally, Perseus will maintain a copy of this reactive state, so any changes made to it will be automatically reflected in the core, meaning you don't have to rely on the browser to keep things like forms filled in the way they were when the user last visited a particular page! (Of course, though, you can turn this off if you don't like it.)

One of the things about `ReactiveState` is that it needs to create a whole new `struct` for the reactive version of your code, and it needs a name for that: this is what the `#[rx(alias = "IndexPageStateRx")]` part does: it tells Perseus to call that thing `IndexPageStateRx` (or, more accurately, to create a type alias to it with that name). 

Now we get to the view function, called `index_page`. Like our `main()` function, this takes a generic `G: Html`, for the same purpose, because Perseus will prerender it on the engine-side, and then hydrate it on the client-side. This function takes two arguments: the first is a Sycamore [`Scope`](prelude/struct.Scope@sycamore), and then second is a reference to our reactive state type. For those familiar with Sycamore, you might be wondering how the heck this works: shouldn't that reference have the same lifetime as the scope? Yes, it should! And that's what [`#[auto_scope]`](=attr.auto_scope@perseus) is for. In reality, the lifetimes on this function are much more complex, but, because you basically don't need to care about them 95% of the time, you can elide them with this macro for convenience. If you dislike macros though, you can write it out manually yourself (see [the macro documentation](=attr.auto_scope@perseus) for how to do this).

Returning to our view function, it returns a `View<G>`, somewhat unsurprisingly given its name, which is just Sycamore's way of representing some *stuff* that can be rendered for the user to see. To create this *stuff*, we use the `view!` macro, which takes a special syntax for creating HTML. First is a `p` element, which is HTML for *paragraph*, and, inside, we use parentheses within our curly brackets to tell Sycamore that we're going to interpolate a variable of some kind. That variable is `state.greeting`, but remember that we've got the reactive version here, so `state.greeting` is an `RcSignal`, which means we need to `.get()` it to actually get the value. Similarly, we could `.set()` it if we wanted to change it, and any `.get()`s would update automatically!

The other element is just an `a` (HTML for *anchor*, which is HTML-specification-writer-speak for link). There's actually something quite important about this though: the link's `href`. Perseus is quite special when it comes to `href`s, because it throws a `<base />` element into the metadata of all your pages that declares where the root is. This means Perseus can be deployed easily to `framesurge.sh`, `framesurge.sh/perseus`, or even `framesurge.sh/some/arbitrarily/nested/url`, and it will work fine. The tradeoff of this is that, unlike what you might initially expect, you can't just omit the `/` to get something relative to the current page. If you need to know what path you're currently at (which you'll find, with Perseus' template-based model, is quite rare), you can use `Reactor::<G>::from_cx(cx).router_state.get_path()`. Again, you probably won't need this.

Now, we get to the `head()` function, which, you might notice, is suspiciously similar to the `index_page` function, except that it takes the *unreactive* version of your state, and that it always has its render backend set to `SsrNode`. Why is that? Because this is responsible for rendering the `<head>` of your pages, which is like the metadata. None of this is visible to users, so it isn't reactive, so Perseus just renders it ahead of time to make things easier. Usually, you'll use this `view!` for things like `title`s, CSS imports, etc. If you want metadata that applies to every page in your app, rather than just every page in one template, check out [the index view example](https://github.com/framesurge/perseus/tree/main/examples/core/index_view).

Then we have the `get_build_state` function, which is responsible for generating the state that will fill out our template. Sure, it's a little pointless here, but this function can do *literally anything*. It can read files, it can request from APIs, it can index databases, *anything*. And, of course, it's `async`, and Perseus does everything in parallel wherever possible, so you won't slow down the rest of your build. (But, if you do, we've got [a fix for that](=utils/fn.cache_res@perseus).) This function returns an instance of the unreactive version of your state: if you're feeling a bit confused about where it's supposed to be reactive and where it's supposed to be unreactive, we understand! But, there's actually only one place where your state will ever be reactive: your view function (e.g. `index_page`). Everywhere else, it's unreactive.

That build state function takes a type called [`StateGeneratorInfo`](=prelude/struct.StateGeneratorInfo@perseus), which contains three things: the path that we're generating state for within the template, the locale we're generating state for, and any [helper state](:state/helper) you might have created. Here though, we don't actually need any of it.

Those [`#[engine_only_fn]`](=prelude/attr.engine_only_fn@perseus) macros are very simple too, and, if you don't like macros, you can easily replicate their functionality manually. All they do is wrap the function they annotate in `#[cfg(engine)]`, and then create a function of the same name, but that takes no arguments, returns nothing, and does nothing, and annotate that with `#[cfg(client)]`. Basically, these will make sure that your function still *exists* on the client-side, but that it's just a dummy. This is very useful for `.build_state_fn()`, which we'll get to, which expects a fully featured `async` function on the engine-side, and a dummy on the client-side. This strategy keeps your bundle sizes low, and your pages fast, while keeping the target-gating to a minimum.

You might be wondering about error handling on the engine-side: surely, if you're connecting to a database, you would need to return errors sometimes? What if the server building your app loses its internet connection? Well, you actually can return errors. In fact, try changing the return types of both `head` and `get_build_state` to return `Result<T, std::io::Error>`, where `T` was what they returned before. If you then wrap what they're returning in `Ok(..)`, there will be no errors. Perseus is designed to accept either fallible or infallible functions, and the error type can be whatever you like, as long as it implements `std::error::Error`. For `get_build_state` though, it's actually a tiny bit more complicated than this, as you'll need to wrap your error type in something called [`BlamedError`](prelude/struct.BlamedError@perseus), which you can learn more about in [the section on build-time state generation](:state/build).

And finally, we come to that famous `get_template` function, which we call from `PerseusApp` to get this whole template. This is responsible for producing a [`Template`](prelude/struct.Template@perseus) that strings everything together. This too takes a `G: Html` bound, and the `Template::build("index")` call is setting up a new template whose pages will all fal under `/index`, but `index` is a special name, and it resolves to an empty string. In other words, you're creating the template for the root of your site. Then we declare our build state function, out view function, and our head function. Since we're not actually using our state in the head, we could have used `.head()` instead of `.head_with_state()`, but we showed the state for demonstration purposes. Finally, we call `.build()` to create the full `Template`, which we return.

This is called the *functional definition* pattern in Perseus: you define your `Template`s inside functions (usually called `get_template()`), which you then call in `PerseusApp`.

## An about page

With all that out of the way, let's create an even simpler page to demonstrate Perseus routing, an about page. Add `pub mod about;` to `src/templates/mod.rs`, and then put this into `src/templates/about.rs`:

```rust
{{#include ../../../examples/core/basic/src/templates/about.rs}}
```

This is very similar to our index templateL it also generates only one page, but it doesn't have any state at all. We've used `.view()`  rather than `.view_with_state()`, and, because there's no state, we don't have to worry about those finicky lifetimes: we can omit the `#[auto_scope]` entirely. The head is similar, except we're also using `.head()` to declare it on the `Template`. Note the different string in `Template::build()`, which is `about` here, the name of the template (and page) that we'll be creating. Because we're rendering one single page here, with no state generation at all, Perseus will put that page at the root of our template, `/about/<empty-string>`, which is the same as `/about`. So, when we link from the index page to `about`, we'll end up here! (It seems simple, but it's worth understanding that whole template generates page thing.)

Now it's time for error handling.
