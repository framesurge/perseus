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
                .template_with_state(index_page)
                .build_state_fn(get_index_build_state),
        )
        .template(Template::new("about").template(about_page))
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

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "IndexPropsRx")]
struct IndexProps {
    name: String,
}

// This function will be run when you build your app, to generate default state
// ahead-of-time
#[engine_only_fn]
async fn get_index_build_state(
    _info: StateGeneratorInfo<()>,
) -> RenderFnResultWithCause<IndexProps> {
    let props = IndexProps {
        name: "User".to_string(),
    };
    Ok(props)
}
// EXCERPT_END

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "This is an example webapp created with Perseus!" }
    }
}
