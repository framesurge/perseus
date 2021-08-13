use sycamore::prelude::{template, component, GenericNode, Template as SycamoreTemplate};
use perseus::template::Template;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimePageProps {
    pub time: String
}

#[component(TimePage<G>)]
pub fn time_page(props: TimePageProps) -> SycamoreTemplate<G> {
	template! {
		p { (format!("The time when this page was last rendered was '{}'.", props.time)) }
	}
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("timeisr")
        .template(template_fn())
        // This page will revalidate every five seconds (to illustrate revalidation)
		.revalidate_after("5s".to_string())
        .incremental_path_rendering(true)
        .build_state_fn(Box::new(get_build_state))
        .build_paths_fn(Box::new(get_build_paths))
}

pub fn get_build_state(_path: String) -> Result<String, String> {
    Ok(serde_json::to_string(
        &TimePageProps {
            time: format!("{:?}", std::time::SystemTime::now())
        }
    ).unwrap())
}

pub fn get_build_paths() -> Result<Vec<String>, String> {
    Ok(vec![
        "test".to_string()
    ])
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|props: Option<String>| template! {
            TimePage(
                serde_json::from_str::<TimePageProps>(&props.unwrap()).unwrap()
            )
        }
    )
}