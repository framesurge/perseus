# Welcome to Perseus 0.5!

[Home][repo] • [Crate Page][crate] • [API Documentation][docs] • [Contributing][contrib]

Welcome to the Perseus 0.5.x documentation! This version brings significant improvements including **Sycamore 0.9.2** support with its simplified reactive model.

## What's New in 0.5.x

- **Sycamore 0.9.2**: Simplified view syntax without `Scope` parameters
- **Cleaner Component API**: Views now return `View` directly without generics
- **Link Component**: Built-in `Link` component for client-side navigation
- **Improved Hydration**: Better SSR hydration with data-hk attributes
- **Enhanced Error Handling**: More robust error views and panic handling

## Quick Example

Here's what a simple Perseus page looks like in 0.5.x:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn my_page() -> View {
    view! {
        h1 { "Hello, Perseus!" }
        p { "This is a simple page." }
        Link(to = "/about") { "Go to About" }
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .view(my_page)
        .build()
}
```

Notice how clean the syntax is - no `cx` parameter, no `G: Html` generics, just straightforward Rust code!

## Getting Started

If you're new to Perseus, start with the [Quickstart](/docs/quickstart) guide to get your first app running in minutes.

If you're upgrading from 0.4.x, check out the [Migration Guide](/docs/migrating) for step-by-step instructions.

If you like Perseus, please consider giving us a star [on GitHub](https://github.com/framesurge/perseus)!

[repo]: https://github.com/framesurge/perseus
[crate]: https://crates.io/crates/perseus
[docs]: https://docs.rs/perseus
[contrib]: ./CONTRIBUTING.md
