use perseus::{RenderFnResultWithCause, Template};
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
    Template::new("time")
        .template(Rc::new(|props| {
            template! {
                TimePage(
                    serde_json::from_str::<TimePageProps>(&props.unwrap()).unwrap()
                )
            }
        }))
        // This page will revalidate every five seconds (to illustrate revalidation)
        // Try changing this to a week, even though the below custom logic says to always revalidate, we'll only do it weekly
        .revalidate_after("5s".to_string())
        .should_revalidate_fn(Rc::new(|| async { Ok(true) }))
        .build_state_fn(Rc::new(get_build_state))
}

pub async fn get_build_state(_path: String) -> RenderFnResultWithCause<String> {
    Ok(serde_json::to_string(&TimePageProps {
        time: format!("{:?}", std::time::SystemTime::now()),
    })?)
}
