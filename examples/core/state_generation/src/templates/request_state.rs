use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    ip: String,
}

#[perseus::template]
fn request_state_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
    view! { cx,
        p {
            (
                format!("Your IP address is {}.", state.ip.get())
            )
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("request_state")
        .request_state_fn(get_request_state)
        .template_with_state(request_state_page)
}

#[engine_only_fn]
async fn get_request_state(
    // We get all the same info as build state in here
    _info: StateGeneratorInfo<()>,
    // Unlike in build state, in request state we *also* get access to the information that the
    // user sent with their HTTP request IN this example, we extract the browser's reporting of
    // their IP address and display it to them
    req: Request,
) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        ip: format!(
            "{:?}",
            req.headers()
                // NOTE: This header can be trivially spoofed, and may well not be the user's actual
                // IP address
                .get("X-Forwarded-For")
                .unwrap_or(&perseus::http::HeaderValue::from_str("hidden from view!").unwrap())
        ),
    })
}
