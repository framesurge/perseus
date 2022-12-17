use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    ip: String,
}

fn request_state_page<'a, 'b, G: Html>(cx: BoundedScope<'a, 'b>, state: PageStateRx<'b>) -> View<G> {
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
        .template_with_state::<PageState, _>(request_state_page)
}

// This returns a `Result<T, BlamedError<E>>` (or just `T`) because, obviously,
// it will be run at request-time: any errors could be a mising file (our fault),
// or a malformed cookie (the client's fault), etc., so we have to note the blame
// to get an accurate HTTP status code. This example is really infallible, but
// we've spelled it all out rather than using `T` so you can see how it works.
#[engine_only_fn]
async fn get_request_state(
    // We get all the same info as build state in here
    _info: StateGeneratorInfo<()>,
    // Unlike in build state, in request state we *also* get access to the information that the
    // user sent with their HTTP request IN this example, we extract the browser's reporting of
    // their IP address and display it to them
    req: Request,
) -> Result<PageState, BlamedError<std::convert::Infallible>> {
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
