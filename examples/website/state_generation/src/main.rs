use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::time::Duration;
use sycamore::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(
        Template::build("post")
            .view_with_state(post_page)
            .build_paths_fn(get_build_paths)
            .build_state_fn(get_build_state)
            // Reload every blog post every day, in case it's changed
            .revalidate_after(Duration::new(60 * 60 * 24, 0))
            // If the user requests a page we haven't created yet, still
            // pass it to `get_build_state()` and cache the output for
            // future users (lazy page building)
            .incremental_generation()
            .build(),
    )
}

#[auto_scope]
// EXCERPT_START
fn post_page<G: Html>(cx: Scope, state: &PostRx) -> View<G> {
    view! { cx,
        h1 { (state.title.get()) }
        p { (state.author.get()) }
        div(
            dangerously_set_inner_html = &state.content.get()
        )
    }
}
// EXCERPT_END

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PostRx")]
struct Post {
    title: String,
    author: String,
    content: String,
}

// EXCERPT_START
// This function will be run for each path under `/post/` to generate its state
#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> Result<Post, BlamedError<MyError>> {
    let raw_post = match get_post_for_path(path) {
        Ok(post) => post,
        // If the user sends us some bogus path with incremental generation,
        // return a 404 appropriately
        Err(err) => {
            return Err(BlamedError {
                blame: ErrorBlame::Client(Some(404)),
                error: MyError(err),
            })
        }
    };
    let html_content = parse_markdown(raw_post.content);
    let post = Post {
        title: raw_post.title,
        author: raw_post.author,
        content: html_content,
    };
    Ok(post)
}
#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        // These will all become URLs at `/post/<name>`
        paths: vec![
            "welcome".to_string(),
            "really-popular-post".to_string(),
            "foobar".to_string(),
        ],
        // Perseus supports helper state, but we don't need it here
        extra: ().into(),
    }
}
// EXCERPT_END

// SNIP
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
struct MyError(io::Error);
fn get_post_for_path(_path: String) -> Result<Post, io::Error> {
    unimplemented!()
}
fn parse_markdown(_content: String) -> String {
    unimplemented!()
}
