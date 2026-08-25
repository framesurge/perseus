use perseus::prelude::*;
use sycamore::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct User {
    id: Option<i64>,
    name: String,
    email: String,
    created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: Option<serde_json::Value>,
}

#[cfg(target_arch = "wasm32")]
use sycamore::futures::spawn_local_scoped;

fn index_page() -> View {
    // Form state
    let name = create_signal(String::new());
    let email = create_signal(String::new());
    let message = create_signal(String::new());
    let message_type = create_signal(String::new());
    let submitting = create_signal(false);

    // Users list state
    let users = create_signal(Vec::<User>::new());
    let loading = create_signal(false);

    // Load users on mount
    #[cfg(target_arch = "wasm32")]
    {
        let users = users.clone();
        spawn_local_scoped(async move {
            if let Ok(user_list) = fetch_users().await {
                users.set(user_list);
            }
        });
    }

    let submit_form = move |_| {
        let name_val = name.get_clone();
        let email_val = email.get_clone();

        if name_val.is_empty() || email_val.is_empty() {
            message.set("Please fill in all fields".to_string());
            message_type.set("error".to_string());
            return;
        }

        submitting.set(true);
        message.set(String::new());

        #[cfg(target_arch = "wasm32")]
        spawn_local_scoped(async move {
            match create_user(name_val, email_val).await {
                Ok(response) => {
                    if response.success {
                        message.set(response.message);
                        message_type.set("success".to_string());
                        name.set(String::new());
                        email.set(String::new());

                        // Reload users
                        if let Ok(user_list) = fetch_users().await {
                            users.set(user_list);
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

    let delete_user_handler = move |id: i64| {
        loading.set(true);

        #[cfg(target_arch = "wasm32")]
        spawn_local_scoped(async move {
            match remove_user(id).await {
                Ok(response) => {
                    if response.success {
                        // Reload users
                        if let Ok(user_list) = fetch_users().await {
                            users.set(user_list);
                        }
                        message.set(response.message);
                        message_type.set("success".to_string());
                    } else {
                        message.set(response.message);
                        message_type.set("error".to_string());
                    }
                }
                Err(e) => {
                    message.set(format!("Error deleting user: {}", e));
                    message_type.set("error".to_string());
                }
            }
            loading.set(false);
        });
    };

    view! {
        div(class = "container") {
            h1 { "User Management System" }

            div(class = "form-card") {
                h2 { "Add New User" }

                form(on:submit = move |e: sycamore::web::events::SubmitEvent| {
                    e.prevent_default();
                    submit_form(());
                }) {
                    div(class = "form-group") {
                        label { "Name:" }
                        input(
                            r#type = "text",
                            placeholder = "Enter name",
                            bind:value = name,
                            disabled = submitting.get()
                        )
                    }

                    div(class = "form-group") {
                        label { "Email:" }
                        input(
                            r#type = "email",
                            placeholder = "Enter email",
                            bind:value = email,
                            disabled = submitting.get()
                        )
                    }

                    button(
                        r#type = "submit",
                        disabled = submitting.get()
                    ) {
                        (if submitting.get() { "Submitting..." } else { "Add User" })
                    }
                }

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

            div(class = "users-card") {
                h2 { "Registered Users" }

                (if users.get_clone().is_empty() {
                    view! {
                        div(class = "empty-state") {
                            p { "No users registered yet. Add your first user above!" }
                        }
                    }
                } else {
                    view! {
                        ul(class = "users-list") {
                            Indexed(
                                list = users,
                                view = move |user| {
                                    let user_id = user.id.unwrap_or(0);
                                    let user_name = user.name.clone();
                                    let user_email = user.email.clone();
                                    view! {
                                        li(class = "user-item") {
                                            div(class = "user-info") {
                                                div(class = "user-name") { (user_name) }
                                                div(class = "user-email") { (user_email) }
                                            }
                                            button(
                                                class = "delete-btn",
                                                on:click = move |_| delete_user_handler(user_id),
                                                disabled = loading.get()
                                            ) {
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            )
                        }
                    }
                })
            }
        }
    }
}

// API functions (client-side only)
#[cfg(target_arch = "wasm32")]
async fn create_user(name: String, email: String) -> Result<ApiResponse, String> {
    use gloo_net::http::Request;

    let response = Request::post("/api/users")
        .json(&serde_json::json!({
            "name": name,
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

#[cfg(target_arch = "wasm32")]
async fn fetch_users() -> Result<Vec<User>, String> {
    use gloo_net::http::Request;

    let response = Request::get("/api/users")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response = response
        .json::<ApiResponse>()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(data) = api_response.data {
        serde_json::from_value(data).map_err(|e| e.to_string())
    } else {
        Ok(Vec::new())
    }
}

#[cfg(target_arch = "wasm32")]
async fn remove_user(id: i64) -> Result<ApiResponse, String> {
    use gloo_net::http::Request;

    let response = Request::delete(&format!("/api/users/{}", id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<ApiResponse>()
        .await
        .map_err(|e| e.to_string())
}

pub fn get_template() -> Template {
    Template::build("index")
        .view(index_page)
        .build()
}
