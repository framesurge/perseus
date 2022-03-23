use perseus::{link, RenderFnResult, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, View};

#[perseus::make_rx(PostPageStateRx)]
pub struct PostPageState {
    title: String,
    content: String,
}

#[perseus::template_rx]
pub fn post_page(props: PostPageStateRx) -> View<G> {
    let title = props.title;
    let content = props.content;
    view! {
        h1 {
            (title.get())
        }
        p {
            (content.get())
        }
        a(href = link!("/post")) { "Root post page" }
        br()
        a(href = link!("/post/blah/test/blah")) { "Complex post page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("post")
        .build_paths_fn(get_static_paths)
        .build_state_fn(get_static_props)
        .template(post_page)
}

#[perseus::autoserde(build_state)]
pub async fn get_static_props(
    path: String,
    _locale: String,
) -> RenderFnResultWithCause<PostPageState> {
    // This is just an example
    let title = urlencoding::decode(&path).unwrap();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        title, path
    );

    Ok(PostPageState {
        title: title.to_string(),
        content,
    }) // This `?` declares the default, that the server is the cause of the error
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec![
        "".to_string(),
        "test".to_string(),
        "blah/test/blah".to_string(),
    ])
}
