// This page illustrates SSR

use perseus::errors::ErrorCause;
use perseus::template::{Template};
use perseus::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize)]
pub struct IpPageProps {
    ip: String,
}

#[component(IpPage<G>)]
pub fn dashboard_page(props: IpPageProps) -> SycamoreTemplate<G> {
    template! {
        p {
            (
                format!("Your IP address is {}.", props.ip)
            )
        }
    }
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("ip")
        .request_state_fn(Box::new(get_request_state))
        .template(template_fn())
}

pub async fn get_request_state(_path: String, req: Request) -> Result<String, (String, ErrorCause)> {
    // Err(("this is a test error!".to_string(), ErrorCause::Client(None)))
    Ok(serde_json::to_string(&IpPageProps {
        // Gets the client's IP address
        ip: format!(
            "{:?}",
            req
                .headers()
                .get("X-Forwarded-For")
                .unwrap_or(&perseus::http::HeaderValue::from_str("hidden from view!").unwrap())
        ),
    })
    .unwrap())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|props: Option<String>| {
        template! {
            IpPage(
                serde_json::from_str::<IpPageProps>(&props.unwrap()).unwrap()
            )
        }
    })
}
