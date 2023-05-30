# Quickstart

To quickly start using Perseus, follow these steps:

1. Install Rust: Make sure you have the Rust language installed. It is recommended to use [`rustup`](https://rustup.rs) for easy management of Rust components.

2. Install Perseus: After installing Rust, run the following command to install Perseus:
   ```sh
   cargo install perseus-cli
   ```

3. Create a new app: Navigate to your desired projects directory and run `perseus new my-app`. This will create a new directory named `my-app/`. Change into that directory by running `cd my-app`.

4. Start the app: Run the following command to start your app:
   ```sh
   perseus serve -w
   ```

   The `serve` command sets up a server for your app, enabling you to use advanced features of Perseus. The `-w` flag watches for changes in the current directory, automatically rebuilding your app when you modify any code. Once the command is executed, visit <http://localhost:8080> to see the welcome screen!

   The `perseus serve` command involves several stages:
   - **Generating your app**: Compiles the engine-side (server-side) of your app and builds all the app's pages.
   - **Building your app to Wasm**: Compiles the browser-side of your app into WebAssembly (Wasm) for running in the browser.
   - **Building server**: Compiles and prepares the server used by Perseus to run your code.
   - **Installing external tools**: (for first-time Perseus users): Downloads external dependencies, such as `wasm-opt`, to enhance Wasm performance in production. Additionally, `rustup` installs the `wasm32-unknown-unknown` target, which represents the browser in Rust. If you're not using `rustup`, manually install this target.

   **Note**: These stages run in parallel, requiring a significant amount of memory. If you're using an older device or running multiple Perseus builds simultaneously, consider using the `--sequential` flag with the `perseus serve` command to execute the stages sequentially.

   Whenever you modify code in `my-app/src/templates/index.rs` (where the landing page code is conventionally stored), the build process will automatically restart, and your browser will reload with the updated changes.

   It's important to note that building compiled languages like Rust takes longer than JavaScript builds. However, future improvements to the Rust compiler, [Sycamore](https://github.com/sycamore-rs/sycamore) (the framework used by Perseus), and Perseus itself will enhance build speeds. For tips on reducing compilation time, refer to [this page](https://framesurge.sh/perseus/en-US/docs/0.4.x/fundamentals/compilation-times/).