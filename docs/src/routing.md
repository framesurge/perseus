# Routing

*You only need this page if you're not using the Perseus CLI, which performs this process for you! It does provide some useful background even so though.*

Perseus will serve your pages on the backend, rendered however you like, but it depends on [Sycamore](https://github.com/sycamore-rs/sycamore) for front-end rendering and routing, so you'll need to provide a router for your pages. You can see more information about Sycamore routing in their official documentation [here](https://sycamore-rs.netlify.app/docs/advanced/routing).

## Usage

You'll need to define a rout `enum` at the root of your app like so to define your app's routes:

```rust
use sycamore::prelude::*;

#[derive(Route)]
enum AppRoute {
    #[to("/")]
    Index,
    #[to("/about")]
    About,
    #[to("/post/new")]
    NewPost,
    #[to("/post/<slug..>")]
    Post { slug: Vec<String> },
    #[to("/ip")]
    Ip,
    #[to("/time")]
    TimeRoot,
    #[to("/timeisr/<slug>")]
    Time { slug: String },
    #[not_found]
    NotFound,
}
```

Note in the above example the usage of the `NewPost` template to override a section of the domain of the `Post` template, specifically the `/post/new` path, where a post writing page is hosted. Notably, such intrusive routes must be placed before. In general, **order your routes by specificity**. If you're not having troubles though, put them in any order you like (but `NotFound` must come last).

You can then match each of your routes and render it like so (subset of the previous example):

```rust,no_run,no_plyaground
let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#_perseus_root")
        .unwrap()
        .unwrap();

sycamore::render_to(
        || {
            template! {
                BrowserRouter(|route: AppRoute| {
                    match route {
                        AppRoute::Index => app_shell(
                            "index".to_string(),
                            pages::index::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::About => app_shell(
                            "about".to_string(),
                            pages::about::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::Post { slug } => app_shell(
                            format!("post/{}", slug.join("/")),
                            pages::post::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::NotFound => template! {
                            p {"Not Found."}
                        }
                    }
                })
            }
        },
        &root,
    );
```

Note that you pass your error pages to the app shell, allowing it to conditionally render them if need be. Also note the template function being reused for the router as well as in the template itself.

The router is the core of your app, and should be rendered to a location from which you'll use Perseus. Perseus is a full framework for rendering, so if you want incremental adoption of reactivity, you should check out the underlying [Sycamore](https://github.com/sycamore-rs/sycamore) library.
