# Deploying

When you've built your app, and you're ready to go to production with it, Perseus provides some nifty tools to make your life easy. First off, you'll notice that all your files are sequestered away in `dist/`, which is all very well for keeping a ton of cached stuff out of your way, but not very useful for getting production binaries!

When you're ready for production, you should run `perseus deploy`, which will build your entire app in release mode (optimizing for size in the browser and speed on the server, which we'll return to), which will take quite a while. This is a good time to make yourself a beverage of some form. When it's done, you'll get a `pkg/` folder with some stuff inside. The main thing is a file `pkg/server`, which is a binary that will run your app's server, using the rest of the stuff in there for all sorts of purposes. Unless you really know what you're doing, you shouldn't add files here or rearrange things, because that can send the production server a little crazy (it's very particular).

If you don't need a server for your app, you can use `perseus deploy -e`, which will produce a set of static files to be uploaded to your file host of choice.

## Optimizations

Of course, when you're deploying your app, you want it to be as fast as possible. On the engine-side, this is handled automatically by Rust, which will naturally produce super-fast binaries. On the browser-side, there are problems though. This is because of the way the internet works --- before your users can run your super-fast code, they need to download it first. That download process is what's involved in loading your app, which is generally the indicator of speed on the web. That means we actually improve the speed of your app by optimizing more aggreassively for the *size* of your app, thus minimizing download times and making your app load faster.

With JavaScript, you can 'chunk' your app into many different files that are loaded at the appropriate times, but no such mechanisms exists yet for Wasm of any kind, which means your final `bundle.wasm` will be big. This is often used as a criticism of Wasm: the Perseus basic example produces a bundle that's over 200kb, where a JavaScript equivalent would be a tenth of the size. However, this comparison is flawed, since JavaScript is actually slower to execute. It's an oversimplification, but you can think of it like this: JS needs to be 'compiled' in the browser, whereas Wasm is already compiled. For that reason, it's better to compare Wasm file sizes to image file sizes (another type of file that doesn't need as much browser processing). In fact, that over 200kb bundle is probably faster than the tenth-of-the-size JS.

If you're getting into real strife with your bundle sizes though, you can, theoretically, split out your app into multiple components by literally building different parts of your website as different apps. This should be an absolute last resort though, and we have never come across an app that was big enough to need this. (Remember that Perseus will still give your users a page very quickly, it's just the interactivity that might take a little longer --- as in a few milliseconds longer.)

However, there are some easy things you can do to make your Wasm bundles much smaller. The first are applied in `Cargo.toml` to the release profile, which allows you to tweak compilation settings in release-mode (used in `perseus deploy`). (Note: if your app is inside a workspace, this has to go at the root of the workspace.)

```toml
[profile.release]
lto = true
opt-level = "z"
codegen-units = 1
```

The first of these lets LLVM inline and prune functions more aggressively, leading to `perseus deploy` taking longer, but producing a faster and smaller app. The second is Cargo's optimization level, which is usually set to 3 for release builds, optimizing aggressively for speed. However, on the web, we get better 'speed' out of smaller sizes as explained above, so we optimize aggressively for size (note that sometimes optimizing normally for size with `s` can actually be better, so you should try both). The third of these is another one that makes compilation take (much) longer with `perseus deploy`, but that decrease speed by letting LLVM basically do more work.

If you're only ever going to export your app, this is fine, but, if you ever use a server, then this will be a problem, as these size-focused optimizations will apply to your server too, slowing everything down again! Unfortunately, Cargo doesn't yet support [target-specific profiles](https://github.com/rust-lang/cargo/issues/4897), so we need to hack our way around this. TODO

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
