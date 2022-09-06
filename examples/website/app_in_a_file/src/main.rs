use perseus::{ErrorPages, Html, PerseusApp, Template, RenderFnResultWithCause};
use sycamore::prelude::*;

// Initialize our app with the `perseus_warp` package's default server (fully customizable)
#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        // Create a new template at `index`, which maps to our landing page
        .template(
            Template::new("index")
                .template(index_page)
                // It will generate state with `get_index_build_state` when you build your app
                .build_state_fn(get_index_build_state)
        )
}

// Our app's landing page (takes in a render scope and some properties, which we can generate)
#[perseus::template_rx]
fn index_page<'a, G: Html>(cx: Scope<'a>, props: IndexPropsRx<'a>) -> View<G> {
    // Output a view, which will be converted to HTML ahead-of-time and hydrated
    view! { cx,
        // Greet the user
        h1 { (format!(
            "Hello, {}!",
            props.name.get()
        )) }
        // Let them specify what their name is
        input(
            placeholder = "Name",
            bind:value = props.name
        )
    }
}

// The index page will take this as state, which it can modify at runtime reactively
#[perseus::make_rx(IndexPropsRx)]
struct IndexProps {
    name: String,
}

// This function will be run when you build your app, to generate default state ahead-of-time
#[perseus::build_state]
async fn get_index_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<IndexProps> {
    let props = IndexProps {
        // If the user hasn't given their name yet, say `Hello, User!`
        name: "User".to_string(),
    };
    Ok(props)
}

// A simple about page that takes no state
#[perseus::template_rx]
fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "This is an example webapp created with Perseus!" }
    }
}
