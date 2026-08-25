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
struct Session {
    session_id: i32,
    session_verifier: String,
}

#[cfg(client)]
use sycamore::futures::spawn_local;

fn login_page() -> View {
    let email = create_signal(String::new());
    let password = create_signal(String::new());
    let message = create_signal(String::new());
    let message_type = create_signal(String::new());
    let submitting = create_signal(false);

    // Password reset state
    let show_reset = create_signal(false);
    let reset_email = create_signal(String::new());

    let submit_login = move |_| {
        let email_val = email.get_clone();
        let password_val = password.get_clone();

        if email_val.is_empty() || password_val.is_empty() {
            message.set("Please fill in all fields".to_string());
            message_type.set("error".to_string());
            return;
        }

        submitting.set(true);
        message.set(String::new());

        #[cfg(client)]
        spawn_local(async move {
            match login_user(&email_val, &password_val).await {
                Ok(response) => {
                    if response.success {
                        if let Some(session) = response.data {
                            // Store session in localStorage
                            #[cfg(client)]
                            {
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        let _ = storage.set_item(
                                            "session_id",
                                            &session.session_id.to_string(),
                                        );
                                        let _ = storage.set_item(
                                            "session_verifier",
                                            &session.session_verifier,
                                        );
                                    }
                                }
                            }

                            message.set("Login successful! Redirecting...".to_string());
                            message_type.set("success".to_string());

                            // Redirect to home
                            #[cfg(client)]
                            {
                                if let Some(window) = web_sys::window() {
                                    let _ = window.location().set_href("/");
                                }
                            }
                        }
                    } else {
                        message.set(response.message);
                        message_type.set("error".to_string());
                    }
                }
                Err(e) => {
                    message.set(format!("Error: {}", e));
                    message_type.set("error".to_string());
                }
            }
            submitting.set(false);
        });
    };

    let submit_reset = move |_| {
        let email_val = reset_email.get_clone();

        if email_val.is_empty() {
            message.set("Please enter your email".to_string());
            message_type.set("error".to_string());
            return;
        }

        submitting.set(true);
        message.set(String::new());

        #[cfg(client)]
        spawn_local(async move {
            match request_password_reset(&email_val).await {
                Ok(response) => {
                    message.set(response.message);
                    if response.success {
                        message_type.set("success".to_string());
                        show_reset.set(false);
                    } else {
                        message_type.set("error".to_string());
                    }
                }
                Err(e) => {
                    message.set(format!("Error: {}", e));
                    message_type.set("error".to_string());
                }
            }
            submitting.set(false);
        });
    };

    let toggle_reset = move |_| {
        show_reset.set(!show_reset.get());
        message.set(String::new());
    };

    view! {
        div(class = "container") {
            div(class = "card") {
                h1 { (if show_reset.get() { "Reset Password" } else { "Sign In" }) }

                (if !show_reset.get() {
                    view! {
                        form(on:submit = move |e: sycamore::web::events::SubmitEvent| {
                            e.prevent_default();
                            submit_login(());
                        }) {
                            div(class = "form-group") {
                                label { "Email" }
                                input(
                                    r#type = "email",
                                    placeholder = "Enter your email",
                                    bind:value = email,
                                    disabled = submitting.get()
                                )
                            }

                            div(class = "form-group") {
                                label { "Password" }
                                input(
                                    r#type = "password",
                                    placeholder = "Enter your password",
                                    bind:value = password,
                                    disabled = submitting.get()
                                )
                            }

                            button(
                                r#type = "submit",
                                disabled = submitting.get()
                            ) {
                                (if submitting.get() { "Signing in..." } else { "Sign In" })
                            }
                        }

                        div(class = "nav-links") {
                            a(href = "#", on:click = toggle_reset) { "Forgot password?" }
                            " | "
                            a(href = "/register") { "Create account" }
                        }
                    }
                } else {
                    view! {
                        form(on:submit = move |e: sycamore::web::events::SubmitEvent| {
                            e.prevent_default();
                            submit_reset(());
                        }) {
                            p(style = "text-align: center; margin-bottom: 1.5rem; color: #666;") {
                                "Enter your email to receive a password reset link"
                            }

                            div(class = "form-group") {
                                label { "Email" }
                                input(
                                    r#type = "email",
                                    placeholder = "Enter your email",
                                    bind:value = reset_email,
                                    disabled = submitting.get()
                                )
                            }

                            button(
                                r#type = "submit",
                                disabled = submitting.get()
                            ) {
                                (if submitting.get() { "Sending..." } else { "Send Reset Link" })
                            }

                            button(
                                r#type = "button",
                                class = "secondary-btn",
                                on:click = toggle_reset,
                                disabled = submitting.get()
                            ) {
                                "Back to Sign In"
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
async fn login_user(email: &str, password: &str) -> Result<ApiResponse<Session>, String> {
    use gloo_net::http::Request;

    let response = Request::post("/auth/login")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<ApiResponse<Session>>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(client)]
async fn request_password_reset(email: &str) -> Result<ApiResponse, String> {
    use gloo_net::http::Request;

    let response = Request::post("/auth/request-reset")
        .json(&serde_json::json!({
            "email": email
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
    Template::build("login").view(login_page).build()
}
