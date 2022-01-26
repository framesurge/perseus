use perseus::{RenderFnResultWithCause, Request, States, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, Html, View};

#[derive(Serialize, Deserialize, Debug)]
pub struct AmalagamationPageProps {
    pub message: String,
}

#[perseus::template(AmalgamationPage)]
#[component(AmalgamationPage<G>)]
pub fn amalgamation_page(props: AmalagamationPageProps) -> View<G> {
    view! {
        p { (format!("The message is: '{}'", props.message)) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("amalgamation")
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
        .amalgamate_states_fn(amalgamate_states)
        .template(amalgamation_page)
}

#[perseus::autoserde(amalgamate_states)]
pub fn amalgamate_states(
    states: States,
) -> RenderFnResultWithCause<Option<AmalagamationPageProps>> {
    // We know they'll both be defined
    let build_state = serde_json::from_str::<AmalagamationPageProps>(&states.build_state.unwrap())?;
    let req_state = serde_json::from_str::<AmalagamationPageProps>(&states.request_state.unwrap())?;

    Ok(Some(AmalagamationPageProps {
        message: format!(
            "Hello from the amalgamation! (Build says: '{}', server says: '{}'.)",
            build_state.message, req_state.message
        ),
    }))
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<AmalagamationPageProps> {
    Ok(AmalagamationPageProps {
        message: "Hello from the build process!".to_string(),
    })
}

#[perseus::autoserde(request_state)]
pub async fn get_request_state(
    _path: String,
    _locale: String,
    _req: Request,
) -> RenderFnResultWithCause<AmalagamationPageProps> {
    // Err(perseus::GenericErrorWithCause {
    //     error: "this is a test error!".into(),
    //     cause: perseus::ErrorCause::Client(None)
    // })
    Ok(AmalagamationPageProps {
        message: "Hello from the server!".to_string(),
    })
}
