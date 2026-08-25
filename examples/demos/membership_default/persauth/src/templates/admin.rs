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
struct DashboardStats {
    total_users: i64,
    total_posts: i64,
    published_posts: i64,
    total_categories: i64,
    total_tags: i64,
    total_support_messages: i64,
    unread_support_messages: i64,
    recent_users: Vec<RecentUser>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RecentUser {
    user_id: i32,
    email: String,
    created_at: String,
}

#[derive(Clone, Copy)]
enum AdminTab {
    Overview,
    Users,
    Support,
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
fn redirect_to_login() {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href("/login");
    }
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
async fn get_stats(session: &SessionData) -> Result<ApiResponse<DashboardStats>, String> {
    use gloo_net::http::Request;
    let response = Request::post("/api/admin/stats")
        .json(&serde_json::json!({"session_id": session.session_id, "session_verifier": &session.session_verifier}))
        .map_err(|e| e.to_string())?.send().await.map_err(|e| e.to_string())?;
    response.json::<ApiResponse<DashboardStats>>().await.map_err(|e| e.to_string())
}

fn admin_page() -> View {
    let loading = create_signal(true);
    let is_admin = create_signal(false);
    let active_tab = create_signal(AdminTab::Overview);
    let stats = create_signal(None::<DashboardStats>);

    #[cfg(client)]
    {
        let is_admin = is_admin.clone();
        let stats = stats.clone();
        let loading = loading.clone();
        sycamore::futures::spawn_local(async move {
            if let Some(session) = get_stored_session() {
                match get_profile(&session).await {
                    Ok(response) => {
                        if let Some(profile) = response.data {
                            let has_admin = profile.roles.iter().any(|r| r == "admin" || r == "editor");
                            is_admin.set(has_admin);
                            if has_admin {
                                if let Ok(response) = get_stats(&session).await {
                                    stats.set(response.data);
                                }
                            }
                        }
                    }
                    Err(_) => redirect_to_login(),
                }
            } else {
                redirect_to_login();
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        // Keep `loading` true on the engine so the initial SSR markup matches the
        // initial client-side view during hydration.
    }

    let set_tab = move |tab: AdminTab| {
        active_tab.set(tab);
    };

    view! {
        div(class = "page-body") {
            div(class = "container-fluid") {
                div(class = "page-header") {
                    div(class = "row") {
                        div(class = "col-lg-6") {
                            div(class = "page-header-left") {
                                h3 { "Admin Dashboard" }
                                p { "Manage your application" }
                            }
                        }
                        div(class = "col-lg-6") {
                            ol(class = "breadcrumb") {
                                li(class = "breadcrumb-item") { Link(to = "/") { "Home" } }
                                li(class = "breadcrumb-item active") { "Dashboard" }
                            }
                        }
                    }
                }

                (if loading.get() {
                    view! { div(class = "loader-box") { div(class = "loader") { } } }
                } else if !is_admin.get() {
                    view! {
                        div(class = "card") {
                            div(class = "card-body") {
                                h4 { "Access Denied" }
                                p { "You don't have permission to access the admin dashboard." }
                                Link(to = "/", class = "btn btn-primary") { "Go Home" }
                            }
                        }
                    }
                } else {
                    view! {
                        div(class = "card") {
                            div(class = "card-header") {
                                ul(class = "nav nav-tabs") {
                                    li(class = "nav-item") {
                                        a(class = if matches!(active_tab.get(), AdminTab::Overview) { "nav-link active" } else { "nav-link" },
                                           on:click = move |_| set_tab(AdminTab::Overview)) { "Overview" }
                                    }
                                    li(class = "nav-item") {
                                        a(class = if matches!(active_tab.get(), AdminTab::Users) { "nav-link active" } else { "nav-link" },
                                           on:click = move |_| set_tab(AdminTab::Users)) { "Users" }
                                    }
                                    li(class = "nav-item") {
                                        a(class = if matches!(active_tab.get(), AdminTab::Support) { "nav-link active" } else { "nav-link" },
                                           on:click = move |_| set_tab(AdminTab::Support)) { "Support" }
                                    }
                                }
                            }
                            div(class = "card-body") {
                                (if matches!(active_tab.get(), AdminTab::Overview) {
                                    let stats = stats.get_clone();
                                    let total_users = stats.as_ref().map(|x| x.total_users).unwrap_or(0);
                                    let total_posts = stats.as_ref().map(|x| x.total_posts).unwrap_or(0);
                                    let unread = stats.as_ref().map(|x| x.unread_support_messages).unwrap_or(0);
                                    let total_support = stats.as_ref().map(|x| x.total_support_messages).unwrap_or(0);
                                    view! {
                                        div(class = "row") {
                                            div(class = "col-xl-3 col-md-6") {
                                                div(class = "card o-hidden") {
                                                    div(class = "card-body") {
                                                        div(class = "d-flex") {
                                                            div(class = "flex-grow-1") {
                                                                span(class = "f-w-600") { "Users" }
                                                                h4(class = "counter") { (total_users) }
                                                            }
                                                            i(class = "feather icon-users", style = "color: #667eea;")
                                                        }
                                                    }
                                                }
                                            }
                                            div(class = "col-xl-3 col-md-6") {
                                                div(class = "card o-hidden") {
                                                    div(class = "card-body") {
                                                        div(class = "d-flex") {
                                                            div(class = "flex-grow-1") {
                                                                span(class = "f-w-600") { "Posts" }
                                                                h4(class = "counter") { (total_posts) }
                                                            }
                                                            i(class = "feather icon-file-text", style = "color: #11998e;")
                                                        }
                                                    }
                                                }
                                            }
                                            div(class = "col-xl-3 col-md-6") {
                                                div(class = "card o-hidden") {
                                                    div(class = "card-body") {
                                                        div(class = "d-flex") {
                                                            div(class = "flex-grow-1") {
                                                                span(class = "f-w-600") { "Support" }
                                                                h4(class = "counter") {
                                                                    (unread) span(class = "f-14 f-w-400") { " / " } (total_support)
                                                                }
                                                            }
                                                            i(class = "feather icon-message-square", style = "color: #ff6a00;")
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if matches!(active_tab.get(), AdminTab::Users) {
                                    view! {
                                        div(class = "card") {
                                            div(class = "card-header") {
                                                h5 { "User Management" }
                                                span(class = "d-block m-t-5", style = "color: #6c757d; font-size: 0.875rem;") { "Manage user accounts and role assignments" }
                                            }
                                            div(class = "card-body") {
                                                Link(to = "/admin/users", class = "btn btn-primary") { "Go to User Management" }
                                            }
                                        }
                                    }
                                } else if matches!(active_tab.get(), AdminTab::Support) {
                                    view! {
                                        div(class = "card") {
                                            div(class = "card-header") { h5 { "Support Inbox" } }
                                            div(class = "card-body") {
                                                p(class = "text-muted") { "Support inbox features coming soon..." }
                                            }
                                        }
                                    }
                                } else { view! {} })
                            }
                        }
                    }
                })
            }
        }
    }
}

pub fn get_template() -> Template {
    Template::build("admin").view(admin_page).build()
}
