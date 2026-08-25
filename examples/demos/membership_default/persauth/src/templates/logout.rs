use perseus::prelude::*;
use sycamore::prelude::*;

use crate::types::{ApiResponse, SessionData};

#[cfg(client)]
use sycamore::futures::spawn_local;

#[cfg(client)]
fn get_stored_session() -> Option<SessionData> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let session_id = storage.get_item("session_id").ok()??;
    let session_verifier = storage.get_item("session_verifier").ok()??;
    Some(SessionData { session_id: session_id.parse().ok()?, session_verifier })
}

#[cfg(client)]
fn clear_session() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("session_id");
            let _ = storage.remove_item("session_verifier");
        }
    }
}

#[cfg(client)]
async fn logout(session: &SessionData) -> Result<ApiResponse, String> {
    use gloo_net::http::Request;

    let response = Request::post("/api/auth/logout")
        .json(&serde_json::json!({
            "session_id": session.session_id,
            "session_verifier": session.session_verifier,
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json::<ApiResponse>().await.map_err(|e| e.to_string())
}

fn logout_page() -> View {
    let message = create_signal("Signing you out...".to_string());

    #[cfg(client)]
    {
        let message = message.clone();
        spawn_local(async move {
            if let Some(session) = get_stored_session() {
                let _ = logout(&session).await;
            }
            clear_session();
            message.set("Signed out. Redirecting...".to_string());
            navigate("/");
        });
    }

    view! {
        div(class = "page-body") {
            div(class = "container-fluid") {
                div(class = "card") {
                    div(class = "card-body") {
                        div(class = "loader-box") { div(class = "loader") { } }
                        p(style = "margin-top: 1rem;") { (message.get_clone()) }
                        Link(to = "/", class = "btn btn-primary", style = "margin-top: 1rem;") { "Go Home" }
                    }
                }
            }
        }
    }
}

pub fn get_template() -> Template {
    Template::build("logout").view(logout_page).build()
}
