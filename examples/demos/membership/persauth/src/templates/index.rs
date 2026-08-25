use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ApiResponse<T = ()> {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserProfile {
    id: i32,
    email: String,
    created_at: Option<String>,
    otp_confirmed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Session {
    session_id: i32,
    session_verifier: String,
}

#[cfg(client)]
use sycamore::futures::spawn_local;

fn index_page() -> View {
    let is_logged_in = create_signal(false);
    let user_email = create_signal(String::new());
    let loading = create_signal(true);
    let message = create_signal(String::new());
    let message_type = create_signal(String::new());

    // Check login status on mount
    #[cfg(client)]
    {
        let is_logged_in = is_logged_in.clone();
        let user_email = user_email.clone();
        let loading = loading.clone();

        spawn_local(async move {
            if let Some(session) = get_stored_session() {
                match get_profile(&session).await {
                    Ok(response) => {
                        if response.success {
                            if let Some(profile) = response.data {
                                is_logged_in.set(true);
                                user_email.set(profile.email);
                            }
                        }
                    }
                    Err(_) => {
                        // Clear invalid session
                        clear_session();
                    }
                }
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        loading.set(false);
    }

    let handle_logout = move |_| {
        #[cfg(client)]
        spawn_local(async move {
            if let Some(session) = get_stored_session() {
                let _ = logout(&session).await;
            }
            clear_session();
            is_logged_in.set(false);
            user_email.set(String::new());
            message.set("Logged out successfully".to_string());
            message_type.set("success".to_string());
        });
    };

    view! {
        div(class = "container") {
            div(class = "card") {
                h1 { "Perseus Membership System" }

                (if loading.get() {
                    view! {
                        p(style = "text-align: center; color: #666;") { "Loading..." }
                    }
                } else if is_logged_in.get() {
                    let email = user_email.get_clone();
                    view! {
                        div(style = "text-align: center;") {
                            p(style = "margin-bottom: 1.5rem; font-size: 1.1rem;") {
                                "Welcome, "
                                strong { (email) }
                            }

                            // Dashboard Navigation
                            div(style = "margin-bottom: 2rem;") {
                                h3(style = "margin-bottom: 1rem; color: #333;") { "Dashboard" }
                                div(style = "display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 1rem; max-width: 600px; margin: 0 auto;") {
                                    a(
                                        href = "/posts",
                                        style = "display: block; padding: 1rem; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border-radius: 8px; text-decoration: none; font-weight: 600; text-align: center;"
                                    ) {
                                        "Posts"
                                    }
                                    a(
                                        href = "/categories",
                                        style = "display: block; padding: 1rem; background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%); color: white; border-radius: 8px; text-decoration: none; font-weight: 600; text-align: center;"
                                    ) {
                                        "Categories"
                                    }
                                    a(
                                        href = "/tags",
                                        style = "display: block; padding: 1rem; background: linear-gradient(135deg, #ee0979 0%, #ff6a00 100%); color: white; border-radius: 8px; text-decoration: none; font-weight: 600; text-align: center;"
                                    ) {
                                        "Tags"
                                    }
                                    a(
                                        href = "/profile",
                                        style = "display: block; padding: 1rem; background: linear-gradient(135deg, #536976 0%, #292E49 100%); color: white; border-radius: 8px; text-decoration: none; font-weight: 600; text-align: center;"
                                    ) {
                                        "Profile"
                                    }
                                }
                            }

                            button(
                                on:click = handle_logout,
                                style = "max-width: 200px; margin: 0 auto;"
                            ) {
                                "Sign Out"
                            }
                        }
                    }
                } else {
                    view! {
                        div(style = "text-align: center;") {
                            p(style = "margin-bottom: 2rem; color: #666; font-size: 1.1rem;") {
                                "A complete authentication system built with Perseus and PostgreSQL"
                            }

                            div(style = "display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap;") {
                                a(
                                    href = "/register",
                                    style = "display: inline-block; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 0.875rem 2rem; border-radius: 8px; text-decoration: none; font-weight: 600;"
                                ) {
                                    "Create Account"
                                }
                                a(
                                    href = "/login",
                                    style = "display: inline-block; background: transparent; color: #667eea; padding: 0.875rem 2rem; border: 2px solid #667eea; border-radius: 8px; text-decoration: none; font-weight: 600;"
                                ) {
                                    "Sign In"
                                }
                            }
                        }

                        div(style = "margin-top: 3rem; padding-top: 2rem; border-top: 1px solid #eee;") {
                            h2(style = "font-size: 1.2rem; margin-bottom: 1rem;") { "Features" }
                            ul(style = "list-style: none; color: #666;") {
                                li(style = "padding: 0.5rem 0;") { "User registration with email verification" }
                                li(style = "padding: 0.5rem 0;") { "Secure password hashing (Argon2/bcrypt)" }
                                li(style = "padding: 0.5rem 0;") { "OTP-based email confirmation" }
                                li(style = "padding: 0.5rem 0;") { "Password reset via email" }
                                li(style = "padding: 0.5rem 0;") { "Session management" }
                            }
                        }
                    }
                })

                (if !message.get_clone().is_empty() {
                    let msg = message.get_clone();
                    let class_name = format!("message {}", message_type.get_clone());
                    view! {
                        div(class = class_name) {
                            (msg)
                        }
                    }
                } else {
                    view! {}
                })
            }
        }
    }
}

#[cfg(client)]
fn get_stored_session() -> Option<Session> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let session_id = storage.get_item("session_id").ok()??;
    let session_verifier = storage.get_item("session_verifier").ok()??;

    Some(Session {
        session_id: session_id.parse().ok()?,
        session_verifier,
    })
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
async fn get_profile(session: &Session) -> Result<ApiResponse<UserProfile>, String> {
    use gloo_net::http::Request;

    let response = Request::post("/auth/profile")
        .json(&serde_json::json!({
            "session_id": session.session_id,
            "session_verifier": session.session_verifier
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<ApiResponse<UserProfile>>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(client)]
async fn logout(session: &Session) -> Result<ApiResponse, String> {
    use gloo_net::http::Request;

    let response = Request::post("/auth/logout")
        .json(&serde_json::json!({
            "session_id": session.session_id,
            "session_verifier": session.session_verifier
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<ApiResponse>()
        .await
        .map_err(|e| e.to_string())
}

pub fn get_template() -> Template {
    Template::build("index").view(index_page).build()
}
