use perseus::{link, RenderFnResult, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, Html, View};

#[derive(Serialize, Deserialize)]
pub struct PostPageProps {
    title: String,
    content: String,
}

#[perseus::template(PostPage)]
#[component(PostPage<G>)]
pub fn post_page(props: PostPageProps) -> View<G> {
    let title = props.title;
    let content = props.content;
    view! {
        h1 {
            (title)
        }
        p {
            (content)
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
) -> RenderFnResultWithCause<PostPageProps> {
    // This is just an example
    let title = urlencoding::decode(&path).unwrap();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        title, path
    );

    Ok(PostPageProps {
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
