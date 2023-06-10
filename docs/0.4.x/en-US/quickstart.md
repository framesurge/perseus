# Quickstart

To quickly start using Perseus, follow these steps:

1. Install Rust: Make sure you have the Rust language installed. We recommend using [`rustup`](https://rustup.rs) to easily manage the different components of the language.

2. Install Perseus: Run the following command to install the Perseus command-line interface:
   ```sh
   cargo install perseus-cli
   ```

3. Create a new app: Navigate to your desired projects directory and run `perseus new my-app` to create a new app. This will create a new directory named `my-app/`, which you can switch to by running `cd my-app`.

4. Start the app: Run the following command to start your app:
   ```sh
   perseus serve -w
   ```

   The `serve` command sets up a server for your app, enabling you to use some more advanced features of Perseus (in comparison to a simpler *static export*). The `-w` flag watches for changes in the current directory, automatically rebuilding your app and reloading your browser when you modify any code. Once the command is executed, visit <http://localhost:8080>, and you should see a welcome screen!

   The `perseus serve` command involves several stages:
   - **Generating your app**: Compiles the engine-side (server-side) of your app and builds all the app's pages.
   - **Building your app to Wasm**: Compiles the browser-side of your app into WebAssembly (Wasm) for running in the browser.
   - **Building server**: Prepares the server used by Perseus to run your code.
   - **Installing external tools**: (for first-time Perseus users): Downloads external dependencies, such as `wasm-opt`, to enhance Wasm performance in production. Additionally, `rustup` installs the `wasm32-unknown-unknown` target, which represents the browser in Rust. If you're not using `rustup`, you may need to manually install this target.

   **Note**: These stages run in parallel, requiring a reasonable amount of memory. If you're using an older device or running multiple Perseus builds simultaneously, consider using the `--sequential` flag with the `perseus serve` command to execute the stages sequentially.

   Now, if you change some code in, say, `my-app/src/templates/index.rs` (where, by convention, you store the code for your app's landing page), you should see the build process automatically restart, and, once it's done, your browser will automatically reload with your changes!

   It's important to note that building compiled languages like Rust takes a bit longer than building an interpreted language like JavaScript, however build speeds will only get quicker in future, with improvements to Perseus, Sycamore, and Rust itself! For tips on reducing compilation time, see [this page](https://framesurge.sh/perseus/en-US/docs/0.4.x/fundamentals/compilation-times/).
