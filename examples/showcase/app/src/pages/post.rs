use sycamore::prelude::*;
use serde::{Serialize, Deserialize};
use perseus::page::Page;

#[derive(Serialize, Deserialize)]
pub struct PostPageProps {
    title: String,
    content: String,
}

#[component(PostPage<G>)]
pub fn post_page(props: PostPageProps) -> Template<G> {
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

pub fn get_page<G: GenericNode>() -> Page<PostPageProps, G> {
    Page::new("post")
        .build_paths_fn(Box::new(get_static_paths))
        .build_state_fn(Box::new(get_static_props))
        .incremental_path_rendering(true)
        .template(Box::new(|props: Option<PostPageProps>| template! {
            PostPage(props.unwrap())
        }))
}

pub fn get_static_props(path: String) -> PostPageProps {
    let path_vec: Vec<&str> = path.split('/').collect();
    let title_slug = path_vec[0];
    // This is just an example
    let title = urlencoding::decode(title_slug).unwrap();
    let content = format!("This is a post entitled '{}'. Its original slug was '{}'.", title, title_slug);

    PostPageProps {
        title: title.to_string(),
        content
    }
}
// TODO
pub fn get_static_paths() -> Vec<String> {
    vec![
        "test".to_string()
    ]
}
