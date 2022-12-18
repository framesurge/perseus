use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PostPageStateRx")]
struct PostPageState {
    title: String,
    content: String,
}

fn post_page<'a, 'b, G: Html>(cx: BoundedScope<'a, 'b>, props: &'b PostPageStateRx) -> View<G> {
    view! { cx,
        h1 {
            (props.title.get())
        }
        p {
            (props.content.get())
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
        .build()
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> PostPageState {
    // This is just an example
    let title = urlencoding::decode(&info.path).unwrap();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        title, info.path
    );

    PostPageState {
        title: title.to_string(),
        content,
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec![
            "".to_string(),
            "test".to_string(),
            "blah/test/blah".to_string(),
        ],
        // We're not using any extra helper state
        extra: ().into(),
    }
}
