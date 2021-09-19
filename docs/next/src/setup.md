# Setup

Perseus is designed to be server-agnostic, and can be hosted functionally anywhere you can run Rust (or at least execute a binary), for example in a serverless function.

## Installation

You can install the Perseus crate by adding the following to your `Cargo.toml` under `[dependencies]`:

```toml
perseus = "0.1"
```

## Project Structure

The structure of a Perseus project is described in detail in the [architecture section](./arch.md), but you'll need two crates, an app and a server. A great example of this is in the showcase example, which you can find on GitHub [here](). We advise setting up a Cargo workspace with an `app` and a `server` crate for development. Soon, Perseus will support a CLI to run your server for you so you can focus more on your app.
