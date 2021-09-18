# CLI

Perseus has a CLI (command line interface) designed to make your life significantly easier when developing Perseus apps. Its primary functions are to build and serve your apps for you, meaning you can focus pretty much entirely on your application code and ignore all the boilerplate!

## Installation

You can install the Perseus CLI by running `cargo install perseus-cli`, it should then be available as `perseus` on your system!

We currently don't provide independent executables installable without `cargo` because you'll need `cargo` and Rust generally to be able to write a Perseus app, and Perseus depends on the `cargo` commands being available, so there's really no point. That said, if you have a use-case for this, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose)!

## Setup

Set up a library project with `cargo`, and add the following to the `[dependencies]` section in your `Cargo.toml`:

```toml
perseus = { path = "../../packages/perseus" }
sycamore = { version = "0.5", features = ["ssr"] }
sycamore-router = "0.5"
# You only need these for pages that take properties
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Then, add a `lib.rs` file to your project under `src/` that contains the following:

```rust
mod pages;
mod error_pages;

use perseus::define_app;

#[derive(perseus::Route)]
pub enum Route {
    #[to("/")]
    Index,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
}
define_app!{
    root: "#root",
    route: Route,
    router: {
        Route::Index => [
            "index".to_string(),
            pages::index::template_fn()
        ],
        Route::About => [
            "about".to_string(),
            pages::about::template_fn()
        ]
    }
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        crate::pages::index::get_template::<G>(),
        crate::pages::about::get_template::<G>()
    ]
    // config_manager: perseus::FsConfigManager::new()
}
```

This assumes you've defined a function to get your error pages elsewhere (you can read more about that [here](error_pages.md)), and that it's in a module called `error_pages`, you can customize that as needed.

The way the rest of this works is pretty simple. First off, you define a router with [Sycamore](https://sycamore-rs.netlify.app/docs/advanced/routing), which defines each of your templates and the paths on your site that it will accept. This **must** have a variant explicitly named `NotFound`, that's handled for you. Then, you define your app itself, which takes the following properties (which need to be in order right now!):

-   `root` – the CSS selector for the element to render Perseus to, which should be unique, like an HTML `id`
-   `route` – the `enum` for your app's routes that you just defined
-   `router` – a match-like input that handles each of the variants of your `route`, except `NotFound` (handled for you); each one gets mapped to the corresponding page's path (e.g. `Post` with slug `test` might be mapped to `format!("post/{}", slug)`), which shouldn't include a leading or trailing `/`
-   `error_pages` – your [error pages](error_pages.md)
-   `templates` – each of your templates, taking the `G` parameter (which will be used at runtime to render them for the server or the client)
-   `config_manager` (optional) – the [config manager](config_manager.md) your app should use, default is the inbuilt `FsConfigManager::new()`

## Usage

Once you've got that out of the way, go ahead and define your templates as usual, and then run the following command in your project's directory:

```
perseus serve
```

That will automatically prepare the CLI to work with your app, then it will build your app and statically generate everything as appropriate (using any custom config manager your specified), and then it will serve your app on <http://localhost:8080> by default!

If you want to change the host/port your app is served on, just set the `HOST`/`PORT` environment variables as you'd like.

## Other Commands

If you just want to build your app, you can run `perseus build`. If you only want to prepare the CLI to interface with your app (which creates a `.perseus/` directory), you can run `perseus prep`.

If you want to serve pre-built files (which you'll have to generate with `perseus build`), you can run `perseus serve --no-build`.

## Watching

All these commands act statically, they don't watch your code for any changes. This feature will be added _very_ soon to the CLI, but until it is, we advise you to use a tool like [`entr`](https://github.com/eradman/entr), which you can make work with Perseus like so (on Linux):

```
find . -not -path "./.perseus/*" -not -path "./target/*" | entr -s "perseus serve"
```

This just lists all files except those in `.perseus/` and `target/` and runs `perseus serve` on any changes. You should exclude anything else as necessary.
