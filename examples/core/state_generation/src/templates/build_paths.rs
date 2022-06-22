use perseus::{RenderFnResult, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, View};

#[perseus::make_rx(PageStateRx)]
pub struct PageState {
    title: String,
    content: String,
}

#[perseus::template_rx]
pub fn build_paths_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
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
    Template::new("build_paths")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .template(build_paths_page)
}

// We'll take in the path here, which will consist of the template name `build_paths` followed by the spcific path we're building for (as exported from `get_build_paths`)
#[perseus::build_state]
pub async fn get_build_state(path: String, _locale: String) -> RenderFnResultWithCause<PageState> {
    let title = path.clone();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        &title, &path
    );

    Ok(PageState { title, content })
}

// This just returns a vector of all the paths we want to generate for underneath `build_paths` (the template's name and root path)
// Like for build state, this function is asynchronous, so you could fetch these paths from a database or the like
// Note that everything you export from here will be prefixed with `<template-name>/` when it becomes a URL in your app
//
// Note also that there's almost no point in using build paths without build state, as every page would come out exactly the same (unless you differentiated them on the client...)
#[perseus::build_paths]
pub async fn get_build_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec![
        "".to_string(),
        "test".to_string(),
        "blah/test/blah".to_string(),
    ])
}
