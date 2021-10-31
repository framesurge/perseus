use perseus::{
    ErrorCause, GenericErrorWithCause, RenderFnResult, RenderFnResultWithCause, Template,
};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize)]
pub struct PostPageProps {
    title: String,
    content: String,
}

#[perseus::template(PostPage)]
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

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("post")
        .build_paths_fn(get_static_paths)
        .build_state_fn(get_static_props)
        .incremental_generation()
        .template(post_page)
}

pub async fn get_static_props(path: String, _locale: String) -> RenderFnResultWithCause<String> {
    // This path is illegal, and can't be rendered
    if path == "post/tests" {
        return Err(GenericErrorWithCause {
            error: "illegal page".into(),
            cause: ErrorCause::Client(Some(404)),
        });
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
    })?)
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec!["test".to_string(), "blah/test/blah".to_string()])
}
