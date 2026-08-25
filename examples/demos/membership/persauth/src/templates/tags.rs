use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(client)]
use sycamore::futures::spawn_local;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
struct Tag {
    id: i32,
    name: String,
    slug: String,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TagsResponse {
    success: bool,
    message: String,
    data: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TagResponse {
    success: bool,
    message: String,
    data: Option<Tag>,
}

fn tags_page() -> View {
    let tags = create_signal(Vec::<Tag>::new());
    let loading = create_signal(true);
    let error_message = create_signal(String::new());
    let success_message = create_signal(String::new());

    // Editor state
    let show_editor = create_signal(false);
    let editing_tag = create_signal(Option::<Tag>::None);
    let editor_name = create_signal(String::new());
    let saving = create_signal(false);

    // Load tags on mount
    #[cfg(client)]
    {
        let tags = tags.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();

        spawn_local(async move {
            match fetch_tags().await {
                Ok(response) => {
                    if response.success {
                        if let Some(data) = response.data {
                            tags.set(data);
                        }
                    } else {
                        error_message.set(response.message);
                    }
                }
                Err(e) => {
                    error_message.set(format!("Failed to load tags: {}", e));
                }
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        loading.set(false);
    }

    let open_new_tag = {
        let show_editor = show_editor.clone();
        let editing_tag = editing_tag.clone();
        let editor_name = editor_name.clone();

        move |_| {
            editing_tag.set(None);
            editor_name.set(String::new());
            show_editor.set(true);
        }
    };

    let open_edit_tag = {
        let show_editor = show_editor.clone();
        let editing_tag = editing_tag.clone();
        let editor_name = editor_name.clone();

        move |tag: Tag| {
            editor_name.set(tag.name.clone());
            editing_tag.set(Some(tag));
            show_editor.set(true);
        }
    };

    let close_editor = {
        let show_editor = show_editor.clone();
        move |_| {
            show_editor.set(false);
        }
    };

    let save_tag = {
        let tags = tags.clone();
        let editing_tag = editing_tag.clone();
        let editor_name = editor_name.clone();
        let show_editor = show_editor.clone();
        let saving = saving.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();

        move |_| {
            #[cfg(client)]
            {
                let tags = tags.clone();
                let editing_tag = editing_tag.clone();
                let editor_name = editor_name.clone();
                let show_editor = show_editor.clone();
                let saving = saving.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    saving.set(true);
                    error_message.set(String::new());
                    success_message.set(String::new());

                    let name = editor_name.get_clone();

                    let result = if let Some(tag) = editing_tag.get_clone() {
                        update_tag(tag.id, &name).await
                    } else {
                        create_tag(&name).await
                    };

                    match result {
                        Ok(response) => {
                            if response.success {
                                // Close editor and reset saving state first
                                saving.set(false);
                                show_editor.set(false);
                                success_message.set(response.message);

                                // Refresh tags list after modal is closed
                                if let Ok(tags_response) = fetch_tags().await {
                                    if let Some(data) = tags_response.data {
                                        tags.set(data);
                                    }
                                }
                            } else {
                                saving.set(false);
                                error_message.set(response.message);
                            }
                        }
                        Err(e) => {
                            saving.set(false);
                            error_message.set(format!("Failed to save tag: {}", e));
                        }
                    }
                });
            }
        }
    };

    let delete_tag_handler = {
        let tags = tags.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();

        move |tag_id: i32| {
            #[cfg(client)]
            {
                let tags = tags.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    match delete_tag(tag_id).await {
                        Ok(response) => {
                            if response.success {
                                success_message.set("Tag deleted successfully".to_string());
                                // Remove from list
                                let current = tags.get_clone();
                                tags.set(current.into_iter().filter(|t| t.id != tag_id).collect());
                            } else {
                                error_message.set(response.message);
                            }
                        }
                        Err(e) => {
                            error_message.set(format!("Failed to delete tag: {}", e));
                        }
                    }
                });
            }
        }
    };

    view! {
        div(class = "container", style = "max-width: 800px;") {
            div(class = "card") {
                div(style = "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;") {
                    h1(style = "margin: 0;") { "Tags" }
                    button(on:click = open_new_tag, style = "width: auto; padding: 0.75rem 1.5rem;") {
                        "+ New Tag"
                    }
                }

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
                        p(style = "text-align: center; color: #666;") { "Loading tags..." }
                    }
                } else {
                    view! {}
                })

                // Tags list
                (if !loading.get() && tags.get_clone().is_empty() {
                    view! {
                        div(style = "text-align: center; padding: 2rem; color: #666;") {
                            p { "No tags yet. Create your first tag!" }
                        }
                    }
                } else {
                    view! {}
                })

                div(style = "display: flex; flex-wrap: wrap; gap: 0.75rem;") {
                    Indexed(
                        list = tags,
                        view = {
                            let open_edit_tag = open_edit_tag.clone();
                            let delete_tag_handler = delete_tag_handler.clone();
                            move |tag| {
                                let tag_for_edit = tag.clone();
                                let tag_id = tag.id;
                                let open_edit = open_edit_tag.clone();
                                let delete_handler = delete_tag_handler.clone();

                                view! {
                                    div(style = "display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem; background: #f8f9fa; border: 1px solid #e0e0e0; border-radius: 2rem;") {
                                        span(style = "font-weight: 500;") { (tag.name.clone()) }
                                        button(
                                            on:click = move |_| {
                                                open_edit(tag_for_edit.clone());
                                            },
                                            style = "width: auto; padding: 0.25rem 0.5rem; font-size: 0.75rem; background: #667eea; border-radius: 4px;"
                                        ) { "Edit" }
                                        button(
                                            on:click = move |_| {
                                                delete_handler(tag_id);
                                            },
                                            style = "width: auto; padding: 0.25rem 0.5rem; font-size: 0.75rem; background: #dc3545; border-radius: 4px;"
                                        ) { "X" }
                                    }
                                }
                            }
                        }
                    )
                }
            }

            // Editor Modal
            (if show_editor.get() {
                let is_editing = editing_tag.get_clone().is_some();
                let title = if is_editing { "Edit Tag" } else { "New Tag" };
                let close_editor = close_editor.clone();
                let save_tag = save_tag.clone();
                let is_saving = saving.get();

                view! {
                    div(class = "link-modal-overlay") {
                        div(class = "card", style = "min-width: 350px; max-width: 400px;") {
                            div(style = "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;") {
                                h2(style = "margin: 0;") { (title) }
                                button(
                                    on:click = close_editor,
                                    style = "width: auto; padding: 0.5rem 1rem; background: #6c757d;"
                                ) { "Close" }
                            }

                            div(class = "form-group") {
                                label { "Name" }
                                input(
                                    r#type = "text",
                                    bind:value = editor_name,
                                    placeholder = "Tag name..."
                                )
                            }

                            div(style = "display: flex; gap: 1rem; justify-content: flex-end;") {
                                button(
                                    on:click = save_tag,
                                    disabled = is_saving
                                ) {
                                    (if is_saving { "Saving..." } else { "Save Tag" })
                                }
                            }
                        }
                    }
                }
            } else {
                view! {}
            })

            div(class = "nav-links", style = "margin-top: 1.5rem;") {
                a(href = "/") { "Back to Home" }
            }
        }
    }
}

#[cfg(client)]
async fn fetch_tags() -> Result<TagsResponse, String> {
    use gloo_net::http::Request;

    let response = Request::get("/api/tags")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<TagsResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(client)]
async fn create_tag(name: &str) -> Result<TagResponse, String> {
    use gloo_net::http::Request;

    // Generate slug from name
    let slug = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    let response = Request::post("/api/tags")
        .json(&serde_json::json!({
            "name": name,
            "slug": slug
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<TagResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(client)]
async fn update_tag(id: i32, name: &str) -> Result<TagResponse, String> {
    use gloo_net::http::Request;

    let response = Request::patch(&format!("/api/tags/{}", id))
        .json(&serde_json::json!({
            "name": name
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<TagResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(client)]
async fn delete_tag(id: i32) -> Result<TagResponse, String> {
    use gloo_net::http::Request;

    let response = Request::delete(&format!("/api/tags/{}", id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<TagResponse>()
        .await
        .map_err(|e| e.to_string())
}

pub fn get_template() -> Template {
    Template::build("tags").view(tags_page).build()
}
