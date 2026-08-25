use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(client)]
use sycamore::futures::spawn_local;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
struct Category {
    id: i32,
    name: String,
    slug: String,
    description: String,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CategoriesResponse {
    success: bool,
    message: String,
    data: Option<Vec<Category>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CategoryResponse {
    success: bool,
    message: String,
    data: Option<Category>,
}

fn categories_page() -> View {
    let categories = create_signal(Vec::<Category>::new());
    let loading = create_signal(true);
    let error_message = create_signal(String::new());
    let success_message = create_signal(String::new());

    // Editor state
    let show_editor = create_signal(false);
    let editing_category = create_signal(Option::<Category>::None);
    let editor_name = create_signal(String::new());
    let editor_description = create_signal(String::new());
    let saving = create_signal(false);

    // Load categories on mount
    #[cfg(client)]
    {
        let categories = categories.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();

        spawn_local(async move {
            match fetch_categories().await {
                Ok(response) => {
                    if response.success {
                        if let Some(data) = response.data {
                            categories.set(data);
                        }
                    } else {
                        error_message.set(response.message);
                    }
                }
                Err(e) => {
                    error_message.set(format!("Failed to load categories: {}", e));
                }
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        loading.set(false);
    }

    let open_new_category = {
        let show_editor = show_editor.clone();
        let editing_category = editing_category.clone();
        let editor_name = editor_name.clone();
        let editor_description = editor_description.clone();

        move |_| {
            editing_category.set(None);
            editor_name.set(String::new());
            editor_description.set(String::new());
            show_editor.set(true);
        }
    };

    let open_edit_category = {
        let show_editor = show_editor.clone();
        let editing_category = editing_category.clone();
        let editor_name = editor_name.clone();
        let editor_description = editor_description.clone();

        move |category: Category| {
            editor_name.set(category.name.clone());
            editor_description.set(category.description.clone());
            editing_category.set(Some(category));
            show_editor.set(true);
        }
    };

    let close_editor = {
        let show_editor = show_editor.clone();
        move |_| {
            show_editor.set(false);
        }
    };

    let save_category = {
        let categories = categories.clone();
        let editing_category = editing_category.clone();
        let editor_name = editor_name.clone();
        let editor_description = editor_description.clone();
        let show_editor = show_editor.clone();
        let saving = saving.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();

        move |_| {
            #[cfg(client)]
            {
                let categories = categories.clone();
                let editing_category = editing_category.clone();
                let editor_name = editor_name.clone();
                let editor_description = editor_description.clone();
                let show_editor = show_editor.clone();
                let saving = saving.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    saving.set(true);
                    error_message.set(String::new());
                    success_message.set(String::new());

                    let name = editor_name.get_clone();
                    let description = editor_description.get_clone();

                    let result = if let Some(category) = editing_category.get_clone() {
                        update_category(category.id, &name, &description).await
                    } else {
                        create_category(&name, &description).await
                    };

                    match result {
                        Ok(response) => {
                            if response.success {
                                // Close editor and reset saving state first
                                saving.set(false);
                                show_editor.set(false);
                                success_message.set(response.message);

                                // Refresh categories list after modal is closed
                                if let Ok(cats_response) = fetch_categories().await {
                                    if let Some(data) = cats_response.data {
                                        categories.set(data);
                                    }
                                }
                            } else {
                                saving.set(false);
                                error_message.set(response.message);
                            }
                        }
                        Err(e) => {
                            saving.set(false);
                            error_message.set(format!("Failed to save category: {}", e));
                        }
                    }
                });
            }
        }
    };

    let delete_category_handler = {
        let categories = categories.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();

        move |category_id: i32| {
            #[cfg(client)]
            {
                let categories = categories.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    match delete_category(category_id).await {
                        Ok(response) => {
                            if response.success {
                                success_message.set("Category deleted successfully".to_string());
                                // Remove from list
                                let current = categories.get_clone();
                                categories.set(
                                    current.into_iter().filter(|c| c.id != category_id).collect(),
                                );
                            } else {
                                error_message.set(response.message);
                            }
                        }
                        Err(e) => {
                            error_message.set(format!("Failed to delete category: {}", e));
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
                    h1(style = "margin: 0;") { "Categories" }
                    button(on:click = open_new_category, style = "width: auto; padding: 0.75rem 1.5rem;") {
                        "+ New Category"
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
                        p(style = "text-align: center; color: #666;") { "Loading categories..." }
                    }
                } else {
                    view! {}
                })

                // Categories list
                (if !loading.get() && categories.get_clone().is_empty() {
                    view! {
                        div(style = "text-align: center; padding: 2rem; color: #666;") {
                            p { "No categories yet. Create your first category!" }
                        }
                    }
                } else {
                    view! {}
                })

                div(style = "display: flex; flex-direction: column; gap: 1rem;") {
                    Indexed(
                        list = categories,
                        view = {
                            let open_edit_category = open_edit_category.clone();
                            let delete_category_handler = delete_category_handler.clone();
                            move |category| {
                                let category_for_edit = category.clone();
                                let category_id = category.id;
                                let open_edit = open_edit_category.clone();
                                let delete_handler = delete_category_handler.clone();

                                view! {
                                    div(style = "border: 1px solid #e0e0e0; border-radius: 8px; padding: 1rem;") {
                                        div(style = "display: flex; justify-content: space-between; align-items: start;") {
                                            div {
                                                h3(style = "margin: 0 0 0.5rem 0;") { (category.name.clone()) }
                                                p(style = "color: #666; margin: 0; font-size: 0.9rem;") { (category.description.clone()) }
                                                p(style = "color: #999; margin: 0.25rem 0 0 0; font-size: 0.75rem;") {
                                                    "Slug: " (category.slug.clone())
                                                }
                                            }
                                            div(style = "display: flex; gap: 0.5rem;") {
                                                button(
                                                    on:click = move |_| {
                                                        open_edit(category_for_edit.clone());
                                                    },
                                                    style = "width: auto; padding: 0.5rem 1rem; font-size: 0.875rem; background: #667eea;"
                                                ) { "Edit" }
                                                button(
                                                    on:click = move |_| {
                                                        delete_handler(category_id);
                                                    },
                                                    style = "width: auto; padding: 0.5rem 1rem; font-size: 0.875rem; background: #dc3545;"
                                                ) { "Delete" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    )
                }
            }

            // Editor Modal
            (if show_editor.get() {
                let is_editing = editing_category.get_clone().is_some();
                let title = if is_editing { "Edit Category" } else { "New Category" };
                let close_editor = close_editor.clone();
                let save_category = save_category.clone();
                let is_saving = saving.get();

                view! {
                    div(class = "link-modal-overlay") {
                        div(class = "card", style = "min-width: 400px; max-width: 500px;") {
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
                                    placeholder = "Category name..."
                                )
                            }

                            div(class = "form-group") {
                                label { "Description" }
                                input(
                                    r#type = "text",
                                    bind:value = editor_description,
                                    placeholder = "Category description..."
                                )
                            }

                            div(style = "display: flex; gap: 1rem; justify-content: flex-end;") {
                                button(
                                    on:click = save_category,
                                    disabled = is_saving
                                ) {
                                    (if is_saving { "Saving..." } else { "Save Category" })
                                }
                            }
                        }
                    }
                }
            } else {
                view! {}
            })

            div(class = "nav-links", style = "margin-top: 1.5rem;") {
                Link(to = "/") { "Back to Home" }
            }
        }
    }
}

#[cfg(client)]
async fn fetch_categories() -> Result<CategoriesResponse, String> {
    use gloo_net::http::Request;

    let response = Request::get("/api/categories")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<CategoriesResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(client)]
async fn create_category(name: &str, description: &str) -> Result<CategoryResponse, String> {
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

    let response = Request::post("/api/categories")
        .json(&serde_json::json!({
            "name": name,
            "slug": slug,
            "description": description
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<CategoryResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(client)]
async fn update_category(id: i32, name: &str, description: &str) -> Result<CategoryResponse, String> {
    use gloo_net::http::Request;

    let response = Request::patch(&format!("/api/categories/{}", id))
        .json(&serde_json::json!({
            "name": name,
            "description": description
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<CategoryResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(client)]
async fn delete_category(id: i32) -> Result<CategoryResponse, String> {
    use gloo_net::http::Request;

    let response = Request::delete(&format!("/api/categories/{}", id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<CategoryResponse>()
        .await
        .map_err(|e| e.to_string())
}

pub fn get_template() -> Template {
    Template::build("categories").view(categories_page).build()
}
