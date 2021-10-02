# Optimizing Code Size

If you're used to working with Rust, you're probably used to two things: performance is everything, and Rust produces big binaries. With Wasm, these actually become problems because of the way the web works. If you think about it, your Wasm files (big because Rust optimizes for speed instead of size by default) need to be sent to browsers. So, the larger they are, the slower your site will be. Fortunately, Perseus only makes this relevant when a user first navigates to your site with its [subsequent loads](../advanced/subsequent-loads.md) system. However, it's still worth optimizing code size in places.

If you've worked with Rust and Wasm before, you may be familiar with `wasm-opt`, which performs a ton of optimizations for you. Perseus does this automatically with `wasm-pack`. But we can do better.

## `wee_alloc`

Rust's memory allocator takes up quite a lot of space in your final Wasm binary, and this can be solved by trading off performance for smaller sizes, which can actually make your site snappier because it will load faster. `wee_alloc` is an alternative allocator built for Wasm, and you can enable it by adding it to your `Cargo.toml` as a dependency:

```toml
wee_alloc = "0.4"
```

And then you can add it to the top of your `src/lib.rs`:

```rust,no_run,no_playground
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

With the [basic example](https://github.com/arctic-hen7/perseus/tree/main/examples/basic), we saw improvements from 369.2kb to 367.8kb with `wee_alloc` and release mode. These aren't much though, and we can do better.

## Aggressive Optimizations

More aggressive optimizations need to be applied to both Perseus' engine and your own code, so you'll need to [eject](../ejecting.md) for this to work properly. Just run `perseus eject`, and then add the following to `.perseus/Cargo.toml`:

```toml
[profile.release]
lto = true
opt-level = "z"
```

Then add the same thing to your own `Cargo.toml`. Note that, if this is the only modification you make after ejecting, `perseus deploy` will still work perfectly as expected.

What this does is enable link-time optimizations, which do magic stuff to make your code smaller, and then we set the compiler to optimize aggressively for speed. On the [basic example](https://github.com/arctic-hen7/perseus/tree/main/examples/basic), we say improvements from 367.8kb with `wee_alloc` and release mode to 295.3kb when we added these extra optimizations. That's very significant, and we recommend using these if you don't have a specific reason not to. Note however that you should definitely test your site's performance after applying these to make sure that you feel you've achieved the right trade-off between performance and speed. If not, you could try setting `opt-level = "s"` instead of `z` to optimize less aggressively for speed, or you could try disabling some optimizations.

<details>
<summary>Read this if something blows up in your face.</summary>

As of time of writing, Netlify (and possibly other providers) doesn't support Rust binaries that use `lto = true` for some reason, it simply doesn't detect them, so you shouldn't use that particular optimization if you're working with Netlify.

</details>
