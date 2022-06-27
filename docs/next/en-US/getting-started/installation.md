# Installing Perseus

Perseus comes in a few parts: there's the core `perseus` crate, there's a server integration crate (like `perseus-warp` or `perseus-actix-web`), and then there's the Perseus CLI.

If you're unfamiliar with Rust's package management system, no problem, *crates* are packages that other people create so you can use their code easily. For example, the `perseus` crate exposes all the functions you need to build a Perseus app.

You also might be wondering why there are separate server integration crates. We could've bundled everything together in the `perseus` crate, but we wanted to give you a choice of which server integration to use. There are quite a few in the Russt ecosystem at the moment, and, especially if you're migrating an existing app from another system, you might already have a whole API defined in an Actix Web server, or an Axum one, or a Warp one. So, there's a Perseus integration crate for each of those, which you can easily plug an existing API into! Note that there's basically no difference between the APIs of integration crates, and that they're all fairly similar in speed (though Actix Web is usually the fastest).

Finally, the Perseus CLI is just a tool to make your life exceedingly easy when building web apps. You just run `perseus serve -w` to run your app and `perseus deploy` to output a folder of stuff to send to production! While you *could* use Perseus without the CLI, that approach isn't well-documented, and you'll probably end up in a tangle. The CLI makes things much easier, performing parallel builds and moving files around so you don't have to.

## Get on with it!

Alright, that's enough theory! Assuming you've already got `cargo` (Rust's package manager installed), you can install the Perseus CLI like so:

```sh
cargo install perseus-cli
```

This will take a few minutes to download and compile everything. (Note: if you don't have Rust or Cargo yet, see [here](https://rust-lang.org/tools/install) for installation instructions.)

Next up, you should create a new directory for your app and set it up like so:

```sh
cargo new --lib my-app
cd my-app
```

This will create a new directory called `my-app/` in your current directory, set it up for a new Rust project, and then move into that directory. If you want to move this directory somewhere else, you can do that as usual, everything's self-contained.

You'll notice in there a file called `Cargo.toml`, which is the manifest of any Rust app; it defines dependencies, the package name, the author, etc.

In that file, add the following underneath the `[dependencies]` line:

```
perseus = { version = "=0.4.0-beta.1", features = [ "hydrate" ] }
sycamore = "=0.8.0-beta.7"
```

*Note: we install Sycamore as well because that's how you write views in Perseus, it's useless without it! We've also used the `=[version]` syntax here to pin our app to a specific beta version of Sycamore, otherwise Cargo will politely update it automatically when a new version comes out. Normally, that's very nice of it, but, when we're working with beta versions (which won't be for much longer, hopefully!), a new version could break your code, so it's best to deliberately update when you decide to.*

Now you can run `cargo build`, and that will fetch the `perseus` crate and get everything ready for you! Note that we haven't defined the integration as a dependency yet, and that's deliberate, because this `Cargo.toml` is going to get *much* more complicated!

But, for now, you're all good to move onto the next section, in which we'll build our first app with Perseus!
