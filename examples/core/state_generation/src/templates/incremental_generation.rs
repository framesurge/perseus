// This is exactly the same as the build paths example except for a few lines
// and some names

use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    title: String,
    content: String,
}

#[perseus::template]
fn incremental_generation_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
    let title = state.title;
    let content = state.content;
    view! { cx,
        h1 {
            (title.get())
        }
        p {
            (content.get())
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("incremental_generation")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        // This line makes Perseus try to render any given path under the template's root path
        // (`incremental_generation`) by putting it through `get_build_state` If you want to
        // filter the path because some are invalid (e.g. entries that aren't in some database), we
        // can filter them out at the state of the build state function
        .incremental_generation()
        .template_with_state(incremental_generation_page)
}

// This will be executed at build-time for all the paths in `get_build_paths()`,
// and then again for any other paths that a user might request while the app is
// live
#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> RenderFnResultWithCause<PageState> {
    // This path is illegal, and can't be rendered
    // Because we're using incremental generation, we could get literally anything
    // as the `path`
    if path == "incremental_generation/tests" {
        // This tells Perseus to return an error that's the client's fault, with the
        // HTTP status code 404 (not found) and the message 'illegal page'
        // You could return this error manually, but this is more convenient
        blame_err!(client, 404, "illegal page");
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
async fn get_build_paths() -> RenderFnResult<BuildPaths> {
    Ok(BuildPaths {
        paths: vec!["test".to_string(), "blah/test/blah".to_string()],
        extra: ().into(),
    })
}
