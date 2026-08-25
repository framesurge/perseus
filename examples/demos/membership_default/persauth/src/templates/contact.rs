use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::types::ApiResponse;
use crate::types::SessionData;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserProfile {
    id: i32,
    email: String,
    roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateMessageRequest {
    sender_name: String,
    sender_email: String,
    subject: String,
    body: String,
}

#[cfg(client)]
fn get_stored_session() -> Option<SessionData> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let session_id = storage.get_item("session_id").ok()??;
    let session_verifier = storage.get_item("session_verifier").ok()??;
    Some(SessionData { session_id: session_id.parse().ok()?, session_verifier })
}

#[cfg(client)]
async fn get_profile(session: &SessionData) -> Result<ApiResponse<UserProfile>, String> {
    use gloo_net::http::Request;
    let response = Request::post("/api/auth/profile")
        .json(&serde_json::json!({"session_id": session.session_id, "session_verifier": &session.session_verifier}))
        .map_err(|e| e.to_string())?.send().await.map_err(|e| e.to_string())?;
    response.json::<ApiResponse<UserProfile>>().await.map_err(|e| e.to_string())
}

#[cfg(client)]
async fn create_message(req: &CreateMessageRequest) -> Result<ApiResponse<i32>, String> {
    use gloo_net::http::Request;
    let response = Request::post("/api/support")
        .json(req).map_err(|e| e.to_string())?.send().await.map_err(|e| e.to_string())?;
    response.json::<ApiResponse<i32>>().await.map_err(|e| e.to_string())
}

#[derive(Clone, Copy)]
enum Tab {
    Overview,
    Users,
    Support,
}

fn contact_page() -> View {
    let loading = create_signal(true);
    let submitting = create_signal(false);
    let sender_name = create_signal(String::new());
    let sender_email = create_signal(String::new());
    let subject = create_signal(String::new());
    let body = create_signal(String::new());
    let message = create_signal(String::new());
    let message_type = create_signal(String::new());

    #[cfg(client)]
    {
        let sender_name = sender_name.clone();
        let sender_email = sender_email.clone();
        let loading = loading.clone();
        sycamore::futures::spawn_local(async move {
            if let Some(session) = get_stored_session() {
                match get_profile(&session).await {
                    Ok(response) => {
                        if let Some(profile) = response.data {
                            sender_name.set(profile.email.split('@').next().unwrap_or("").to_string());
                            sender_email.set(profile.email);
                        }
                    }
                    Err(_) => {}
                }
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        loading.set(false);
    }

    let handle_submit = move |_: web_sys::MouseEvent| {
        #[cfg(client)]
        {
            let sender_name = sender_name.clone();
            let sender_email = sender_email.clone();
            let subject = subject.clone();
            let body = body.clone();
            let submitting = submitting.clone();
            let message = message.clone();
            let message_type = message_type.clone();

            sycamore::futures::spawn_local(async move {
                submitting.set(true);
                message.set(String::new());
                message_type.set(String::new());

                let req = CreateMessageRequest {
                    sender_name: sender_name.get_clone(),
                    sender_email: sender_email.get_clone(),
                    subject: subject.get_clone(),
                    body: body.get_clone(),
                };

                match create_message(&req).await {
                    Ok(response) => {
                        if response.success {
                            message.set("Your message has been sent!".to_string());
                            message_type.set("success".to_string());
                            sender_name.set(String::new());
                            sender_email.set(String::new());
                            subject.set(String::new());
                            body.set(String::new());
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
        }
    };

    view! {
        div(class = "page-body") {
            div(class = "container-fluid") {
                div(class = "card") {
                    div(class = "card-header") {
                        h5 { "Contact Us" }
                        span(class = "d-block m-t-5") { "Send us a message and we'll respond as soon as possible" }
                    }
                    div(class = "card-body") {
                        div {
                            div(class = "form-group") {
                                label(r#for = "sender_name") { "Your Name" }
                                input(id = "sender_name", r#type = "text", class = "form-control", placeholder = "Enter your name", bind:value = sender_name)
                            }
                            div(class = "form-group") {
                                label(r#for = "sender_email") { "Email Address" }
                                input(id = "sender_email", r#type = "email", class = "form-control", placeholder = "Enter your email", bind:value = sender_email)
                            }
                            div(class = "form-group") {
                                label(r#for = "subject") { "Subject" }
                                input(id = "subject", r#type = "text", class = "form-control", placeholder = "What is this regarding?", bind:value = subject)
                            }
                            div(class = "form-group") {
                                label(r#for = "body") { "Message" }
                                textarea(id = "body", class = "form-control", rows = "5", placeholder = "Describe your inquiry or issue...", bind:value = body)
                            }
                            div(class = "form-group") {
                                button(r#type = "button", class = "btn btn-primary", disabled = submitting.get(), on:click = handle_submit) {
                                    (if submitting.get() { "Sending..." } else { "Send Message" })
                                }
                            }
                            (if !message.get_clone().is_empty() {
                                view! { div(class = format!("alert alert-{}", message_type.get_clone())) { (message.get_clone()) } }
                            } else { view! {} })
                        }
                    }
                }
                div(class = "row") {
                    div(class = "col-md-6") {
                        div(class = "card") {
                            div(class = "card-body") {
                                h5 { "Frequently Asked Questions" }
                                div(class = "faq-item m-b-20") {
                                    h6 { "How do I reset my password?" }
                                    p(class = "text-muted") { "Go to login page and click 'Forgot Password'" }
                                }
                                div(class = "faq-item m-b-20") {
                                    h6 { "How long does verification take?" }
                                    p(class = "text-muted") { "Email verification is typically instant" }
                                }
                                div(class = "faq-item") {
                                    h6 { "Can I change my email?" }
                                    p(class = "text-muted") { "Contact support with your request" }
                                }
                            }
                        }
                    }
                    div(class = "col-md-6") {
                        div(class = "card") {
                            div(class = "card-body") {
                                h5 { "Other Ways to Reach Us" }
                                div(class = "support-info") {
                                    div(class = "media") {
                                        i(class = "feather icon-mail", style = "font-size: 24px; margin-right: 15px; color: #667eea;")
                                        div(class = "media-body") {
                                            h6 { "Email" }
                                            p { "support@example.com" }
                                        }
                                    }
                                }
                                div(class = "support-info m-t-20") {
                                    div(class = "media") {
                                        i(class = "feather icon-clock", style = "font-size: 24px; margin-right: 15px; color: #667eea;")
                                        div(class = "media-body") {
                                            h6 { "Response Time" }
                                            p { "Within 24-48 hours" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn get_template() -> Template {
    Template::build("contact").view(contact_page).build()
}
