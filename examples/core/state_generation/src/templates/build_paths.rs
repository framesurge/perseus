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
fn build_paths_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
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
        .template_with_state(build_paths_page)
}

// We take in `StateGeneratorInfo`, which has the path we're generating for
// (*not* including the template name), along with the locale, and some
// arbitrary helper state (which we're not using, hence the `()`)
async fn get_build_state(info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<PageState> {
    let title = info.path.clone();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        &title, &info.path
    );

    Ok(PageState { title, content })
}

// This just returns a special `struct` containing all the paths we want to
// generate underneath `build_paths` (the template's name and root path). Like
// for build state, this function is asynchronous, so you could fetch these
// paths from a database or the like Note that everything you export from here
// will be prefixed with `<template-name>/` when it becomes a URL in your app
//
// Note also that there's almost no point in using build paths without build
// state, as every page would come out exactly the same (unless you
// differentiated them on the client...)
async fn get_build_paths() -> RenderFnResult<BuildPaths> {
    Ok(BuildPaths {
        // These are the paths we want to generate for, with an empty string being at the root of
        // the template name (here, `/build_paths`)
        paths: vec![
            "".to_string(),
            "test".to_string(),
            "blah/test/blah".to_string(),
            "a test".to_string(), // Perseus can even handle paths with special characters!
        ],
        // Sometimes, you want to do something once to generate some helper state for building each
        // page, and you can put literally anything in here (but we're not using it).
        // The `.into()` makes sure Perseus can understand whatever we put in here.
        extra: ().into(),
    })
}
