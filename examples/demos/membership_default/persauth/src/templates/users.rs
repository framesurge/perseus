use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::futures::spawn_local;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct User {
    pub user_id: i32,
    pub email: String,
    pub roles: Vec<String>,
    pub otp_confirmed: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UsersResponse {
    success: bool,
    message: String,
    users: Vec<User>,
    total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RolesResponse {
    success: bool,
    message: String,
    data: Option<Vec<Role>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AssignRoleResponse {
    success: bool,
    message: String,
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
async fn fetch_users() -> Result<UsersResponse, String> {
    use gloo_net::http::Request;
    let (session_id, session_verifier) = get_session_credentials();
    let response = Request::post("/api/admin/users")
        .json(&serde_json::json!({"session_id": session_id, "session_verifier": session_verifier}))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json::<UsersResponse>().await.map_err(|e| e.to_string())
}

#[cfg(client)]
async fn fetch_roles() -> Result<RolesResponse, String> {
    use gloo_net::http::Request;
    let response = Request::get("/api/permissions/roles")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json::<RolesResponse>().await.map_err(|e| e.to_string())
}

#[cfg(client)]
async fn assign_user_role(user_id: i32, role_id: i32) -> Result<AssignRoleResponse, String> {
    use gloo_net::http::Request;
    let response = Request::post("/api/permissions/user-roles")
        .json(&serde_json::json!({
            "user_id": user_id,
            "role_id": role_id
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json::<AssignRoleResponse>().await.map_err(|e| e.to_string())
}

fn users_page() -> View {
    let loading = create_signal(true);
    let error_message = create_signal(String::new());
    let success_message = create_signal(String::new());
    let users = create_signal(Vec::<User>::new());
    let roles = create_signal(Vec::<Role>::new());
    let is_admin = create_signal(false);

    // Form state
    let selected_user_id = create_signal(Option::<i32>::None);
    let selected_role_id = create_signal(String::new());
    let show_assign_role = create_signal(false);
    let assigning = create_signal(false);

    // Load users and roles on mount
    #[cfg(client)]
    {
        let users = users.clone();
        let roles = roles.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();
        let is_admin = is_admin.clone();

        spawn_local(async move {
            let (session_id, _) = get_session_credentials();
            if session_id == 0 {
                loading.set(false);
                redirect_to_login();
                return;
            }

            match fetch_users().await {
                Ok(response) => {
                    users.set(response.users);
                    is_admin.set(true);
                }
                Err(e) => {
                    error_message.set(format!("Failed to load users: {}", e));
                }
            }
            loading.set(false);
        });

        spawn_local(async move {
            match fetch_roles().await {
                Ok(response) => {
                    if response.success && response.data.is_some() {
                        roles.set(response.data.unwrap());
                    }
                }
                Err(_) => {}
            };
        });
    }

    #[cfg(engine)]
    {
        // Keep `loading` true on the engine so the initial SSR markup matches the
        // initial client-side view during hydration.
    }

    let close_assign_role = {
        let show_assign_role = show_assign_role.clone();
        move |_| {
            show_assign_role.set(false);
            selected_role_id.set(String::new());
            selected_user_id.set(Option::<i32>::None);
        }
    };

    let open_assign_role = {
        let selected_user_id = selected_user_id.clone();
        let show_assign_role = show_assign_role.clone();
        move |user_id: i32| {
            selected_user_id.set(Some(user_id));
            show_assign_role.set(true);
        }
    };

    let assign_role = move |_| {
        #[cfg(client)]
        {
            let user_id = match selected_user_id.get_clone() {
                Some(id) => id,
                None => return,
            };
            let role_id = match selected_role_id.get_clone().parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    error_message.set("Please select a role".to_string());
                    return;
                }
            };
            if role_id <= 0 {
                error_message.set("Please select a role".to_string());
                return;
            }

            let assigning = assigning.clone();
            let success_message = success_message.clone();
            let error_message = error_message.clone();
            let show_assign_role = show_assign_role.clone();
            let users = users.clone();

            spawn_local(async move {
                assigning.set(true);
                error_message.set(String::new());

                let result = assign_user_role(user_id, role_id).await;

                match result {
                    Ok(response) => {
                        if response.success {
                            success_message.set(response.message);
                            show_assign_role.set(false);
                            selected_role_id.set(String::new());
                            selected_user_id.set(Option::<i32>::None);
                            // Refresh users list
                            if let Ok(users_response) = fetch_users().await {
                                users.set(users_response.users);
                            }
                        } else {
                            error_message.set(response.message);
                        }
                    }
                    Err(e) => {
                        error_message.set(format!("Failed to assign role: {}", e));
                    }
                }
                assigning.set(false);
            });
        }
    };

    view! {
        nav(class = "navbar", style = "background: white; box-shadow: 0 2px 10px rgba(0,0,0,0.08); padding: 0.75rem 2rem; display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;") {
            div(class = "navbar-brand", style = "display: flex; align-items: center; gap: 10px;") {
                Link(to = "/", style = "font-size: 1.25rem; font-weight: 700; color: #333; text-decoration: none;") {
                    span(style = "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent;") { "Perseus" }
                    span(style = "color: #666; font-weight: 400;") { "Membership" }
                }
            }
            div(class = "navbar-nav", style = "display: flex; gap: 0.5rem; align-items: center;") {
                Link(to = "/admin", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Dashboard" }
                Link(to = "/admin/users", style = "display: inline-block; padding: 0.5rem 1rem; color: #667eea; background: rgba(102,126,234,0.1); text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Users" }
                Link(to = "/admin/roles", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Roles" }
                Link(to = "/admin/support", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Support" }
            }
        }

        div(class = "page-body") {
            div(class = "container-fluid") {
                div(class = "card") {
                    div(class = "card-header") {
                        h5 { "User Management" }
                        span(class = "d-block m-t-5", style = "color: #6c757d; font-size: 0.875rem;") { "Manage user accounts and role assignments" }
                    }
                    div(class = "card-body") {
                        // Messages
                        (if !success_message.get_clone().is_empty() {
                            let msg = success_message.get_clone();
                            view! { div(class = "alert alert-success") { (msg) } }
                        } else { view! {} })

                        (if !error_message.get_clone().is_empty() {
                            let msg = error_message.get_clone();
                            view! { div(class = "alert alert-danger") { (msg) } }
                        } else { view! {} })

                        // Loading state
                        (if loading.get() {
                            view! { div(class = "loader-box") { div(class = "loader") } }
                        } else if !is_admin.get() {
                            view! {
                                div(class = "alert alert-warning") {
                                    "You don't have permission to access this page. Please log in as an admin."
                                }
                            }
                        } else {
                            view! {
                                div(class = "table-responsive") {
                                    table(class = "table") {
                                        thead {
                                            tr {
                                                th { "ID" }
                                                th { "Email" }
                                                th { "Roles" }
                                                th { "OTP" }
                                                th { "Created" }
                                                th { "Actions" }
                                            }
                                        }
                                        tbody {
                                            Indexed(
                                                list = users,
                                                view = move |user: User| {
                                                    let user_id = user.user_id;
                                                    let user_email = user.email.clone();
                                                    let user_roles = user.roles.clone();
                                                    let user_otp = user.otp_confirmed;
                                                    let user_created = user.created_at.clone();

                                                    view! {
                                                        tr {
                                                            td { (user_id) }
                                                            td { strong { (user_email) } }
                                                            td {
                                                                Indexed(
                                                                    list = create_signal(user_roles),
                                                                    view = move |role: String| {
                                                                        view! {
                                                                            span(class = "badge badge-info", style = "margin-right: 0.25rem;") { (role) }
                                                                        }
                                                                    }
                                                                )
                                                            }
                                                            td {
                                                                (if user_otp {
                                                                    view! { span(class = "badge badge-success") { "Verified" } }
                                                                } else {
                                                                    view! { span(class = "badge badge-warning") { "Pending" } }
                                                                })
                                                            }
                                                            td { (user_created) }
                                                            td {
                                                                button(
                                                                    on:click = move |_| open_assign_role(user_id),
                                                                    class = "btn btn-primary btn-sm"
                                                                ) { "Assign Role" }
                                                            }
                                                        }
                                                    }
                                                }
                                            )
                                        }
                                    }
                                }

                                (if users.get_clone().is_empty() {
                                    view! {
                                        div(class = "text-center", style = "padding: 2rem; color: #999;") {
                                            "No users found. Users will appear here once registered."
                                        }
                                    }
                                } else { view! {} })
                            }
                        })
                    }
                }
            }
        }

        // Assign Role Modal
        (if show_assign_role.get() {
            let assigning_clone = assigning.get();
            view! {
                div(class = "link-modal-overlay") {
                    div(class = "card", style = "max-width: 400px;") {
                        div(style = "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;") {
                            h2(style = "margin: 0;") { "Assign Role" }
                            button(on:click = close_assign_role, style = "width: auto; padding: 0.5rem 1rem; background: #6c757d; border: none; cursor: pointer;") { "X" }
                        }

                        div(class = "form-group") {
                            label { "Select Role" }
                            select(bind:value = selected_role_id, class = "form-control") {
                                option(value = "") { "-- Select Role --" }
                                Indexed(
                                    list = roles,
                                    view = move |role: Role| {
                                        let role_name = role.name.clone();
                                        let role_id = role.id.to_string();
                                        view! { option(value = role_id) { (role_name) } }
                                    }
                                )
                            }
                        }

                        div(style = "display: flex; gap: 1rem; justify-content: flex-end; margin-top: 1rem;") {
                            button(on:click = close_assign_role, disabled = assigning_clone, style = "width: auto; padding: 0.75rem 1.5rem; background: #6c757d; color: white; border: none; cursor: pointer;") { "Cancel" }
                            button(on:click = assign_role, disabled = assigning_clone, style = "width: auto; padding: 0.75rem 1.5rem; background: #667eea; color: white; border: none; cursor: pointer;") {
                                (if assigning_clone { "Assigning..." } else { "Assign Role" })
                            }
                        }
                    }
                }
            }
        } else { view! {} })

        script {
            r#"
            (function() {
                console.log('Users management page loaded');
            })();
            "#
        }
    }
}

pub fn get_template() -> Template {
    Template::build("admin/users").view(users_page).build()
}
