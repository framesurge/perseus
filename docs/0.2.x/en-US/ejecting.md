# Ejecting

The Perseus CLI is fantastic at enabling rapid and efficient development, but sometimes it can be overly restrictive. If there's a use-case that the CLI doesn't seem to support, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose) on GitHub, and we'll look into supporting it out-of-the-box.

However, there are some things that are too advanced for the CLI to support, and, in those cases, you'll need to eject. Don't worry, you'll still be able to use the CLI itself for running your app, but you'll be given access to the engine that underlies it, and you'll be able to tweak basically anything you want.

*Note: ejecting from Perseus exposes the bones of the system, and you should be quite familiar with Rust before doing this. That said, if you're just doing it for fun, go right ahead!*

## Ejecting

`perseus eject`

This command does two things: it removes `.perseus/` from your `.gitignore` file, and it adds a new file called `.perseus/.ejected`.

After ejecting, there are a few things that change.

- You can no longer run `perseus clean` unless you provide the `--dist` flag (otherwise it would delete the engine you're tweaking!)
- A ton of files appear in Git that you should commit, all from `.perseus/`

## Architecture

Under the hood, Perseus' CLI is only responsible for running commands like `cargo run` and `wasm-pack build`. All the logic is done in `.perseus/`, which provides two crates, one for your app itself (which also contains a binary for running static generation) and another for the server that will run your app. That means that you can still use the CLI!

One of the first things you'll probably want to do if you choose to eject is to remove the `[workspace]` declaration from `.perseus/Cargo.toml` and instead add both crates inside to your project's workspace. This will make sure that linters like RLS will check your modifications to `.perseus/` for any problems, and you won't be flying blind.

The rest of the documentation on how Perseus works under the hood can be found in the *Advanced* section of the book, which you'll want to peruse if you choose to eject.

## Reversing Ejection

If, after taking a look at the innards, you decide that you'd like to find a solution for your problem that works without having to perform what can easily seem like the programming equivalent of brain surgery, you can easily reverse ejection by deleting the `.perseus/.ejected` file and running `perseus clean`, which will permanently delete your modifications and allow you to start again with a clean slate. Note that the reversal of ejection is irreversible, so it pays to have a backup of your changes in case you want to check something later!
