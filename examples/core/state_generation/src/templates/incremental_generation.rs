// This is exactly the same as the build paths example except for a few lines and some names

use perseus::{blame_err, RenderFnResult, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, View};

#[perseus::make_rx(PageStateRx)]
pub struct PageState {
    title: String,
    content: String,
}

#[perseus::template_rx(IncrementalGenerationPage)]
pub fn incremental_generation_page(state: PageStateRx) -> View<G> {
    let title = state.title;
    let content = state.content;
    view! {
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
        // This line makes Perseus try to render any given path under the template's root path (`incremental_generation`) by putting it through `get_build_state`
        // If you want to filter the path because some are invalid (e.g. entries that aren't in some database), we can filter them out at the state of the build state function
        .incremental_generation()
        .template(incremental_generation_page)
}

// We'll take in the path here, which will consist of the template name `incremental_generation` followed by the spcific path we're building for (as exported from `get_build_paths`)
#[perseus::autoserde(build_state)]
pub async fn get_build_state(path: String, _locale: String) -> RenderFnResultWithCause<PageState> {
    // This path is illegal, and can't be rendered
    // Because we're using incremental generation, we could gte literally anything as the `path`
    if path == "incremental_generation/tests" {
        // This tells Perseus to return an error that's the client's fault, with the HTTP status code 404 (not found) and the message 'illegal page'
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

// This just returns a vector of all the paths we want to generate for underneath `incremental_generation` (the template's name and root path)
// Like for build state, this function is asynchronous, so you could fetch these paths from a database or the like
// Note that everything you export from here will be prefixed with `<template-name>/` when it becomes a URL in your app
//
// Note also that there's almost no point in using build paths without build state, as every page would come out exactly the same (unless you differentiated them on the client...)
pub async fn get_build_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec!["test".to_string(), "blah/test/blah".to_string()])
}
