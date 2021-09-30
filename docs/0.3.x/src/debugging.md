# Debugging

If you're used to Rust, you might be expecting to be able to call `println!` or `dbg!` to easily print a value to the browser console while working on an app, however this is unfortunately not yet the case (this is an issue in the lower-level libraries that Perseus depends on).

However, Perseus exports a macro called `web_log!` that can be used to print to the console. It accepts syntax identical to `format!`, `println!`, and the like and behaves in the same way, but it will print to the browser console instead of the terminal.
