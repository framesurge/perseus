# Installing Perseus

Perseus comes in a few parts: there's the core `perseus` crate, there's a server integration crate (like `perseus-warp` or `perseus-actix-web`), and then there's the Perseus CLI.

If you're unfamiliar with Rust's package management system, no problem, *crates* are packages that other people create so you can use their code easily. For example, the `perseus` crate exposes all the functions you need to build a Perseus app.

You also might be wondering why there are separate server integration crates. We could've bundled everything together in the `perseus` crate, but we wanted to give you a choice of which server integration to use. There are quite a few in the Rust ecosystem at the moment, and, especially if you're migrating an existing app from another system, you might already have a whole API defined in an Actix Web server, or an Axum one, or a Warp one. So, there's a Perseus integration crate for each of those, which you can easily plug an existing API into! Note that there's basically no difference between the APIs of integration crates, and that they're all fairly similar in speed (though Actix Web is usually the fastest).

Finally, the Perseus CLI is just a tool to make your life exceedingly easy when building web apps. You just run `perseus serve -w` to run your app and `perseus deploy` to output a folder of stuff to send to production! While you *could* use Perseus without the CLI, that approach isn't well-documented, and you'll probably end up in a tangle. The CLI makes things much easier, performing parallel builds and moving files around so you don't have to.

## Get on with it!

Alright, that's enough theory! Assuming you've already got `cargo` (Rust's package manager installed), you can install the Perseus CLI like so:

```sh
cargo install perseus-cli
```

This will take a few minutes to download and compile everything. (Note: if you don't have Rust or Cargo yet, see [here](https://rust-lang.org/tools/install) for installation instructions.)

Next up, you should set up your new app like so:

```sh
perseus new my-app
cd my-app
```

This will create a new directory called `my-app/` in your current directory, setting it up for a new Perseus project. If you want to move this directory somewhere else, you can do that as usual, everything's self-contained.

Note that any `perseus` command will also install the `wasm32-unknown-unknown` target if you have `rustup` available to do so, since you need it for developing with Perseus. Also note that the Perseus CLI used to have some other dependencies, namely `wasm-pack`, but these are now all inbuilt, and will be automatically installed and managed for you!

You can run `perseus serve -w` now if you want to see the placeholder app, or you can move ahead to the next section to get your feet wet.
