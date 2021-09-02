# Building

*You only need this page if you're not using the Perseus CLI, which performs this process for you!*

After you've defined all your templates and the like, you'll of course need to build them into pages! Perseus tries to make this process as simple as possible.

## Usage

You'll want to be able to execute this code as part of an executable, so defining a new binary in your `Cargo.toml` is advised like so:

```toml
[[bin]]
name = "ssg"
path = "src/bin/build.rs"
```

Then put this code in `bin/build.rs` (or wherever else you put your binary)

```rust
use futures::executor::block_on;
use perseus::{build::build_templates, config_manager::FsConfigManager};
use perseus_showcase_app::pages;
use sycamore::prelude::SsrNode;

fn main() {
    let config_manager = FsConfigManager::new();

    let fut = build_templates(
        vec![
            pages::index::get_page::<SsrNode>(),
            pages::about::get_page::<SsrNode>(),
            pages::post::get_page::<SsrNode>(),
            pages::new_post::get_page::<SsrNode>(),
            pages::ip::get_page::<SsrNode>(),
            pages::time::get_page::<SsrNode>(),
            pages::time_root::get_page::<SsrNode>(),
        ],
        &config_manager,
    );
    block_on(fut).expect("Static generation failed!");

    println!("Static generation successfully completed!");
}
```

This code defines a synchronous `main` function that blocks to call `build_templates`, which, unsurprisingly, builds your templates! Each entry in the vector you give this function should be a template, and note that we specify they should be `SsrNode`s, which is Sycamore's way of saying they should be prepared to be rendered on the server rather than in the browser, which makes sense given that we're building them!

The reason we don't just make this whole function asynchronous is so we don't have to include a runtime like `tokio`, which would be unnecessary.

## File Storage

It may have crossed your mind as to where all these static files are stored in production, and Perseus provides an excellent solution to this problem with custom read/write systems, documented in-depth [here](./config_managers.md).
