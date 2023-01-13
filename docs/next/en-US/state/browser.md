# Using state

Most of this section has been devoted to methods of generating state, but what about actually *using* it in views? This is actually a complex subject, mainly because of how Perseus handles the difference between reactive and unreactive state.

## The flow of state

Once your state has been generated on the engine-side, at build-time, request-time, whatever, it will be used to render the HTML of your page in advance (as early as possible). This will produce an HTML fragment, which will be interpolated into the index view of your app, with a few variables set (including a JSON representation of your state, which can be used in hydration). This is then sent to the browser, where [hydration](:fundamentals/hydration) occurs and your state is deserialized into your state type.

From here, things get a bit more complicated, because of the reactive state system. The simplest thing possible would be for your deserialized state to go straight to your page, but Perseus intervenes here. All state types in Perseus must implement four traits: `Serialize` and `Deserialize` (from Serde, to allow turning them into JSON and back again), `Clone` (for some internal mechanics, but this is used sparingly), and `MakeRx`. Now, this fourth one definitely qualifies as implementation details, and you don't need to know how this works to use it, but a lot of Rust developers like to know what's going on behind the scenes, so here you go!

When you derive `ReactiveState`, what that macro does is create an implementation of `MakeRx` that takes each field of your state and wraps it in an [`RcSignal`], from Sycamore, which makes it *reactive*, meaning you can run `.get()` and `.set()` on it. This reactive version is named according to the `#[rx(alias = "..")]` derive macro helper that you provide. Then, this reactive version has `MakeUnrx` implemented on it, which allows it to be turned back into its unreactive version. There are also some more special traits involved with [state freezing](:state/freezing), but that will be dealt with later.

<details>

<summary>How does unreactive state work?</summary>

The `MakeRx` implementation just creates a wrapper that isn't really reactive, and the `MakeUnrx` implementation just removes that wrapper. Yeah, it's that simple.

</details>

Once Perseus has made your state reactive, it will store it in the *state store*, which is pretty much a giant repository of all the states your app has. As a user visits a new page, its state will be added to this cache, allowing that page to be re-rendered later without any network requests. This can be thought of as the caching equivalent of SPA routing (if you're familiar with that), and it allows Perseus to ensure a seamless experience for your users. The number of pages that can be in the state store at any one time is 25 by default (but this may change in a future release), and you can it manually with the `.pss_max_size()` method on your `PerseusApp`.

Because Perseus makes your state reactive, *and then* stores it in the state store (abbreviated PSS for Perseus state store, since the alternative is quite unsavoury), any updates your pages make to their state will be reflected in this cache, meaning that, when users come back to, say, a pahge whose state included some form inputs, those inputs will be as they left them, without needing to rely on the browser to provide this. We strongly believe this behavior should be the default for the web, and it's built into Perseus. (If you'd like to avoid it though, you can always use unreactive state, or use `Signal`s manually that aren't checked into Perseus.)

When the user goes to a page they've already visited in the past, Perseus will try to find the cached state in the PSS, and it will use that if it can. Otherwise, it will request the state only (no HTML) from the server, and then cache it.

## Using reactive state

When you're writing views that don't take state, the function signatures are very simple: just accept a Sycamore scope, and return a `View<G>`. But, when there's state involved, things get *way* more complicated. Most of the time, you'll write something like this:

```
#[auto_scope]
fn my_view<G: Html>(cx: Scope, state: &MyStateRx) -> View<G>
```

This is made possible by the `#[auto_scope]` macro, which rewrites this function signature into something much more complicated with lifetimes everywhere:

```
fn my_view<'page, G: Html>(cx: BoundedScope<'_, 'page>, state: &'page MyStateRx) -> View<G> 
```

So let's break this down. We've gone from `Scope` to `BoundedScope`, which is an important difference. Basically, a `BoundedScope` is the fundamental primitive in Sycamore: it takes the lifetime of some root-level scope, and then the lifetime of itself. The reason for this is that, in Sycamore, you can have *child scopes*: so, in Perseus, the first lifetime is `'app`, and the second is `'page`, where the app will outlive the page. `Scope` is actually just an alias for a special type of `BoundedScope` where the lifetimes are the same, but it's much easier to write, so `#[auto_scope]` lets you do that. Notice that the `'app` lifetime can be elided, and Rust will figure this out itself.

The next thing is that the state is borrowed for the lifetime of the page, which might not make sense at first: don't you want it to live as long as the app if it's in some cache? Well, this gets to the idea of Perseus being a *framework*, not a *library*. Perseus is in charge of your state, so the cache actually comes first. The cache is what has the owned copy of the state, and you get a reference. Since the reactive version of your state is all `RcSignal`s anyway, there's no cost to `Clone`ing it, but, if we use a reference with the same lifetime as the page, Sycamore's `view!` macro can understand that it's safe to interpolate the state anywhere we want: it is *guaranteed* through Rust's type system to live as long as the page. This avoids all sorts of nasty lifetime errors, as anyone who used Sycamore before v0.8 can attest to!

Note that it's perfectly fine for you to write out the full lifetime bounds if you want to, the `#[auto_scope]` macro just exists for convenience. If you don't like the magic of it, you don't have to use it at all. (In fact, you don't have to use *any* of Perseus' macros if you don't want, and you can even disable them altogether, they're gated by the `macros` feature, which is enabled by default.)

## Unreactive state

When you're using unreactive state, none of this is necessary, because Perseus just gives you an owned copy of your state to do with as you please, and you don't need `#[auto_scope]` or any special lifetimes. (You can even use a normal `Scope`, which is a white lie to Rust's type system, but it's totally immaterial to the output, so it's a useful elision.)
