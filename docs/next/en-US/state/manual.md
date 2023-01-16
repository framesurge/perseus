# Manually implementing `ReactiveState`

For all its benefits, the `ReactiveState` derive macro does have limitations, and you'll occasionally come across a state type that you just can't derive it on. Currently, this will apply to any `enum` state type (though this will be fixed in a future version), any `struct` with generics, and any other type where you need fine-grained control over exactly how its reactivity works. Most of the time, however, this will be totally unnecessary (though reading this page is still recommended for a conceptual understanding of how the macro works).

Note that, if you want custom reactive primitives, such as a reactive `Vec`, `HashMap`, etc., these already exist [here](=state/rx_collections@perseus), once you enable the `rx-collections` feature flag! If you'd like to extend these, see the [module documentation](=state/rx_collections@perseus), since it's highly structured to enable easy user extension (and please consider contributing your new types back to the community through a crate, and let us know if you do!).

## What the macro does

The `ReactiveState` macro is responsible for the following (assuming your state is called `MyState`, with reactive alias `MyStateRx`):

- Creating a reactive version of your state as a separate type (`MyStateRx`)
- Implementing `MakeRx<Rx = MyStateRx>` for `MyState`
- Implementing `MakeUnrx<Unrx = MyState>` for `MyStateRx` (including [suspense](:state/suspense) implementation)
- Implementing [`Freeze`](=state/trait.Freeze@perseus) for `MyStateRx`

One thing worth noting is that the reactive type isn't actually called `MyStateRx`, it's named internally, and then given a type alias (but this behavior may change in future).

## How to do that yourself

Your best resource for understanding how the macro works is the code itself, which is fairly self-explanatory if you look mostly at the `quote!` sections (which output the actual code the macro creates). Even if you have no experience with macro development, this code should at least be somewhat helpful to you: you can find it [here].

### 1. Creating a reactive type

This is probably the easiest stage, because it just involves copying and pasting your existing type, just with all the fields being either wrapped in `RcSignal`s or being their respective reactive version (e.g. if you're nesting the field `foo` of type `FooState`, which has `ReactiveState` derived, then you would use `FooStateRx` or similar here).

Be sure to derive `Clone` on this type.

### 2. Implementing `MakeRx`

The [`MakeRx`](=state/trait.MakeRx@perseus) trait is the backbone of the Perseus reactive state platform, but it's actually surprisingly simply to implement! All you need to do is something like this:

```rust
impl MakeRx for MyState {
    type Rx = MyStateRx;
    fn make_rx(self) -> Self::Rx {
        // Convert  `MyState` -> `MyStateRx`
    }
}
```

Usually, the body of that `make_rx()` function will be simply wrapping all the existing fields in `create_rc_signal`, or calling `.make_rx()` on them, if they're nested.

### 3. Implementing `MakeUnrx`

The [`MakeUnrx`](=state/trait.MakeUnrx@perseus) trait is slightly more complicated, because it involves converting out of `RcSignal`s, and also the suspense system. Like `MakeRx`, there is an associated type `Unrx`, which should just reference your unreactive state type (which must implement `Serialize + Deserialize + MakeRx`). For nested reactive fields, you can simply call `.make_unrx()` to make them unreactive, whereas non-nested fields will need something like this:

```rust
(*self.my_field.get_untracked()).clone()
```

The trickiest part of this is the `compute_suspense()` function (which must be target-gated as `#[cfg(client)]`). If you're not using [suspended state](:state/suspense), you can safely leave the body of this completely empty, but if you are, you'll need to get acquainted with the [`compute_suspense`](=state/fn.compute_suspense@perseus) and [`compute_suspense_nested`](=state/fn.compute_suspense_nested@perseus) functions. These simply take the provided Sycamore reactive scope, a clone of the reactive field, and then the future returned by your suspense handler.

The most complex part of this is the suspense handler, because you want to call the function, but not `.await` on it, meaning the future can be handled by Perseus appropriately. To do this, you'll want to call your handler like this:

```rust
my_handler(
    cx,
    create_ref(cx, self.my_field.clone())
)
```

Notice how `create_ref()` is used on the field, which produces a reference scoped to the given context (incidentally, this is how all those scoped lifetimes are handled in Perseus).

### 4. Implementing `Freeze`

Once youv've done `MakeUnrx`, you're over the hump, and now you can pretty much just copy this code, substituting in the names of your state types of course:

```rust
impl Freeze for MyStateRx {
    fn freeze(&self) -> String {
        use perseus::state::MakeUnrx;
        let unrx = self.clone().make_unrx();
        serde_json::to_string(&unrx).unwrap()
    }
}
```

That `.unwrap()` is nearly always absolutely safe, provided any maps in your state have simple stringifable keys, as opposed to, say, tuples, which can't be keys in the JSON specification. If you are using a pattern like that, this would always panic, and that would unfortunately not be compatible with the Perseus state platform.

## Unreactive state

If you find the `UnreactiveState` macro doesn't work for some particular one of your types (usually one with generics), you can always implement it manually by implementing the [`UnreactiveState`](=state/trait.UnreactiveState@perseus) trait, which has no methods, no associated types, and nothing else: it's simply a marker trait! Perseus then uses that to figure out how it should handle reactivity for those particular types. 
