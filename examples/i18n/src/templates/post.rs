use perseus::{link, RenderFnResult, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize)]
pub struct PostPageProps {
    title: String,
    content: String,
}

#[component(PostPage<G>)]
pub fn post_page(props: PostPageProps) -> SycamoreTemplate<G> {
    let title = props.title;
    let content = props.content;
    template! {
        h1 {
            (title)
        }
        p {
            (content)
        }
        a(href = link!("/post")) { "Root post page" }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("post")
        .build_paths_fn(Rc::new(get_static_paths))
        .build_state_fn(Rc::new(get_static_props))
        .template(Rc::new(|props| {
            template! {
                PostPage(
                    serde_json::from_str::<PostPageProps>(&props.unwrap()).unwrap()
                )
            }
        }))
}

pub async fn get_static_props(path: String, _locale: String) -> RenderFnResultWithCause<String> {
    // This is just an example
    let title = urlencoding::decode(&path).unwrap();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        title, path
    );

    Ok(serde_json::to_string(&PostPageProps {
        title: title.to_string(),
        content,
    })?) // This `?` declares the default, that the server is the cause of the error
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec![
        "".to_string(),
        "test".to_string(),
        "blah/test/blah".to_string(),
    ])
}
