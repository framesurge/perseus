# Using capsules

Using capsules in your own code involves a few steps. First, you'll want to create a `capsules/` directory in the root of your project (this is just convention), which is just like `templates/`, but (surprise surprise) for capsules. You'll also probably want to bring [`lazy_static`](https://docs.rs/lazy_static/latest/lazy_static) into your project as a dependency so you can use the *referential defin pattern* of capsule definition. This means, rather than having something like a `get_capsule()` function that you use to get your capsule, you create a static reference to it that you can use from anywhere in your program. This is because, unlike templates, capsules get used in more than one place than just `PerseusApp`: you'll also need them where you want to interpolate them into templates. Note that doing this means you should add your capsules to your `PerseusApp` like so:

```rust
.capsule_ref(&*crate::capsules::my::capsule::CAPSULE)
```

## `Capsule` vs. `Template`

When you're creating a capsule, there's a new type to get familiar with, called [`Capsule`](=prelude/struct.Capsule@perseus), which works similarly to the templates you're used to, but it's also slightly different. Basically, you want to start with a `Template`, define all your state generation stuff on there (but not your views), then pass that to `Capsule::build()` *without* alling `.build()` on it, and then specify your capsule-specific properties on `Capsule`. It's best to give an example:

```rust
{{#include ../../../examples/core/capsules/src/capsules/greeting.rs}}
```

This is a very simple capsule analogous to the *Hello world!* template we built in the first tutorial, but it shows how capsules generally work. You have the definition of the capsule as a static reference up the top (here using a `get_capsule()` function, but you can do whatever you like here). Notice the use of [`PerseusNodeType`](=prelude/type.PerseusNodeType@perseus), which denotes the correct rendering backend based on system circumstances (namely the presence of the `hydrate` feature and the `engine`/`client` flags), since generics aren't allowed in lazy statics. This will be reconciled using internal transmutes with whatever `G` you use in your templates (this is unsafe code internally, but your app will panic if it thinks it will undergo any undefined behavior, and this should really never happen unless you're doing some extremely wacky things).

Notice that `greeting_capsule`, our view function, is almost identical to that for a template, except it takes a third argument `props` for the properties, which just need to be `Clone`, and that also makes the second generic on `Capsule` itself.

In the `get_capsule` function, you can see we're creating a `Template` with the name of the capsule (which will be gated behind a special internal prefix, so it won't conflict with any of your templates), and then we declare state generation functions and the like on there (like [build-time state](:state/build)). This is *identical* to a full template.

Then, we set the *fallback function*, which is a view that will be used while widgets that this capsule produces are still being fetched: this is usually a loader or something similar, but here we're just using `.empty_fallback()` to use an empty view. Any capsules without fallback functions will fail the Perseus build process.

We then use `.view_with_state()`, which might look exactly the same as the template one, but remember that capsule functions take an extra argument! They also have completely different internal logic compared to templates, so make sure you're defining your views on the `Capsule` rather than the `Template`! (If you make a mistake here, your capsules will simply be blank, rather than causing untold internal errors.)

Finally, we call `.build()` to turn that all into a proper `Capsule`.

## Using widgets

Here's an example of a page that uses some widgets:

```rust
{{#include ../../../examples/core/capsules/src/templates/index.rs}}
```

This template uses two widgets: one called `LINKS`, and another called `WRAPPER` (which is a wrapper over the `GREETING` capsule we defined in the previous example). To use these, we interpolate them like variables into a Sycamore `view!` using the `.widget()` function, which takes three arguments: the Sycamore scope, *the path to the widget*, and the properties. For capsules that have no properties, we use the unit type `()`.

Note that there is no place where we have to declare all the widgets a page uses, and they can even be state dependent (e.g. only if the state property `foo` is set to `5` do we render a widget from the `BAR` capsule). Perseus will figure out which widgets a page uses by actually rendering it. This also means that you can nest widgets (as in the `WRAPPER` capsule in the above example), but don't do too much nesting, since the best Perseus can do is build each layer one at a time, meaning, if you have five layers of nesting, it will take five sequential re-renders to render your whole page on the engine-side (a similar thing goes for fetching on the client-side).

### Widget paths

That second argument to the capsule's `.widget()` function is by far the most important, and this is why we've emphasized that idea of **template + page = state**. Based on that, and Perseus' routing systems, any template `foo` will render pages under `/foo/` on your website. So this argument is what goes under `/foo/`. Let's say we have a capsule `ALPHABET` that renders one widget for every letter of the Latin alphabet: we might put it `/a` as our path if we wanted to render the `__capsule/alphabet/a` widget, or `/x` if we wanted to render `__capsule/alphabet/x`. (That `__capsule` prefix is applied internally, and you'll only need it if you're manually querying Perseus' API.)

In the above example, we used the empty string to denote the index page, because the capsules we're using only render one widget each.

if you want to see a more advanced example of a capsule that uses incremental generation to build widgets, check out [this code].

*Note: while it might seem extremely weird, there is nothing to stop you from reactively changing the widgets you render on the client-side, as in [this example].*

## Delayed widgets

As explained earlier, Perseus automatically collates all widgets into one HTMl page before serving an *initial load*, for speed, but sometimes this is undesirable. Sometimes, no matter what, you want one section of your page to be loaded after the rest of the page is ready, usually to improve page load times by delaying the load of a heavy section. This can be done trivially by replacing `.widget()` with `.delayed_widget()`. Yep, that's all.

Deciding whether you want to use a delayed widget rather than a normal one should be a decision carefully made with reference to real user data! In some circumstances, they're a great idea that will improve performance, but, at other times, they can *worsen* performance!

## Rescheduling

One of the trickiest parts of the capsule system to build internally centered around this problem: what should Perseus do when you create a page that uses build state, and make that use a widget that uses request state? The page would normally be rendered at build-time, but it can't be, because it's waiting on a widget. In these cases, Perseus needs to *reschedule* the build of such pages to request-time, which it needs your permission to do (since this will reduce performance). When you're building your app, you should keep this in the back of your mind, and be prepared for rescheduling errors that might arise in the build process: these can always be solved by adding `.allow_rescheduling()` to the definition of a template that fits these properties. Do *not* pre-emptively add `.allow_rescheduling()`, wait for the actual error to make sure you need it (there are some cases where Perseus can optimize things).

One notable instance where this isn't necessary is in incremental generation. For example, let's say you have a capsule `number` that has a build paths listing of `1`, `2`, and `3`, but it can incrementally render any number. Then let's say you have a page called `four` that uses `number/4` --- typically, Perseus would wait until somebody requested the `foo/4` widget to render it, but here it's being very clearly used at build-time, so Perseus will figure out what you mean and just render it at build-time anyway. This means you don't have to laboriously keep all your build paths in sync, which can lead to faster development and fewer maintainence headaches.

## Support

The Perseus capsules system is not only very new, it is a completely novel architecture, so there are bound to be bugs and idiosyncracies that can be improved. If you're having problems, even if you don't think they're a bug, please let us know through [a GitHub discussion], or [on Discord] --- every little bit of feedback helps us improve Perseus and make it easier for you to develop lightning-fast apps! If you do reckon you've found a bug, or if you'd like to request a new feature, please open an issue [on GitHub] and let us know!
