use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::types::ApiResponse;
use crate::types::SessionData;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserProfile {
    id: i32,
    email: String,
    created_at: Option<String>,
    otp_confirmed: bool,
    roles: Vec<String>,
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
fn clear_session() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("session_id");
            let _ = storage.remove_item("session_verifier");
        }
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
async fn logout(session: &SessionData) -> Result<ApiResponse, String> {
    use gloo_net::http::Request;
    let response = Request::post("/api/auth/logout")
        .json(&serde_json::json!({"session_id": session.session_id, "session_verifier": &session.session_verifier}))
        .map_err(|e| e.to_string())?.send().await.map_err(|e| e.to_string())?;
    response.json::<ApiResponse>().await.map_err(|e| e.to_string())
}

fn dashboard_page() -> View {
    let loading = create_signal(true);
    let is_logged_in = create_signal(false);
    let profile = create_signal(None::<UserProfile>);
    let has_create_posts = create_signal(false);

    #[cfg(client)]
    {
        let profile = profile.clone();
        let has_create_posts = has_create_posts.clone();
        let is_logged_in = is_logged_in.clone();
        let loading = loading.clone();

        sycamore::futures::spawn_local(async move {
            if let Some(session) = get_stored_session() {
                match get_profile(&session).await {
                    Ok(response) => {
                        if let Some(p) = response.data {
                            profile.set(Some(p.clone()));
                            is_logged_in.set(true);
                            has_create_posts.set(p.roles.iter().any(|r| r == "admin" || r == "editor" || r == "author"));
                        }
                    }
                    Err(_) => {
                        clear_session();
                        let window = web_sys::window().unwrap();
                        window.location().set_href("/login").unwrap();
                    }
                }
            } else {
                let window = web_sys::window().unwrap();
                window.location().set_href("/login").unwrap();
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        loading.set(false);
    }

    let handle_logout = move |_: web_sys::MouseEvent| {
        #[cfg(client)]
        {
            let is_logged_in = is_logged_in.clone();
            sycamore::futures::spawn_local(async move {
                if let Some(session) = get_stored_session() {
                    let _ = logout(&session).await;
                }
                clear_session();
                is_logged_in.set(false);
                let window = web_sys::window().unwrap();
                window.location().set_href("/").unwrap();
            });
        }
    };

    view! {
        div(class = "page-body") {
            div(class = "container-fluid") {
                div(class = "page-header") {
                    div(class = "row") {
                        div(class = "col-lg-6") {
                            div(class = "page-header-left") {
                                h3 { "My Dashboard" }
                                span(class = "d-block") { "Manage your account and activity" }
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
                } else if !is_logged_in.get() {
                    view! {
                        div(class = "card") {
                            div(class = "card-body") {
                                h4 { "Please Log In" }
                                p { "You need to be logged in to view your dashboard." }
                                Link(to = "/login", class = "btn btn-primary") { "Login" }
                            }
                        }
                    }
                } else {
                    let profile = profile.get_clone();
                    let profile_ref = profile.as_ref();
                    let email_val = profile_ref.and_then(|p| Some(p.email.clone())).unwrap_or_default();
                    let role_val = profile_ref.map(|p| if p.roles.is_empty() { "Member".to_string() } else { p.roles[0].clone() }).unwrap_or("Member".to_string());
                    let otp_val = profile_ref.map(|p| p.otp_confirmed).unwrap_or(false);
                    let created_val = profile_ref.and_then(|p| p.created_at.clone()).unwrap_or("N/A".to_string());

                    let profile_email = email_val.clone();
                    let profile_role = role_val.clone();
                    view! {
                        div(class = "row") {
                            div(class = "col-lg-4") {
                                div(class = "card") {
                                    div(class = "card-body") {
                                        div(class = "profile-details text-center") {
                                            img(src = "https://ui-avatars.com/api/?name=User&background=667eea&color=fff", class = "img-70 rounded-circle", alt = "")
                                            h5(class = "f-w-600 f-16 mb-0") { (email_val) }
                                            span(class = "badge badge-primary") { (role_val) }
                                        }
                                        hr { }
                                        ul(class = "profile-social") {
                                            li { Link(to = "/profile", class = "btn btn-primary btn-block") { "Edit Profile" } }
                                            li { button(class = "btn btn-secondary btn-block", on:click = handle_logout) { "Logout" } }
                                        }
                                        div(class = "table-responsive") {
                                            table(class = "table table-border-none mb-0") {
                                                tbody {
                                                    tr {
                                                        td { "Email" }
                                                        td { span { (profile_email) } }
                                                    }
                                                    tr {
                                                        td { "OTP Status" }
                                                        td {
                                                            (if otp_val {
                                                                view! { span(class = "badge badge-success") { "Verified" } }
                                                            } else {
                                                                view! { span(class = "badge badge-warning") { "Pending" } }
                                                            })
                                                        }
                                                    }
                                                    tr {
                                                        td { "Member Since" }
                                                        td { span { (created_val) } }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            div(class = "col-lg-8") {
                                div(class = "row") {
                                    div(class = "col-md-12") {
                                        div(class = "card") {
                                            div(class = "card-header") { h5 { "Quick Actions" } }
                                            div(class = "card-body") {
                                                div(class = "row") {
                                                    div(class = "col-sm-4") {
                                                        Link(to = "/posts", class = "btn btn-primary btn-block") {
                                                            i(class = "feather icon-file-text m-r-10") "View Posts"
                                                        }
                                                    }
                                                    (if has_create_posts.get() {
                                                        view! {
                                                            div(class = "col-sm-4") {
                                                                Link(to = "/posts/create", class = "btn btn-secondary btn-block") {
                                                                    i(class = "feather icon-plus m-r-10") "New Post"
                                                                }
                                                            }
                                                        }
                                                    } else { view! {} })
                                                    div(class = "col-sm-4") {
                                                        Link(to = "/contact", class = "btn btn-info btn-block") {
                                                            i(class = "feather icon-message-square m-r-10") "Contact Us"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    div(class = "col-md-12") {
                                        div(class = "card") {
                                            div(class = "card-header") {
                                                h5 { "My Support Messages" }
                                                Link(to = "/contact", class = "btn btn-sm btn-primary") { "New Message" }
                                            }
                                            div(class = "card-body") {
                                                p(class = "text-center text-muted") { "Your support messages will appear here." }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div(class = "row m-t-20") {
                            div(class = "col-md-4") {
                                div(class = "card") {
                                    div(class = "card-body") {
                                        div(class = "d-flex") {
                                            div(class = "flex-grow-1") {
                                                span(class = "f-w-600") { "Posts" }
                                                h4(class = "counter") { "0" }
                                            }
                                            i(class = "feather icon-file-text f-28", style = "color: #667eea;")
                                        }
                                    }
                                }
                            }
                            div(class = "col-md-4") {
                                div(class = "card") {
                                    div(class = "card-body") {
                                        div(class = "d-flex") {
                                            div(class = "flex-grow-1") {
                                                span(class = "f-w-600") { "Categories" }
                                                h4(class = "counter") { "0" }
                                            }
                                            i(class = "feather icon-book f-28", style = "color: #11998e;")
                                        }
                                    }
                                }
                            }
                            div(class = "col-md-4") {
                                div(class = "card") {
                                    div(class = "card-body") {
                                        div(class = "d-flex") {
                                            div(class = "flex-grow-1") {
                                                span(class = "f-w-600") { "Tags" }
                                                h4(class = "counter") { "0" }
                                            }
                                            i(class = "feather icon-tag f-28", style = "color: #ee0979;")
                                        }
                                    }
                                }
                            }
                        }
                    }
                })
            }
        }
    }
}

pub fn get_template() -> Template {
    Template::build("dashboard").view(dashboard_page).build()
}
