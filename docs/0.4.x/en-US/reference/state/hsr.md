# Hot State Reloading

If you've started using Perseus with reactive state, you may have already noticed something pretty cool. If you're using `-w` in the CLI to rebuild your app when you change your code, you'll find that the state of your app is maintained! This is called *hot state reloading*, and it's currently a feature entirely unique to Perseus.

If you've come from the JavaScript world, you might have heard of *hot module reloading*, or HMR, which is used by many JS frameworks to figure out which parts of your code you changed in development so that they only need to change the smallest part, meaning most of your app's state is retained. This approach requires being able to break your code up into many chunks, which is easy with JS, but currently extremely difficult with Wasm, so Perseus takes a different approach.

Perseus supports [state freezing](:reference/state/freezing) automatically, which allows you to store the entire state of your app in a string and reload from that at any time. Perseus also supports [freezing that state to IndexedDB](:reference/state/idb-freezing). When you combine that with Perseus' [live reloading](:reference/live-reloading) system, why not freeze the state before every live reload and thaw it afterward? This is exactly what Perseus does, and it means that you can change your code and pick up from *exactly* where you were in your app without missing a beat.

If you ever want to ditch the current state completely, just manually reload the page in your browser (usually by pressing `Ctrl+R` or the reload button) and Perseus will start your app afresh!

HSR is inbuilt into Perseus, and it automatically enabled for all development builds. When you build for production (e.g. with `perseus deploy`), HSR will automatically be turned off, and you won't have to worry about it anymore. If you feel HSR gets in your way, you can easily disable it by disabling Perseus' default features (by adding `default-features = false` to the properties of the dependency `perseus` in your `Cargo.toml`). Note though that this will also disable [live reloading](:reference/live-reloading), and you'll need to manually the `live-reload` feature to get that back.

## Problems

HSR is far less buggy than most implementations of HMR because it takes advantage of features built into Perseus' core, though there are some cases in which it won't work. If you're finding that you make a modification to your code and your last state isn't being restored, you'll probably find the reason here.

### Incorrect Data Model

This is the most common case. If you're finding that most state properties are being restored except one or two, then you'll probably find that that those properties aren't in your template's data model. In other words, they aren't part of the state for your template that you're providing to Perseus, which usually means you're setting them up as `Signal`s separately. Moving them into your data model should solve your problems.

### New Data Model

If you've changed the structure of your template's data model, for example by adding a new property that it includes, then you'll find that Perseus can't deserialize the state it kept from before properly (it saved your old data model, which is different to the new one), so it'll abort attempting to thaw the old state and regenerate from scratch. Unfortunately, due to the strictness of Rust's type system, this is unavoidable.

### Corrupt Entry

If your state isn't restored and the above reasons don't fit, then it's possible that the state may have somehow been corrupted. That said, this is very unlikely, and really shouldn't happen outside contrived scenarios. That said, Perseus should automatically resolve this by clearing the stale state and the next reload should work properly. If it doesn't, you should manually reload the page to get out of any strange logic issues. If that still doesn't work, try going into your browser's developer tools and making the console logs persist across reloads, there could be some helpful error messages in there (if they occur just before the CLi-induced reload, they'll be wiped away by the browser).
