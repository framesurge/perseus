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
    Template::new("time")
        .template(template_fn())
        // This page will revalidate every five seconds (to illustrate revalidation)
        // Try changing this to a week, even though the below custom logic says to always revalidate, we'll only do it weekly
		.revalidate_after("5s".to_string())
        .should_revalidate_fn(Box::new(|| async {
            Ok(true)
        }))
        .build_state_fn(Box::new(get_build_state))
}

pub async fn get_build_state(_path: String) -> Result<String, String> {
    Ok(serde_json::to_string(
        &TimePageProps {
            time: format!("{:?}", std::time::SystemTime::now())
        }
    ).unwrap())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|props: Option<String>| template! {
            TimePage(
                serde_json::from_str::<TimePageProps>(&props.unwrap()).unwrap()
            )
        }
    )
}