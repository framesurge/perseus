# Optimizing Code Size

If you're used to working with Rust, you're probably used to two things: performance is everything, and Rust produces big binaries. With Wasm, these actually become problems because of the way the web works. If you think about it, your Wasm files (big because Rust optimizes for speed instead of size by default) need to be sent to browsers. So, the larger they are, the slower your site will be. Fortunately, Perseus only makes this relevant when a user first navigates to your site with its [subsequent loads](:advanced/subsequent-loads) system. However, it's still worth optimizing code size in places.

If you've worked with Rust and Wasm before, you may be familiar with `wasm-opt`, which performs a ton of optimizations for you. Perseus does this automatically with `wasm-pack`. But we can do better.

The easiest way to apply size optimizations for Perseus is the [`perseus-size-opt` plugin](https://github.com/framesurge/perseus-size-opt), which prevents the need for ejecting to apply optimizations in `.perseus/`, and requires only a single line of code to use. Check it out [here](https://github.com/framesurge/perseus-size-opt) for more details! It's recommended that all Perseus apps use this plugin before they deploy to production, because it can result in size decreases of upwards of 100kb, which translates into real increases in loading time for your users.

*Note: every size optimization has a trade-off with either speed or compile-time, and you should test the performance of your app after applying these optimizations to make sure you're happy with the results, because the balance between speed and size will be different for every app. That said, using even all these optimizations usually has a negligible performance impact.*
