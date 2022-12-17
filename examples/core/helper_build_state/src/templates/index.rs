use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

fn index_page<'a, 'b, G: Html>(cx: BoundedScope<'a, 'b>, state: PageStateRx<'b>) -> View<G> {
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

// This is our page state, so it does have to be either reactive or unreactive
#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    title: String,
    content: String,
}

// Notice that this doesn't need to be reactive/unreactive, since it's
// engine-only (note that this helper state is very simple, but you could have
// any `struct` here)
#[derive(Serialize, Deserialize)]
struct HelperState(String);

// In almost every other example, we use `StateGeneratorInfo<()>`, since that
// type parameter is the type of your helper state! (Make sure you don't confuse
// this with your *template* state!)
#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<HelperState>) -> PageState {
    let title = format!("Path: {}", &info.path);
    let content = format!(
        "This post's original slug was '{}'. Extra state: {}",
        &title,
        // We can't directly access `extra`, we use this function call
        info.get_extra().0,
    );

    PageState { title, content }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec![
            "".to_string(),
            "test".to_string(),
            "blah/test/blah".to_string(),
        ],
        // Behind the scenes, Perseus converts this into the magical `TemplateState` type,
        // which can handle *any* owned type you give it! Hence, we need to pop a `.into()`
        // on the end of this.
        extra: HelperState("extra helper state!".to_string()).into(),
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .template_with_state::<PageState, _>(index_page)
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
}
