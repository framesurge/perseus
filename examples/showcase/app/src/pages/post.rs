use perseus::{ErrorCause, StringResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
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
    }
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("post")
        .build_paths_fn(Arc::new(get_static_paths))
        .build_state_fn(Arc::new(get_static_props))
        .incremental_path_rendering(true)
        .template(template_fn())
}

pub async fn get_static_props(path: String) -> StringResultWithCause<String> {
    // This path is illegal, and can't be rendered
    if path == "post/tests" {
        return Err(("illegal page".to_string(), ErrorCause::Client(Some(404))));
    }
    // This is just an example
    let title = urlencoding::decode(&path).unwrap();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        title, path
    );

    Ok(serde_json::to_string(&PostPageProps {
        title: title.to_string(),
        content,
    })
    .unwrap())
}

pub async fn get_static_paths() -> Result<Vec<String>, String> {
    Ok(vec!["test".to_string(), "blah/test/blah".to_string()])
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Arc::new(|props, _| {
        template! {
            PostPage(
                serde_json::from_str::<PostPageProps>(&props.unwrap()).unwrap()
            )
        }
    })
}
