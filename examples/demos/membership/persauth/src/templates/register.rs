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

fn register_page() -> View {
    let email = create_signal(String::new());
    let password = create_signal(String::new());
    let confirm_password = create_signal(String::new());
    let message = create_signal(String::new());
    let message_type = create_signal(String::new());
    let submitting = create_signal(false);

    // OTP verification state
    let show_otp = create_signal(false);
    let otp_code = create_signal(String::new());
    let session_id = create_signal(0i32);
    let session_verifier = create_signal(String::new());

    let submit_register = move |_| {
        let email_val = email.get_clone();
        let password_val = password.get_clone();
        let confirm_val = confirm_password.get_clone();

        if email_val.is_empty() || password_val.is_empty() {
            message.set("Please fill in all fields".to_string());
            message_type.set("error".to_string());
            return;
        }

        if password_val.len() < 8 {
            message.set("Password must be at least 8 characters".to_string());
            message_type.set("error".to_string());
            return;
        }

        if password_val != confirm_val {
            message.set("Passwords do not match".to_string());
            message_type.set("error".to_string());
            return;
        }

        submitting.set(true);
        message.set(String::new());

        #[cfg(client)]
        spawn_local(async move {
            match register_user(&email_val, &password_val).await {
                Ok(response) => {
                    if response.success {
                        if let Some(session) = response.data {
                            session_id.set(session.session_id);
                            session_verifier.set(session.session_verifier);
                            show_otp.set(true);
                            message.set(
                                "Registration successful! Check your email for confirmation code."
                                    .to_string(),
                            );
                            message_type.set("success".to_string());
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

    let submit_otp = move |_| {
        let code = otp_code.get_clone();
        let sid = session_id.get();
        let sverifier = session_verifier.get_clone();

        if code.is_empty() {
            message.set("Please enter the confirmation code".to_string());
            message_type.set("error".to_string());
            return;
        }

        submitting.set(true);
        message.set(String::new());

        #[cfg(client)]
        spawn_local(async move {
            match confirm_otp(&code, sid, &sverifier).await {
                Ok(response) => {
                    if response.success {
                        message.set("Email confirmed! Redirecting to login...".to_string());
                        message_type.set("success".to_string());

                        // Redirect to login after short delay
                        #[cfg(client)]
                        {
                            use wasm_bindgen::JsCast;
                            let window = web_sys::window().unwrap();
                            let _ = window.location().set_href("/login");
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

    let resend_code = move |_| {
        let sid = session_id.get();
        let sverifier = session_verifier.get_clone();

        submitting.set(true);

        #[cfg(client)]
        spawn_local(async move {
            match resend_otp(sid, &sverifier).await {
                Ok(response) => {
                    if response.success {
                        if let Some(session) = response.data {
                            session_id.set(session.session_id);
                            session_verifier.set(session.session_verifier);
                        }
                        message.set("New code sent to your email".to_string());
                        message_type.set("success".to_string());
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

    view! {
        div(class = "container") {
            div(class = "card") {
                h1 { "Create Account" }

                (if !show_otp.get() {
                    view! {
                        form(on:submit = move |e: sycamore::web::events::SubmitEvent| {
                            e.prevent_default();
                            submit_register(());
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
                                    placeholder = "Enter password (min 8 characters)",
                                    bind:value = password,
                                    disabled = submitting.get()
                                )
                            }

                            div(class = "form-group") {
                                label { "Confirm Password" }
                                input(
                                    r#type = "password",
                                    placeholder = "Confirm your password",
                                    bind:value = confirm_password,
                                    disabled = submitting.get()
                                )
                            }

                            button(
                                r#type = "submit",
                                disabled = submitting.get()
                            ) {
                                (if submitting.get() { "Creating account..." } else { "Create Account" })
                            }
                        }

                        div(class = "nav-links") {
                            "Already have an account? "
                            a(href = "/login") { "Sign in" }
                        }
                    }
                } else {
                    view! {
                        form(on:submit = move |e: sycamore::web::events::SubmitEvent| {
                            e.prevent_default();
                            submit_otp(());
                        }) {
                            p(style = "text-align: center; margin-bottom: 1.5rem; color: #666;") {
                                "Enter the confirmation code sent to your email"
                            }

                            div(class = "form-group") {
                                label { "Confirmation Code" }
                                input(
                                    r#type = "text",
                                    class = "otp-input",
                                    placeholder = "12345",
                                    maxlength = "5",
                                    bind:value = otp_code,
                                    disabled = submitting.get()
                                )
                            }

                            button(
                                r#type = "submit",
                                disabled = submitting.get()
                            ) {
                                (if submitting.get() { "Verifying..." } else { "Verify Code" })
                            }

                            button(
                                r#type = "button",
                                class = "secondary-btn",
                                on:click = resend_code,
                                disabled = submitting.get()
                            ) {
                                "Resend Code"
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
async fn register_user(email: &str, password: &str) -> Result<ApiResponse<Session>, String> {
    use gloo_net::http::Request;

    let response = Request::post("/auth/register")
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
async fn confirm_otp(
    code: &str,
    session_id: i32,
    session_verifier: &str,
) -> Result<ApiResponse, String> {
    use gloo_net::http::Request;

    let response = Request::post("/auth/confirm")
        .json(&serde_json::json!({
            "code": code,
            "session_id": session_id,
            "session_verifier": session_verifier
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

#[cfg(client)]
async fn resend_otp(
    session_id: i32,
    session_verifier: &str,
) -> Result<ApiResponse<Session>, String> {
    use gloo_net::http::Request;

    let response = Request::post("/auth/resend-otp")
        .json(&serde_json::json!({
            "session_id": session_id,
            "session_verifier": session_verifier
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

pub fn get_template() -> Template {
    Template::build("register").view(register_page).build()
}
