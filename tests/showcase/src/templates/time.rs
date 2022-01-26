use perseus::{
    ErrorCause, GenericErrorWithCause, RenderFnResult, RenderFnResultWithCause, Template,
};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, Html, View};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimePageProps {
    pub time: String,
}

#[perseus::template(TimePage)]
#[component(TimePage<G>)]
pub fn time_page(props: TimePageProps) -> View<G> {
    view! {
        p { (format!("The time when this page was last rendered was '{}'.", props.time)) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("timeisr")
        .template(time_page)
        // This page will revalidate every five seconds (to illustrate revalidation)
        .revalidate_after("5s".to_string())
        .incremental_generation()
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    path: String,
    _locale: String,
) -> RenderFnResultWithCause<TimePageProps> {
    // This path is illegal, and can't be rendered
    if path == "timeisr/tests" {
        return Err(GenericErrorWithCause {
            error: "illegal page".into(),
            cause: ErrorCause::Client(Some(404)),
        });
    }
    Ok(TimePageProps {
        time: format!("{:?}", std::time::SystemTime::now()),
    })
}

pub async fn get_build_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec!["test".to_string()])
}
