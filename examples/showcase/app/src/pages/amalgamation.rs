use perseus::{Request, States, StringResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize, Debug)]
pub struct AmalagamationPageProps {
    pub message: String,
}

#[component(AboutPage<G>)]
pub fn about_page(props: AmalagamationPageProps) -> SycamoreTemplate<G> {
    template! {
        p { (format!("The message is: '{}'", props.message)) }
    }
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("amalgamation")
        .build_state_fn(Arc::new(get_build_state))
        .request_state_fn(Arc::new(get_request_state))
        .amalgamate_states_fn(Arc::new(amalgamate_states))
        .template(template_fn())
}

pub fn amalgamate_states(states: States) -> StringResultWithCause<Option<String>> {
    // We know they'll both be defined
    let build_state =
        serde_json::from_str::<AmalagamationPageProps>(&states.build_state.unwrap()).unwrap();
    let req_state =
        serde_json::from_str::<AmalagamationPageProps>(&states.request_state.unwrap()).unwrap();

    Ok(Some(
        serde_json::to_string(&AmalagamationPageProps {
            message: format!(
                "Hello from the amalgamation! (Build says: '{}', server says: '{}'.)",
                build_state.message, req_state.message
            ),
        })
        .unwrap(),
    ))
}

pub async fn get_build_state(_path: String) -> StringResultWithCause<String> {
    Ok(serde_json::to_string(&AmalagamationPageProps {
        message: "Hello from the build process!".to_string(),
    })
    .unwrap())
}

pub async fn get_request_state(_path: String, _req: Request) -> StringResultWithCause<String> {
    // Err(("this is a test error!".to_string(), perseus::ErrorCause::Client(None)))
    Ok(serde_json::to_string(&AmalagamationPageProps {
        message: "Hello from the server!".to_string(),
    })
    .unwrap())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Arc::new(|props, _| {
        template! {
            AboutPage(
                serde_json::from_str::<AmalagamationPageProps>(&props.unwrap()).unwrap()
            )
        }
    })
}
