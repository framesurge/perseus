// This is exactly the same as the build paths example except for a few lines
// and some names

use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    title: String,
    content: String,
}

fn incremental_generation_page<'a, G: Html>(
    cx: BoundedScope<'_, 'a>,
    state: &'a PageStateRx,
) -> View<G> {
    view! { cx,
        h1 {
            (state.title.get())
        }
        p {
            (state.content.get())
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("incremental_generation")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        // This line makes Perseus try to render any given path under the template's root path
        // (`incremental_generation`) by putting it through `get_build_state` If you want to
        // filter the path because some are invalid (e.g. entries that aren't in some database), we
        // can filter them out at the state of the build state function
        .incremental_generation()
        .view_with_state(incremental_generation_page)
        .build()
}

// This will be executed at build-time for all the paths in `get_build_paths()`,
// and then again for any other paths that a user might request while the app is
// live, meaning any errors could come from either the server or the client,
// hence why this returns a `BlamedError`. We use a `std::io::Error` because we
// need soemthing that implements `std::error::Error`, but you could use
// anything here.
#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> Result<PageState, BlamedError<std::io::Error>> {
    // This path is illegal, and can't be rendered
    // Because we're using incremental generation, we could get literally anything
    // as the `path`
    if path == "tests" {
        // This tells Perseus to return an error that's the client's fault, with the
        // HTTP status code 404 (not found) and the message 'illegal page'. Note that
        // this is a `BlamedError<String>`, but we could use any error type that
        // implements `std::error::Error` (note that this does make `anyhow` a
        // bit tricky, if you use it).
        return Err(BlamedError {
            // If we used `None` instead, it would default to 400 for the client and 500 for the
            // server
            blame: ErrorBlame::Client(Some(404)),
            // This is just an example, and you could put any error type here, usually your own
            error: std::io::Error::new(std::io::ErrorKind::NotFound, "illegal page"),
        });
    }
    let title = path.clone();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        &title, &path
    );

    Ok(PageState { title, content })
}

// See `../build_paths.rs` for an explanation of this
#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["test".to_string(), "blah/test/blah".to_string()],
        extra: ().into(),
    }
}
