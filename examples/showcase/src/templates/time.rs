use perseus::{ErrorCause, StringResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimePageProps {
    pub time: String,
}

#[component(TimePage<G>)]
pub fn time_page(props: TimePageProps) -> SycamoreTemplate<G> {
    template! {
        p { (format!("The time when this page was last rendered was '{}'.", props.time)) }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("timeisr")
        .template(template_fn())
        // This page will revalidate every five seconds (to illustrate revalidation)
        .revalidate_after("5s".to_string())
        .incremental_generation()
        .build_state_fn(Rc::new(get_build_state))
        .build_paths_fn(Rc::new(get_build_paths))
}

pub async fn get_build_state(path: String) -> StringResultWithCause<String> {
    // This path is illegal, and can't be rendered
    if path == "timeisr/tests" {
        return Err(("illegal page".to_string(), ErrorCause::Client(Some(404))));
    }
    Ok(serde_json::to_string(&TimePageProps {
        time: format!("{:?}", std::time::SystemTime::now()),
    })
    .unwrap())
}

pub async fn get_build_paths() -> Result<Vec<String>, String> {
    Ok(vec!["test".to_string()])
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Rc::new(|props| {
        template! {
            TimePage(
                serde_json::from_str::<TimePageProps>(&props.unwrap()).unwrap()
            )
        }
    })
}
