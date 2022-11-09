use perseus::{Html, PerseusApp, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

// Initialize our app with the `perseus_warp` package's default server (fully
// customizable)
#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        // Create a new template at `index`, which maps to our landing page
        .template(|| {
            Template::new("index")
                .template(index_page)
                .build_state_fn(get_index_build_state)
        })
        .template(|| Template::new("about").template(about_page))
}

// EXCERPT_START
#[perseus::template]
fn index_page<'a, G: Html>(cx: Scope<'a>, props: IndexPropsRx<'a>) -> View<G> {
    view! { cx,
        h1 { (format!(
            "Hello, {}!",
            props.name.get()
        )) }
        input(
            placeholder = "Name",
            bind:value = props.name
        )
        a(href = "about") { "About" }
    }
}

#[perseus::make_rx(IndexPropsRx)]
struct IndexProps {
    name: String,
}

// This function will be run when you build your app, to generate default state
// ahead-of-time
#[perseus::build_state]
async fn get_index_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexProps> {
    let props = IndexProps {
        name: "User".to_string(),
    };
    Ok(props)
}
// EXCERPT_END

#[perseus::template]
fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "This is an example webapp created with Perseus!" }
    }
}
