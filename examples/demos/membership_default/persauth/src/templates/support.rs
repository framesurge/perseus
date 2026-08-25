use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::futures::spawn_local;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SupportMessage {
    pub id: i32,
    pub sender_name: String,
    pub sender_email: String,
    pub user_id: Option<i32>,
    pub subject: String,
    pub body: String,
    pub status: String,
    pub admin_reply: Option<String>,
    pub replied_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupportMessagesResponse {
    pub success: bool,
    pub message: String,
    pub messages: Vec<SupportMessage>,
    pub total: i64,
}

#[derive(Clone, Copy)]
enum SupportTab {
    Inbox,
    Stats,
}

#[cfg(client)]
fn get_session_credentials() -> (i32, String) {
    let window = match web_sys::window() { Some(w) => w, None => return (0, String::new()) };
    let storage = match window.local_storage() { Ok(Some(s)) => s, _ => return (0, String::new()) };
    let session_id = storage.get_item("session_id").ok().flatten().and_then(|s| s.parse().ok()).unwrap_or(0);
    let session_verifier = storage.get_item("session_verifier").ok().flatten().unwrap_or_default();
    (session_id, session_verifier)
}

#[cfg(client)]
fn redirect_to_login() {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href("/login");
    }
}

#[cfg(client)]
async fn fetch_support_messages(
    session_id: i32,
    session_verifier: &str,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<SupportMessagesResponse, String> {
    use gloo_net::http::Request;

    let response = Request::post("/api/admin/messages")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "session_id": session_id,
            "session_verifier": session_verifier,
            "status": status,
            "limit": limit,
            "offset": offset,
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<SupportMessagesResponse>()
        .await
        .map_err(|e| e.to_string())
}

// Simple view with Sycamore signals for tab navigation
fn support_management_page() -> View {
    let active_tab = create_signal(SupportTab::Inbox);
    let loading = create_signal(true);
    let error_message = create_signal(String::new());
    let success_message = create_signal(String::new());
    let is_admin = create_signal(false);
    let messages = create_signal(Vec::<SupportMessage>::new());
    let total = create_signal(0i64);
    let selected_message = create_signal(None::<SupportMessage>);
    let show_message_modal = create_signal(false);

    #[cfg(client)]
    {
        let is_admin = is_admin.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();
        let messages = messages.clone();
        let total = total.clone();

        spawn_local(async move {
            let (session_id, session_verifier) = get_session_credentials();
            if session_id == 0 {
                loading.set(false);
                redirect_to_login();
                return;
            }
            match fetch_support_messages(session_id, &session_verifier, None, 50, 0).await {
                Ok(resp) => {
                    if resp.success {
                        is_admin.set(true);
                        messages.set(resp.messages);
                        total.set(resp.total);
                    } else {
                        error_message.set(resp.message);
                    }
                }
                Err(e) => error_message.set(e),
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        // Keep `loading` true on the engine so the initial SSR markup matches the
        // initial client-side view during hydration.
    }

    let set_tab = move |tab: SupportTab| {
        active_tab.set(tab);
    };
    view! {
        // Simple navigation header
        nav(class = "navbar", style = "background: white; box-shadow: 0 2px 10px rgba(0,0,0,0.08); padding: 0.75rem 2rem; display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;") {
            div(class = "navbar-brand", style = "display: flex; align-items: center; gap: 10px;") {
                Link(to = "/", style = "font-size: 1.25rem; font-weight: 700; color: #333; text-decoration: none;") {
                    span(style = "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent;") { "Perseus" }
                    span(style = "color: #666; font-weight: 400;") { "Membership" }
                }
            }
            div(class = "navbar-nav", style = "display: flex; gap: 0.5rem; align-items: center;") {
                Link(to = "/admin", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Dashboard" }
                Link(to = "/admin/users", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Users" }
                Link(to = "/admin/roles", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Roles" }
                Link(to = "/admin/support", style = "display: inline-block; padding: 0.5rem 1rem; color: #667eea; background: rgba(102,126,234,0.1); text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Support" }
            }
        }

        div(class = "page-body") {
            div(class = "container-fluid") {
                (if loading.get() {
                    view! { div(class = "loader-box") { div(class = "loader") } }
                } else if !is_admin.get() {
                    view! {
                        div(class = "card") {
                            div(class = "card-body") {
                                div(class = "alert alert-warning") {
                                    "Access denied. Please log in as an admin."
                                }
                            }
                        }
                    }
                } else {
                    view! {
                        // Tabs using Sycamore signals
                        ul(class = "nav-tabs", style = "margin-bottom: 1rem;") {
                            li(class = "nav-item") {
                                a(class = if matches!(active_tab.get(), SupportTab::Inbox) { "nav-link active" } else { "nav-link" },
                                   on:click = move |_| set_tab(SupportTab::Inbox)) { "Inbox" }
                            }
                            li(class = "nav-item") {
                                a(class = if matches!(active_tab.get(), SupportTab::Stats) { "nav-link active" } else { "nav-link" },
                                   on:click = move |_| set_tab(SupportTab::Stats)) { "Stats" }
                            }
                        }

                        (if matches!(active_tab.get(), SupportTab::Inbox) {
                            view! {
                                div(class = "card") {
                                    div(class = "card-header") {
                                        h5 { "Support Messages" }
                                        span(class = "d-block m-t-5", style = "color: #6c757d; font-size: 0.875rem;") { "Manage customer support inquiries" }
                                    }
                                    div(class = "card-body") {
                                        div(class = "alert alert-info") {
                                            strong { "Support Inbox" }
                                            " - Manage support tickets and customer inquiries."
                                        }

                                        div(class = "table-responsive") {
                                            table(class = "table") {
                                                thead {
                                                    tr {
                                                        th { "Status" }
                                                        th { "From" }
                                                        th { "Subject" }
                                                        th { "Date" }
                                                        th { "Actions" }
                                                    }
                                                }
                                                tbody {
                                                    Indexed(
                                                        list = messages,
                                                        view = move |msg: SupportMessage| {
                                                            let badge_class = match msg.status.as_str() {
                                                                "new" => "badge badge-warning",
                                                                "read" => "badge badge-info",
                                                                "replied" => "badge badge-success",
                                                                "closed" => "badge badge-secondary",
                                                                _ => "badge badge-light",
                                                            };
                                                            let open_message = {
                                                                let selected_message = selected_message.clone();
                                                                let show_message_modal = show_message_modal.clone();
                                                                let msg_for_modal = msg.clone();
                                                                move |_| {
                                                                    selected_message.set(Some(msg_for_modal.clone()));
                                                                    show_message_modal.set(true);
                                                                }
                                                            };

                                                            view! {
                                                                tr {
                                                                    td { span(class = badge_class) { (msg.status.clone()) } }
                                                                    td {
                                                                        div {
                                                                            strong { (msg.sender_name.clone()) }
                                                                            br()
                                                                            span(class = "text-muted", style = "font-size: 0.875rem;") { (msg.sender_email.clone()) }
                                                                        }
                                                                    }
                                                                    td { (msg.subject.clone()) }
                                                                    td { (msg.created_at.clone()) }
                                                                    td {
                                                                        button(class = "btn btn-primary btn-sm", on:click = open_message) { "View" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    )
                                                }
                                            }
                                        }

                                        (if messages.get_clone().is_empty() {
                                            view! {
                                                div(class = "text-center", style = "padding: 2rem; color: #999;") {
                                                    "No support messages found."
                                                }
                                            }
                                        } else { view! {} })

                                        (if success_message.get_clone().is_empty() && error_message.get_clone().is_empty() {
                                            view! {}
                                        } else if !success_message.get_clone().is_empty() {
                                            let msg = success_message.get_clone();
                                            view! { div(class = "alert alert-success", style = "margin-top: 1rem;") { (msg) } }
                                        } else {
                                            let msg = error_message.get_clone();
                                            view! { div(class = "alert alert-danger", style = "margin-top: 1rem;") { (msg) } }
                                        })
                                    }
                                }
                            }
                        } else if matches!(active_tab.get(), SupportTab::Stats) {
                            let msgs = messages.get_clone();
                            let new_count = msgs.iter().filter(|m| m.status == "new").count() as i64;
                            let read_count = msgs.iter().filter(|m| m.status == "read").count() as i64;
                            let replied_count = msgs.iter().filter(|m| m.status == "replied").count() as i64;
                            let closed_count = msgs.iter().filter(|m| m.status == "closed").count() as i64;
                            let total_count = total.get();
                            view! {
                                div(class = "card") {
                                    div(class = "card-header") {
                                        h5 { "Quick Stats" }
                                    }
                                    div(class = "card-body") {
                                        div(class = "row") {
                                            div(class = "col-md-3") {
                                                div(class = "card text-center", style = "background: #fff3cd;") {
                                                    div(class = "card-body") {
                                                        h3 { (new_count) }
                                                        p { "New Messages" }
                                                    }
                                                }
                                            }
                                            div(class = "col-md-3") {
                                                div(class = "card text-center", style = "background: #d1ecf1;") {
                                                    div(class = "card-body") {
                                                        h3 { (read_count) }
                                                        p { "Read" }
                                                    }
                                                }
                                            }
                                            div(class = "col-md-3") {
                                                div(class = "card text-center", style = "background: #d4edda;") {
                                                    div(class = "card-body") {
                                                        h3 { (replied_count) }
                                                        p { "Replied" }
                                                    }
                                                }
                                            }
                                            div(class = "col-md-3") {
                                                div(class = "card text-center", style = "background: #f8d7da;") {
                                                    div(class = "card-body") {
                                                        h3 { (closed_count) }
                                                        p { "Closed / Total" }
                                                        p(class = "text-muted", style = "margin-bottom: 0; font-size: 0.875rem;") {
                                                            "Total: " (total_count)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else { view! {} })
                    }
                })
            }
        }

        // Message modal
        (if show_message_modal.get() {
            let close_modal = {
                let show_message_modal = show_message_modal.clone();
                move |_| show_message_modal.set(false)
            };
            let msg_opt = selected_message.get_clone();

            view! {
                div(class = "link-modal-overlay") {
                    div(class = "card", style = "min-width: 700px; max-width: 900px; max-height: 90vh; overflow-y: auto;") {
                        div(style = "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;") {
                            h2(style = "margin: 0;") { "Support Message" }
                            button(
                                on:click = close_modal,
                                style = "width: auto; padding: 0.5rem 1rem; background: #6c757d;"
                            ) { "Close" }
                        }

                        (if let Some(msg) = msg_opt.as_ref() {
                            let sender_name = msg.sender_name.clone();
                            let sender_email = msg.sender_email.clone();
                            let subject = msg.subject.clone();
                            let status = msg.status.clone();
                            let body = msg.body.clone();
                            let admin_reply = msg.admin_reply.clone();

                            view! {
                                div(class = "form-group") {
                                    label { "From" }
                                    p { strong { (sender_name) } " (" (sender_email) ")" }
                                }
                                div(class = "form-group") {
                                    label { "Subject" }
                                    p { (subject) }
                                }
                                div(class = "form-group") {
                                    label { "Status" }
                                    p { (status) }
                                }
                                div(class = "form-group") {
                                    label { "Message" }
                                    pre(style = "white-space: pre-wrap; background: #f8f9fa; padding: 1rem; border-radius: 8px;") { (body) }
                                }
                                (if let Some(reply) = admin_reply {
                                    view! {
                                        div(class = "form-group") {
                                            label { "Admin Reply" }
                                            pre(style = "white-space: pre-wrap; background: #eef6ff; padding: 1rem; border-radius: 8px;") { (reply) }
                                        }
                                    }
                                } else { view! {} })
                            }
                        } else {
                            view! { p { "No message selected." } }
                        })
                    }
                }
            }
        } else { view! {} })

        // Client-side initialization script
        script {
            r#"
            (function() {
                console.log('Support management page loaded');
            })();
            "#
        }
    }
}

pub fn get_template() -> Template {
    Template::build("admin/support").view(support_management_page).build()
}
