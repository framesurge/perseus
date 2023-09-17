# Debugging

For all its features, Perseus isn't a miracle-worker, and, until AI replaces all programmers, you're still going to need to do your fair share of debugging when you're building a Perseus app. Most of time, bugs will be caught neatly by the compiler system, which you can run in a loop with `perseus check -w`. This will re-run every time you change some code, and it will check both the engine-side and the client-side of your app, making sure you don't miss any bugs. If you want to also catch any of what we call *build-time errors* (which are runtime errors in Rust, but they occur at build-time, so they're more similar to compile-time errors from Perseus; perspective), you can run `perseus check -gw` to also test state generation.

In the vast majority of cases, if `perseus check -gw` passes, then any other Perseus command will also pass. Any deviations from this are most likely to be bugs in your request-time logic (e.g. incorrectly parsing a cookie).

## Client-side debugging

Unfortunately, debugging Wasm isn't the best experience yet, as debuggers really aren't too well-equipped for this just yet. Usually, the best policy here is to use some good old `println!` logging, but you might quickly discover that `println!()`, `dbg!()`, etc. don't actually work at all in the browser. One day, this will hopefully change, but, for now, you can use [`web_log!()`](=macro.web_log@perseus), which behaves exactly like `println!()` to print to the browser console. Note that Perseus enforces that all the types it exposes implement `Debug`, so you shouldn't have any problems when debugging things coming from Perseus.

Using this macro on the engine-side will lead to it just calling `println!()`, but you could also use `dbg!()` in such cases, as it's often more convenient.

## Engine-side logging

However, if you try to, say, call `dbg!()` in your build-time logic, you might discover that you get absolutely zilch output in the console unless the whole process fails. This is because Perseus takes the conservative route, and only prints the output of its underlying calls to `cargo` if the build process fails. This can make subtle logic errors very difficult to debug, so Perseus provides the `snoop` commands to help you. There are three:

- `perseus snoop build` will run the build process directly, with no frills, allowing you to see all the output of your own code (Perseus performs no logging)
- `perseus snoop wasm-build` will run the Wasm build process, which is just compiling your code to Wasm (you probably won't use this unless you're having Wasm-specific compiler errors)
- `perseus snoop serve` will run the server directly, allowing you to see any `dbg!()` calls or the like that occur on requests

Importantly, you'll have to run `perseus build` before `perseus snoop serve`, since it expects your app to be built before it executes. If you have errors about files not being found, you've probably forgotten `perseus build`.

Note that the output in `perseus snoop serve` may differ depending on the server integration you're using (e.g. Actix Web will clearly output when a thread fails).
