use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(client)]
use sycamore::futures::spawn_local;

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
struct UserWithRoles {
    user_id: i32,
    email: String,
    roles: Vec<String>,
    permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Session {
    session_id: i32,
    session_verifier: String,
}

fn profile_page() -> View {
    let user_profile = create_signal(Option::<UserProfile>::None);
    let user_roles = create_signal(Vec::<String>::new());
    let user_permissions = create_signal(Vec::<String>::new());
    let loading = create_signal(true);
    let error_message = create_signal(String::new());
    let success_message = create_signal(String::new());

    // Password change fields
    let current_password = create_signal(String::new());
    let new_password = create_signal(String::new());
    let confirm_password = create_signal(String::new());
    let changing_password = create_signal(false);

    // Load user profile on mount
    #[cfg(client)]
    {
        let user_profile = user_profile.clone();
        let user_roles = user_roles.clone();
        let user_permissions = user_permissions.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();

        spawn_local(async move {
            if let Some(session) = get_stored_session() {
                // Get profile
                match get_profile(&session).await {
                    Ok(response) => {
                        if response.success {
                            if let Some(profile) = response.data {
                                // Fetch roles and permissions
                                if let Ok(perms_response) =
                                    get_user_permissions(profile.id).await
                                {
                                    if perms_response.success {
                                        if let Some(data) = perms_response.data {
                                            user_roles.set(
                                                data.get("roles")
                                                    .and_then(|r| r.as_array())
                                                    .map(|arr| {
                                                        arr.iter()
                                                            .filter_map(|v| v.as_str().map(String::from))
                                                            .collect()
                                                    })
                                                    .unwrap_or_default(),
                                            );
                                            user_permissions.set(
                                                data.get("permissions")
                                                    .and_then(|p| p.as_array())
                                                    .map(|arr| {
                                                        arr.iter()
                                                            .filter_map(|v| v.as_str().map(String::from))
                                                            .collect()
                                                    })
                                                    .unwrap_or_default(),
                                            );
                                        }
                                    }
                                }
                                user_profile.set(Some(profile));
                            }
                        } else {
                            error_message.set(response.message);
                        }
                    }
                    Err(e) => {
                        error_message.set(format!("Failed to load profile: {}", e));
                    }
                }
            } else {
                error_message.set("Not logged in. Please log in first.".to_string());
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        loading.set(false);
    }

    let handle_password_change = {
        let current_password = current_password.clone();
        let new_password = new_password.clone();
        let confirm_password = confirm_password.clone();
        let changing_password = changing_password.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();

        move |_| {
            #[cfg(client)]
            {
                let current_password = current_password.clone();
                let new_password = new_password.clone();
                let confirm_password = confirm_password.clone();
                let changing_password = changing_password.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    let new_pw = new_password.get_clone();
                    let confirm_pw = confirm_password.get_clone();

                    if new_pw != confirm_pw {
                        error_message.set("Passwords do not match".to_string());
                        return;
                    }

                    if new_pw.len() < 8 {
                        error_message.set("Password must be at least 8 characters".to_string());
                        return;
                    }

                    changing_password.set(true);
                    error_message.set(String::new());

                    // Note: Password change API endpoint would need to be implemented
                    // For now, just show a message
                    success_message.set("Password change feature coming soon".to_string());
                    changing_password.set(false);

                    // Clear fields
                    current_password.set(String::new());
                    new_password.set(String::new());
                    confirm_password.set(String::new());
                });
            }
        }
    };

    view! {
        div(class = "container", style = "max-width: 600px;") {
            div(class = "card") {
                h1 { "Profile" }

                // Messages
                (if !success_message.get_clone().is_empty() {
                    let msg = success_message.get_clone();
                    view! {
                        div(class = "message success") { (msg) }
                    }
                } else {
                    view! {}
                })

                (if !error_message.get_clone().is_empty() {
                    let msg = error_message.get_clone();
                    view! {
                        div(class = "message error") { (msg) }
                    }
                } else {
                    view! {}
                })

                // Loading state
                (if loading.get() {
                    view! {
                        p(style = "text-align: center; color: #666;") { "Loading profile..." }
                    }
                } else {
                    view! {}
                })

                // Profile info
                (if let Some(profile) = user_profile.get_clone() {
                    view! {
                        div(style = "margin-bottom: 2rem;") {
                            h2(style = "font-size: 1.2rem; margin-bottom: 1rem;") { "Account Information" }

                            div(style = "background: #f8f9fa; padding: 1rem; border-radius: 8px;") {
                                p(style = "margin-bottom: 0.5rem;") {
                                    strong { "Email: " }
                                    (profile.email)
                                }
                                (if let Some(created) = &profile.created_at {
                                    let date = created.clone();
                                    view! {
                                        p(style = "margin-bottom: 0.5rem;") {
                                            strong { "Member since: " }
                                            (date)
                                        }
                                    }
                                } else {
                                    view! {}
                                })
                                p {
                                    strong { "Email verified: " }
                                    (if profile.otp_confirmed { "Yes" } else { "No" })
                                }
                            }
                        }

                        // Roles section
                        div(style = "margin-bottom: 2rem;") {
                            h2(style = "font-size: 1.2rem; margin-bottom: 1rem;") { "Roles" }

                            (if user_roles.get_clone().is_empty() {
                                view! {
                                    p(style = "color: #666;") { "No roles assigned" }
                                }
                            } else {
                                let roles = user_roles.get_clone();
                                view! {
                                    div(style = "display: flex; flex-wrap: wrap; gap: 0.5rem;") {
                                        Indexed(
                                            list = create_signal(roles),
                                            view = move |role| {
                                                view! {
                                                    span(style = "display: inline-block; padding: 0.25rem 0.75rem; background: #667eea; color: white; border-radius: 1rem; font-size: 0.875rem;") {
                                                        (role)
                                                    }
                                                }
                                            }
                                        )
                                    }
                                }
                            })
                        }

                        // Permissions section
                        div(style = "margin-bottom: 2rem;") {
                            h2(style = "font-size: 1.2rem; margin-bottom: 1rem;") { "Permissions" }

                            (if user_permissions.get_clone().is_empty() {
                                view! {
                                    p(style = "color: #666;") { "No permissions" }
                                }
                            } else {
                                let permissions = user_permissions.get_clone();
                                view! {
                                    div(style = "display: flex; flex-wrap: wrap; gap: 0.5rem;") {
                                        Indexed(
                                            list = create_signal(permissions),
                                            view = move |perm| {
                                                view! {
                                                    span(style = "display: inline-block; padding: 0.25rem 0.75rem; background: #28a745; color: white; border-radius: 1rem; font-size: 0.75rem;") {
                                                        (perm)
                                                    }
                                                }
                                            }
                                        )
                                    }
                                }
                            })
                        }

                        // Password change section
                        div(style = "border-top: 1px solid #e0e0e0; padding-top: 2rem;") {
                            h2(style = "font-size: 1.2rem; margin-bottom: 1rem;") { "Change Password" }

                            div(class = "form-group") {
                                label { "Current Password" }
                                input(
                                    r#type = "password",
                                    bind:value = current_password,
                                    placeholder = "Enter current password"
                                )
                            }

                            div(class = "form-group") {
                                label { "New Password" }
                                input(
                                    r#type = "password",
                                    bind:value = new_password,
                                    placeholder = "Enter new password"
                                )
                            }

                            div(class = "form-group") {
                                label { "Confirm New Password" }
                                input(
                                    r#type = "password",
                                    bind:value = confirm_password,
                                    placeholder = "Confirm new password"
                                )
                            }

                            button(
                                on:click = handle_password_change,
                                disabled = changing_password.get()
                            ) {
                                (if changing_password.get() { "Changing..." } else { "Change Password" })
                            }
                        }
                    }
                } else if !loading.get() {
                    view! {
                        div(style = "text-align: center; padding: 2rem;") {
                            p(style = "color: #666;") { "Please log in to view your profile." }
                            a(href = "/login", style = "color: #667eea;") { "Go to Login" }
                        }
                    }
                } else {
                    view! {}
                })
            }

            div(class = "nav-links", style = "margin-top: 1.5rem;") {
                a(href = "/") { "Back to Home" }
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
async fn get_user_permissions(user_id: i32) -> Result<ApiResponse<serde_json::Value>, String> {
    use gloo_net::http::Request;

    let response = Request::get(&format!("/api/permissions/users/{}/permissions", user_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<ApiResponse<serde_json::Value>>()
        .await
        .map_err(|e| e.to_string())
}

pub fn get_template() -> Template {
    Template::build("profile").view(profile_page).build()
}
