use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// Initialize our app with the `perseus_warp` package's default server (fully
// customizable)
#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        // Create a new template at `index`, which maps to our landing page
        .template(
            Template::build("index")
                .view_with_state(index_page)
                .build_state_fn(get_build_state)
                .build(),
        )
        .template(Template::build("about").view(about_page).build())
}

#[auto_scope]
// EXCERPT_START
fn index_page<G: Html>(cx: Scope, state: &IndexStateRx) -> View<G> {
    view! { cx,
        h1 { (format!(
            "Hello, {}!",
            state.name.get()
        )) }
        input(
            placeholder = "Name",
            bind:value = state.name
        )
        a(href = "about") { "About" }
    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexStateRx")]
struct IndexState {
    name: String,
}

// This function will be run when you build your app, to generate default state
// ahead-of-time
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexState {
    IndexState {
        name: "User".to_string(),
    }
}
// EXCERPT_END

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "This is an example webapp created with Perseus!" }
    }
}
