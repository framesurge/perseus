#[cfg(not(target_arch = "wasm32"))]
use perseus::Request;
use perseus::{RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, View};

#[perseus::make_rx(PageStateRx)]
pub struct PageState {
    ip: String,
}

#[perseus::template_rx]
pub fn request_state_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
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
        .template(request_state_page)
}

#[perseus::request_state]
pub async fn get_request_state(
    _path: String,
    _locale: String,
    // Unlike in build state, in request state we get access to the information that the user sent with their HTTP request
    // IN this example, we extract the browser's reporting of their IP address and display it to them
    req: Request,
) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        ip: format!(
            "{:?}",
            req.headers()
                // NOTE: This header can be trivially spoofed, and may well not be the user's actual IP address
                .get("X-Forwarded-For")
                .unwrap_or(&perseus::http::HeaderValue::from_str("hidden from view!").unwrap())
        ),
    })
}
