# Development Cycle

When you're developing a Perseus app, you'll generally have two "modes": coding, and fine-tuning. In the *coding* stage, you're building features of your app, which will typically involve quite a lot of working on business logic, etc. If you're familiar with Rust programming, this is the stage when you'd be using `cargo check` instead of `cargo run`. Conveniently, Perseus provides `perseus check -w` for this, which will not only `cargo check` your app's engine-side, but also the browser-side, because each one is built for a different target. This command is *much* faster than `perseus serve`, because it just checks your code, rather than actually compiling it. If you want to test your build logic as well, you can run `perseus check -gw`, which will also test this (but that will take a bit longer).

When you're using an IDE, like VS Code, you'll usually want proper syntax highlighting, and you may find that Perseus causea  few problems. This is because Perseus distinguishes between the engine and the browser by using a custom feature, so you'll need to create a `.cargo/config.toml` file in the root of your project with the following contents:

```toml
[build]
rustflags = [ "--cfg", "engine" ]
```

That will set up your IDE to only check your app's engine-side code, which, somewhat counterintuitively, *does* include things like `view!`, because, remember, Perseus renders everything ahead of time, so it still needs access to all that on the engine-side. Usually, this will be enough, but, when you're working on some browser-only logic, you can change that `engine` to be `browser` instead, and your IDE will automatically update. These settings won't affect commands like `perseus check` or `perseus serve`, which provide these flags automatically.

Importantly, any time you don't need to be actually seeing the views your app is producing, you should use `perseus check` instead of one of the other commands, because it will be *much* faster (especially if you follow [these tips](:fundamentals/compilation-times)).

Then, when you need to see what your app looks like in a browser, for example when you're styling it, or testing a particular feature, you can use `perseus serve -w`. If you're updating static content (like a `.css` file), rebuilds will be pretty much instant, but updating the Rust code of your app will be a fair bit slower. This is unfortunately a downside of working with Rust web development, but, in return, you get an *extremely* performant site that eliminates whole classes of bugs that run rampant in JS code.

*Note: there is currently ongoing development on the Sycamore side for a system to remove the need for recompilation when you change things in the `view! { .. }` macro, which will dramatically improve performance.*
