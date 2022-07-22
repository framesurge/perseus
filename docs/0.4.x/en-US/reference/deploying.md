# Deploying

When you've built your app, and you're ready to go to production with it, Perseus provides some nifty tools to make your life easy. First off, you'll notice that all your files are sequestered away in `dist/`, which is all very well for keeping a ton of cached stuff out of your way, but not very useful for getting production binaries!

When you're ready for production, you should run `perseus deploy`, which will build your entire app in release mode (optimizing for size in the browser and speed on the server, which we'll return to), which will take quite a while. This is a good time to make yourself a beverage of some form. When it's done, you'll get a `pkg/` folder with some stuff inside. The main thing is a file `pkg/server`, which is a binary that will run your app's server, using the rest of the stuff in there for all sorts of purposes. Unless you really know what you're doing, you shouldn't add files here or rearrange things, because that can send the production server a little crazy (it's very particular).

If you don't need a server for your app, you can use `perseus deploy -e`, which will produce a set of static files to be uploaded to your file host of choice.

## Optimizations

Of course, when you're deploying your app, you want it to be as fast as possible. On the engine-side, this is handled automatically by Rust, which will naturally produce super-fast binaries. On the browser-side, there are problems though. This is because of the way the internet works --- before your users can run your super-fast code, they need to download it first. That download process is what's involved in loading your app, which is generally the indicator of speed on the web. That means we actually improve the speed of your app by optimizing more aggreassively for the *size* of your app, thus minimizing download times and making your app load faster.

With JavaScript, you can 'chunk' your app into many different files that are loaded at the appropriate times, but no such mechanisms exists yet for Wasm of any kind, which means your final `bundle.wasm` will be big. This is often used as a criticism of Wasm: the Perseus basic example produces a bundle that's over 200kb, where a JavaScript equivalent would be a tenth of the size. However, this comparison is flawed, since JavaScript is actually slower to execute. It's an oversimplification, but you can think of it like this: JS needs to be 'compiled' in the browser, whereas Wasm is already compiled. For that reason, it's better to compare Wasm file sizes to image file sizes (another type of file that doesn't need as much browser processing). In fact, that over 200kb bundle is probably faster than the tenth-of-the-size JS.

If you're getting into real strife with your bundle sizes though, you can, theoretically, split out your app into multiple components by literally building different parts of your website as different apps. This should be an absolute last resort though, and we have never come across an app that was big enough to need this. (Remember that Perseus will still give your users a page very quickly, it's just the interactivity that might take a little longer --- as in a few milliseconds longer.)

Very usefully, the Perseus CLI automatically applies several optimizations when you build in release mode. Specifically, Cargo's optimization level is set to `z`, which means it will aggressively optimize for size at the expense of speed, which actually means a faster site, due to faster load times for the Wasm bundle. Additionally, `codegen-units` is set to `1`, which slows down compilation with `perseus deploy`, but both speeds up, and reduces the size of, the final bundle.

Notably, these optimizations are enabled through the `RUSTFLAGS` environment variable on the Wasm build, and only in release-mode (e.g. `perseus deploy`). If you want to tweak these changes, you can directly override the value of that environment variable in this context (i.e. you can apply your own optimization settings) by setting the `PERSEUS_WASM_RELEASE_RUSTFLAGS` environment variable. This takes the same format as `RUSTFLAGS`, and its default value is `-C opt-level=z -C codegen-units=1`.

*Note: the reason these optimizations are applied through `RUSTFLAGS` rather than `Cargo.toml` is because Cargo doesn't yet support target-specific release profiles, and we only want to optimize for size on the browser-side. Applying the same optimizations to the server would slow things down greatly!*

The next thing you can do is switch to `wee_alloc`, an alternative allocator designed for the web that produces less efficient, but smaller bundles. Again though, that lower efficiency is barely noticeable, while every kilobyte you can shave off the bundle's size leads to a notably faster load speed. Importantly, you still want to retain that efficiency on the server, so it's very important to only use `wee_alloc` on the browser-side, which you can do by adding the following to the very top of your `lib.rs`:

```rust
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

To make this work, you should also add the following to your `Cargo.toml` under the `[target.'cfg(target_arch = "wasm32")'.dependencies]` section (for browser-only dependencies):

```toml
wee_alloc = "0.4"
```

Finally, for another small cut to bundle sizes, you can set `wasm-opt`, a tool that `wasm-pack` runs automatically (and the Perseus CLI runs `wasm-pack`) to optimize for size by adding the following to your `Cargo.toml`:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = [ "-Oz" ]
```

You can find more information about optimizing Wasm bundle sizes [here](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size).
