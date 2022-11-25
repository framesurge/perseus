use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PostPageStateRx")]
struct PostPageState {
    title: String,
    content: String,
}

#[perseus::template]
fn post_page<'a, G: Html>(cx: Scope<'a>, props: PostPageStateRx<'a>) -> View<G> {
    let title = props.title;
    let content = props.content;
    view! { cx,
        h1 {
            (title.get())
        }
        p {
            (content.get())
        }
        a(href = link!("/post", cx)) { "Root post page" }
        br()
        a(href = link!("/post/blah/test/blah", cx)) { "Complex post page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("post")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .template_with_state(post_page)
}

async fn get_build_state(info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<PostPageState> {
    // This is just an example
    let title = urlencoding::decode(&info.path).unwrap();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        title, info.path
    );

    Ok(PostPageState {
        title: title.to_string(),
        content,
    })
}

async fn get_build_paths() -> RenderFnResult<BuildPaths> {
    Ok(BuildPaths {
        paths: vec![
            "".to_string(),
            "test".to_string(),
            "blah/test/blah".to_string(),
        ],
        // We're not using any extra helper state
        extra: ().into(),
    })
}
