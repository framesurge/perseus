# Quickstart

To get started with Perseus, you should first make sure you have the Rust language installed, and it's recommended that you do this through [`rustup`](https://rustup.rs), which will let you manage the different parts of Rust very easily.

Once you have Rust installed, you can run the following command to install Perseus:

```sh
cargo install perseus-cli --version 0.4.0-beta.21
```

(While v0.4.x is still in beta, that `--version` flag is needed to make sure you get the latest beta version.)

Now, pop over to some directory where you keep your projects, and run `perseus new my-app`. That will create a new directory called `my-app/`, which you can easily `cd` into, and, once you're there, you can run this command to start your app:

```sh
perseus serve -w
```

The `serve` command tells Perseus to spin up a proper server for your app, rather than just exporting it to a series of static files (doing this lets you use some of the more advanced features of Perseus, if you want), and `-w` tells it to watch the files in the current directory for changes, so that your app will automatically be rebuilt if you change any code. After this command is done, go and take a look at <http://localhost:8080>, and you should see a welcome screen!

This command has a few stages. First, there's *Generating your app...*, which will compile your app's *engine-side* (often called the *server-side*, but Perseus has exporting, tinkering, and a million other things that happen there, so we just call the not-browser the *engine* for simplicity) and build all your app's pages. Then, there's *Building your app to Wasm...*, which compiles your app's *browser-side* into WebAssembly, allowing it to be run in the browser. Finally, there's *Building server...*, which just compiles and prepares the server that Perseus will use to run your code. If it's the first time you're running Perseus, there'll also be a stage for *installing external tools...*, in which Perseus downloads some external dependencies, like `wasm-opt`, which helps to supercharge your Wasm in production. Here, Perseus will also use `rustup` to install the `wasm32-unknown-unknown` target, which is Rust's way of saying 'the browser'. If you're not using `rustup`, you'll need to install this target manually.

*In fact, all these stages run in parallel, which is why Perseus builds take up a fair bit of memory. If you're running on an older device, you might want to add the `--sequential` flag to the above command, which will run these steps one-by-one. This can also be useful if you're running multiple Perseus builds at once.*

Now, if you change some code in, say, `my-app/src/templates/index.rs` (where, by convention, you store the code for your app's landing page), you should see the build process automatically restart, and, once it's done, your browser will automatically reload with your changes! 

Unfortunately, one downside of compiled languages like Rust is that building them takes a while, so don't expect Perseus builds to be as snappy as JS builds. However, this usually isn't actually too much of a problem, and the builds will only get faster in future, as the Rust compiler improves, as Sycamore improves, and as Perseus improves! For tips on reducing compilation time, take a look at [this page](:fundamentals/compilation-times).
