use perseus::{RenderFnResultWithCause, Request, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize)]
pub struct IpPageProps {
    ip: String,
}

#[perseus::template(IpPage)]
#[component(IpPage<G>)]
pub fn ip_page(props: IpPageProps) -> SycamoreTemplate<G> {
    template! {
        p {
            (
                format!("Your IP address is {}.", props.ip)
            )
        }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("ip")
        .request_state_fn(get_request_state)
        .template(ip_page)
}

pub async fn get_request_state(
    _path: String,
    _locale: String,
    req: Request,
) -> RenderFnResultWithCause<String> {
    // Err(perseus::GenericErrorWithCause {
    //     error: "this is a test error!".into(),
    //     cause: perseus::ErrorCause::Client(None)
    // })
    Ok(serde_json::to_string(&IpPageProps {
        // Gets the client's IP address
        ip: format!(
            "{:?}",
            req.headers()
                .get("X-Forwarded-For")
                .unwrap_or(&perseus::http::HeaderValue::from_str("hidden from view!").unwrap())
        ),
    })?)
}
