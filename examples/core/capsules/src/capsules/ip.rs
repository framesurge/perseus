use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

lazy_static! {
    pub static ref IP: Capsule<PerseusNodeType, ()> = get_capsule();
}

// Note the use of props as `()`, indicating that this capsule doesn't take any
// properties
fn ip_capsule<G: Html>(cx: Scope, state: IpState, _props: ()) -> View<G> {
    view! { cx,
        p(id = "ip") { (state.ip) }
    }
}

// This uses unreactive state, just to show that it works
#[derive(Serialize, Deserialize, Clone, UnreactiveState)]
struct IpState {
    ip: String,
}

pub fn get_capsule<G: Html>() -> Capsule<G, ()> {
    Capsule::build(Template::build("ip").request_state_fn(get_request_state))
        .empty_fallback()
        // Very importantly, we declare our views on the capsule, **not** the template!
        // This lets us use properties.
        .view_with_unreactive_state(ip_capsule)
        .build()
}

#[engine_only_fn]
async fn get_request_state(_info: StateGeneratorInfo<()>, req: Request) -> IpState {
    IpState {
        ip: format!(
            "{:?}",
            req.headers()
                // NOTE: This header can be trivially spoofed, and may well not be the user's actual
                // IP address
                .get("X-Forwarded-For")
                .unwrap_or(&perseus::http::HeaderValue::from_str("hidden from view!").unwrap())
        ),
    }
}
