use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::futures::spawn_local;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_system: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Clone, Copy)]
enum RbacTab {
    Users,
    Roles,
    Permissions,
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

fn roles_management_page() -> View {
    let active_tab = create_signal(RbacTab::Users);
    let loading = create_signal(true);
    let error_message = create_signal(String::new());
    let success_message = create_signal(String::new());
    let is_admin = create_signal(false);

    // Form state for role creation
    let role_name = create_signal(String::new());
    let role_description = create_signal(String::new());
    let creating_role = create_signal(false);

    #[cfg(client)]
    {
        let is_admin = is_admin.clone();
        let loading = loading.clone();

        spawn_local(async move {
            let (session_id, _) = get_session_credentials();
            if session_id == 0 {
                loading.set(false);
                redirect_to_login();
                return;
            }
            // For now, just allow access
            is_admin.set(true);
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        // Keep `loading` true on the engine so the initial SSR markup matches the
        // initial client-side view during hydration.
    }

    let set_tab = move |tab: RbacTab| {
        active_tab.set(tab);
    };

    let create_role = move |_| {
        #[cfg(client)]
        {
            let name = role_name.get_clone();
            let description = role_description.get_clone();
            if name.is_empty() {
                error_message.set("Role name is required".to_string());
                return;
            }

            let creating_role = creating_role.clone();
            let success_message = success_message.clone();
            let error_message = error_message.clone();
            let role_name = role_name.clone();
            let role_description = role_description.clone();

            spawn_local(async move {
                creating_role.set(true);
                error_message.set(String::new());
                success_message.set(String::new());

                let (session_id, session_verifier) = get_session_credentials();

                use gloo_net::http::Request;
                let result = Request::post("/api/permissions/roles")
                    .json(&serde_json::json!({
                        "session_id": session_id,
                        "session_verifier": session_verifier,
                        "name": name,
                        "description": description
                    }))
                    .map_err(|e| e.to_string())
                    .unwrap()
                    .send()
                    .await
                    .map_err(|e| e.to_string());

                match result {
                    Ok(response) => {
                        if response.ok() {
                            success_message.set("Role created successfully".to_string());
                            role_name.set(String::new());
                            role_description.set(String::new());
                        } else {
                            error_message.set("Failed to create role".to_string());
                        }
                    }
                    Err(e) => {
                        error_message.set(format!("Error: {}", e));
                    }
                }
                creating_role.set(false);
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
                Link(to = "/admin/users", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Users" }
                Link(to = "/admin/roles", style = "display: inline-block; padding: 0.5rem 1rem; color: #667eea; background: rgba(102,126,234,0.1); text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Roles" }
                Link(to = "/admin/support", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Support" }
            }
        }

        div(class = "page-body") {
            div(class = "container-fluid") {
                div(class = "card") {
                    div(class = "card-header") {
                        h5 { "Role-Based Access Control" }
                        span(class = "d-block m-t-5", style = "color: #6c757d; font-size: 0.875rem;") { "Manage roles, permissions, and user access" }
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
                                    "Access denied. Please log in as an admin."
                                }
                            }
                        } else {
                            view! {
                                // Tabs using Sycamore signals
                                ul(class = "nav-tabs", style = "margin-bottom: 1.5rem;") {
                                    li(class = "nav-item") {
                                        a(class = if matches!(active_tab.get(), RbacTab::Users) { "nav-link active" } else { "nav-link" },
                                           on:click = move |_| set_tab(RbacTab::Users)) { "Users" }
                                    }
                                    li(class = "nav-item") {
                                        a(class = if matches!(active_tab.get(), RbacTab::Roles) { "nav-link active" } else { "nav-link" },
                                           on:click = move |_| set_tab(RbacTab::Roles)) { "Roles" }
                                    }
                                    li(class = "nav-item") {
                                        a(class = if matches!(active_tab.get(), RbacTab::Permissions) { "nav-link active" } else { "nav-link" },
                                           on:click = move |_| set_tab(RbacTab::Permissions)) { "Permissions" }
                                    }
                                }

                                // Tab content
                                (if matches!(active_tab.get(), RbacTab::Users) {
                                    view! {
                                        div(style = "margin-bottom: 2rem;") {
                                            h6 { "User Management" }
                                            p(class = "text-muted") { "Manage user roles and permissions." }
                                            div(class = "table-responsive") {
                                                table(class = "table") {
                                                    thead {
                                                        tr {
                                                            th { "ID" }
                                                            th { "Email" }
                                                            th { "Roles" }
                                                            th { "Actions" }
                                                        }
                                                    }
                                                    tbody {
                                                        tr {
                                                            td { "1" }
                                                            td { "admin@example.com" }
                                                            td {
                                                                span(class = "badge badge-info", style = "margin-right: 0.25rem;") { "admin" }
                                                            }
                                                            td {
                                                                Link(to = "/admin/users", class = "btn btn-primary btn-sm") { "Manage" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if matches!(active_tab.get(), RbacTab::Roles) {
                                    view! {
                                        div(style = "margin-bottom: 2rem;") {
                                            h6 { "Role Management" }
                                            div(class = "card", style = "margin-bottom: 1.5rem;") {
                                                div(class = "card-header") {
                                                    h6 { "Create New Role" }
                                                }
                                                div(class = "card-body") {
                                                    div(class = "row") {
                                                        div(class = "col-md-4") {
                                                            div(class = "form-group") {
                                                                label(r#for = "role_name") { "Role Name" }
                                                                input(r#type = "text", id = "role_name", bind:value = role_name, class = "form-control", placeholder = "e.g., moderator")
                                                            }
                                                        }
                                                        div(class = "col-md-6") {
                                                            div(class = "form-group") {
                                                                label(r#for = "role_description") { "Description" }
                                                                input(r#type = "text", id = "role_description", bind:value = role_description, class = "form-control", placeholder = "Role description")
                                                            }
                                                        }
                                                        div(class = "col-md-2") {
                                                            div(class = "form-group") {
                                                                label { "&nbsp;" }
                                                                button(on:click = create_role, disabled = creating_role.get(), class = "btn btn-primary") {
                                                                    (if creating_role.get() { "Creating..." } else { "Create" })
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            h6 { "Existing Roles" }
                                            div(class = "table-responsive") {
                                                table(class = "table") {
                                                    thead {
                                                        tr {
                                                            th { "ID" }
                                                            th { "Name" }
                                                            th { "Description" }
                                                            th { "System" }
                                                            th { "Actions" }
                                                        }
                                                    }
                                                    tbody {
                                                        tr {
                                                            td { "1" }
                                                            td { strong { "admin" } }
                                                            td { "Full system access" }
                                                            td { span(class = "badge badge-success") { "Yes" } }
                                                            td {
                                                                button(class = "btn btn-secondary btn-sm", disabled = true) { "Edit" }
                                                                button(class = "btn btn-danger btn-sm", style = "margin-left: 0.25rem;", disabled = true) { "Delete" }
                                                            }
                                                        }
                                                        tr {
                                                            td { "2" }
                                                            td { strong { "editor" } }
                                                            td { "Can edit content" }
                                                            td { span(class = "badge badge-secondary") { "No" } }
                                                            td {
                                                                button(class = "btn btn-secondary btn-sm") { "Edit" }
                                                                button(class = "btn btn-danger btn-sm", style = "margin-left: 0.25rem;") { "Delete" }
                                                            }
                                                        }
                                                        tr {
                                                            td { "3" }
                                                            td { strong { "user" } }
                                                            td { "Basic user access" }
                                                            td { span(class = "badge badge-secondary") { "No" } }
                                                            td {
                                                                button(class = "btn btn-secondary btn-sm") { "Edit" }
                                                                button(class = "btn btn-danger btn-sm", style = "margin-left: 0.25rem;") { "Delete" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if matches!(active_tab.get(), RbacTab::Permissions) {
                                    view! {
                                        div {
                                            h6 { "System Permissions" }
                                            div(class = "table-responsive") {
                                                table(class = "table") {
                                                    thead {
                                                        tr {
                                                            th { "ID" }
                                                            th { "Name" }
                                                            th { "Description" }
                                                        }
                                                    }
                                                    tbody {
                                                        tr {
                                                            td { "1" }
                                                            td { code { "create_posts" } }
                                                            td { "Can create new posts" }
                                                        }
                                                        tr {
                                                            td { "2" }
                                                            td { code { "edit_posts" } }
                                                            td { "Can edit any post" }
                                                        }
                                                        tr {
                                                            td { "3" }
                                                            td { code { "delete_posts" } }
                                                            td { "Can delete any post" }
                                                        }
                                                        tr {
                                                            td { "4" }
                                                            td { code { "manage_users" } }
                                                            td { "Can manage user accounts" }
                                                        }
                                                        tr {
                                                            td { "5" }
                                                            td { code { "view_dashboard" } }
                                                            td { "Can view admin dashboard" }
                                                        }
                                                        tr {
                                                            td { "6" }
                                                            td { code { "manage_support" } }
                                                            td { "Can manage support tickets" }
                                                        }
                                                    }
                                                }
                                            }

                                            div(class = "alert alert-info", style = "margin-top: 1.5rem;") {
                                                h6 { "About Permissions" }
                                                p { "Permissions define what actions users can perform. Each role has a set of permissions. Users inherit permissions from their assigned roles." }
                                            }
                                        }
                                    }
                                } else { view! {} })
                            }
                        })
                    }
                }
            }
        }

        script {
            r#"
            (function() {
                console.log('Roles management page loaded');
            })();
            "#
        }
    }
}

pub fn get_template() -> Template {
    Template::build("admin/roles").view(roles_management_page).build()
}
