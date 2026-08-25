use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(client)]
use sycawysgy::{Editor, Delta, render_delta_to_html};

#[cfg(client)]
use sycamore::futures::spawn_local;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PostResponse {
    success: bool,
    message: String,
    data: Option<CreatePostResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreatePostResponse {
    id: i32,
    title: String,
    slug: String,
}

fn create_post_page() -> View {
    let success_message = create_signal(String::new());
    let error_message = create_signal(String::new());
    let saving = create_signal(false);
    let is_authenticated = create_signal(false);
    let show_login_prompt = create_signal(false);

    // Editor state
    let editor_title = create_signal(String::new());
    let editor_summary = create_signal(String::new());
    let editor_content = create_signal(String::new());
    let editor_meta_title = create_signal(String::new());
    let editor_meta_description = create_signal(String::new());
    let editor_meta_keywords = create_signal(String::new());
    let editor_og_image = create_signal(String::new());
    let editor_is_published = create_signal(false);
    let show_seo_section = create_signal(false);

    // Check auth on mount
    #[cfg(client)]
    {
        let is_authenticated = is_authenticated.clone();

        spawn_local(async move {
            is_authenticated.set(is_authenticated_fn());
        });

        // Ensure the editor starts empty (sycawysgy loads persisted content from IndexedDB).
        spawn_local(async move {
            let _ = overwrite_editor_document(Delta::new().insert("\n")).await;
        });
    }

    #[cfg(engine)]
    {
        is_authenticated.set(false);
    }

    let save_post = {
        let editor_title = editor_title.clone();
        let editor_summary = editor_summary.clone();
        let editor_meta_title = editor_meta_title.clone();
        let editor_meta_description = editor_meta_description.clone();
        let editor_meta_keywords = editor_meta_keywords.clone();
        let editor_og_image = editor_og_image.clone();
        let editor_is_published = editor_is_published.clone();
        let saving = saving.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();
        let show_login_prompt = show_login_prompt.clone();

        move |_| {
            #[cfg(client)]
            {
                // Check authentication first
                if !is_authenticated_fn() {
                    show_login_prompt.set(true);
                    return;
                }

                let editor_title = editor_title.clone();
                let editor_summary = editor_summary.clone();
                let editor_meta_title = editor_meta_title.clone();
                let editor_meta_description = editor_meta_description.clone();
                let editor_meta_keywords = editor_meta_keywords.clone();
                let editor_og_image = editor_og_image.clone();
                let editor_is_published = editor_is_published.clone();
                let saving = saving.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    saving.set(true);
                    error_message.set(String::new());
                    success_message.set(String::new());

                    // See `posts.rs` for details. We read the HTML directly from the editor DOM
                    // and attempt to load the latest Delta from IndexedDB (best-effort).
                    let delta =
                        load_editor_delta().await.unwrap_or_else(|_| Delta::new().insert("\n"));
                    let content_delta: serde_json::Value =
                        serde_json::to_value(&delta).unwrap_or_default();
                    let content_html = get_editor_html("#create-post-editor");
                    let content = if content_html.trim().is_empty() {
                        render_delta_to_html(&delta)
                    } else {
                        content_html
                    };

                    let title = editor_title.get_clone();
                    let summary = editor_summary.get_clone();
                    let meta_title = editor_meta_title.get_clone();
                    let meta_description = editor_meta_description.get_clone();
                    let meta_keywords = editor_meta_keywords.get_clone();
                    let og_image = editor_og_image.get_clone();
                    let is_published = editor_is_published.get();

                    if title.is_empty() {
                        error_message.set("Please enter a title for your post".to_string());
                        saving.set(false);
                        return;
                    }

                    if summary.is_empty() {
                        error_message.set("Please enter a summary for your post".to_string());
                        saving.set(false);
                        return;
                    }

                    let result = create_post(
                        &title,
                        &summary,
                        &content,
                        &content_delta,
                        &meta_title,
                        &meta_description,
                        &meta_keywords,
                        &og_image,
                        is_published,
                    )
                    .await;

                    match result {
                        Ok(response) => {
                            if response.success {
                                success_message.set(response.message);
                                // Reset form
                                editor_title.set(String::new());
                                editor_summary.set(String::new());
                                editor_meta_title.set(String::new());
                                editor_meta_description.set(String::new());
                                editor_meta_keywords.set(String::new());
                                editor_og_image.set(String::new());
                                editor_is_published.set(false);
                                let _ = overwrite_editor_document(Delta::new().insert("\n")).await;
                            } else {
                                error_message.set(response.message);
                            }
                        }
                        Err(e) => {
                            error_message.set(format!("Failed to create post: {}", e));
                        }
                    }
                    saving.set(false);
                });
            }
        }
    };

    let toggle_seo_section = {
        let show_seo_section = show_seo_section.clone();
        move |_| {
            show_seo_section.set(!show_seo_section.get());
        }
    };

    let close_login_prompt = {
        let show_login_prompt = show_login_prompt.clone();
        move |_| {
            show_login_prompt.set(false);
        }
    };

    view! {
        div(class = "container", style = "max-width: 900px;") {
            div(class = "card") {
                div(style = "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;") {
                    h1(style = "margin: 0;") { "Create New Post" }
                    Link(to = "/posts", style = "display: inline-flex; align-items: center; padding: 0.5rem 1rem; background: #6c757d; color: white; border-radius: 8px; text-decoration: none; font-weight: 500; font-size: 0.875rem;") {
                        "Back to Posts"
                    }
                }

                // Messages
                (if !success_message.get_clone().is_empty() {
                    let msg = success_message.get_clone();
                    view! {
                        div(class = "alert alert-success") { (msg) }
                    }
                } else {
                    view! {}
                })

                (if !error_message.get_clone().is_empty() {
                    let msg = error_message.get_clone();
                    view! {
                        div(class = "alert alert-danger") { (msg) }
                    }
                } else {
                    view! {}
                })

                div(class = "form-group") {
                    label { "Title" }
                    input(
                        r#type = "text",
                        bind:value = editor_title,
                        placeholder = "Enter post title...",
                        class = "form-control"
                    )
                }

                div(class = "form-group") {
                    label { "Summary" }
                    input(
                        r#type = "text",
                        bind:value = editor_summary,
                        placeholder = "Brief summary for previews...",
                        class = "form-control"
                    )
                    p(style = "color: #999; font-size: 0.8rem; margin-top: 0.25rem;") {
                        "A short description that appears in post listings and social shares."
                    }
                }

                div(class = "form-group") {
                    label { "Content" }

                    ({
                        #[cfg(client)]
                        {
                            view! {
                                div(id = "create-post-editor") {
                                    Editor {}
                                }
                            }
                        }
                        #[cfg(engine)]
                        {
                            view! {
                                div(style = "padding: 1rem; background: #f5f5f5; border-radius: 8px; text-align: center;") {
                                    "Editor will load here..."
                                }
                            }
                        }
                    })
                }

                // SEO Section (collapsible)
                div(style = "border-top: 1px solid #e0e0e0; margin-top: 1rem; padding-top: 1rem;") {
                    button(
                        on:click = toggle_seo_section,
                        style = "width: auto; padding: 0.5rem 1rem; background: transparent; color: #667eea; border: 1px solid #667eea; margin-bottom: 1rem;"
                    ) {
                        (if show_seo_section.get() { "Hide SEO Settings" } else { "Show SEO Settings" })
                    }

                    (if show_seo_section.get() {
                        view! {
                            div {
                                div(class = "form-group") {
                                    label { "Meta Title (for search engines)" }
                                    input(
                                        r#type = "text",
                                        bind:value = editor_meta_title,
                                        placeholder = "Leave empty to use post title...",
                                        class = "form-control"
                                    )
                                    p(style = "color: #999; font-size: 0.8rem; margin-top: 0.25rem;") {
                                        "Recommended: 50-60 characters"
                                    }
                                }

                                div(class = "form-group") {
                                    label { "Meta Description" }
                                    input(
                                        r#type = "text",
                                        bind:value = editor_meta_description,
                                        placeholder = "Description for search results...",
                                        class = "form-control"
                                    )
                                    p(style = "color: #999; font-size: 0.8rem; margin-top: 0.25rem;") {
                                        "Recommended: 150-160 characters"
                                    }
                                }

                                div(class = "form-group") {
                                    label { "Meta Keywords" }
                                    input(
                                        r#type = "text",
                                        bind:value = editor_meta_keywords,
                                        placeholder = "keyword1, keyword2, keyword3...",
                                        class = "form-control"
                                    )
                                    p(style = "color: #999; font-size: 0.8rem; margin-top: 0.25rem;") {
                                        "Comma-separated keywords"
                                    }
                                }

                                div(class = "form-group") {
                                    label { "Open Graph Image URL" }
                                    input(
                                        r#type = "text",
                                        bind:value = editor_og_image,
                                        placeholder = "https://example.com/image.jpg",
                                        class = "form-control"
                                    )
                                    p(style = "color: #999; font-size: 0.8rem; margin-top: 0.25rem;") {
                                        "Image shown when shared on social media"
                                    }
                                }
                            }
                        }
                    } else {
                        view! {}
                    })
                }

                // Publishing section
                div(style = "border-top: 1px solid #e0e0e0; margin-top: 1rem; padding-top: 1rem;") {
                    div(style = "display: flex; align-items: center; gap: 0.5rem; margin-bottom: 1rem;") {
                        input(
                            r#type = "checkbox",
                            bind:checked = editor_is_published,
                            style = "width: auto;"
                        )
                        label(style = "margin: 0;") { "Publish this post" }
                    }
                    p(style = "color: #666; font-size: 0.875rem;") {
                        "Unpublished posts will be saved as drafts and won't be visible to visitors."
                    }
                }

                div(style = "display: flex; gap: 1rem; justify-content: flex-end; margin-top: 1.5rem;") {
                    Link(
                        to = "/posts",
                        style = "display: inline-flex; align-items: center; padding: 0.75rem 1.5rem; background: #6c757d; color: white; border-radius: 8px; text-decoration: none;"
                    ) { "Cancel" }
                    button(
                        on:click = save_post,
                        disabled = saving.get(),
                        style = "display: inline-flex; align-items: center; padding: 0.75rem 1.5rem; background: #667eea; color: white; border: none; border-radius: 8px; cursor: pointer;"
                    ) {
                        (if saving.get() { "Saving..." } else { "Save Post" })
                    }
                }
            }

            // Login Prompt Modal
            (if show_login_prompt.get() {
                view! {
                    div(class = "link-modal-overlay") {
                        div(class = "card", style = "max-width: 400px; text-align: center;") {
                            h2(style = "margin-bottom: 1rem;") { "Authentication Required" }
                            p(style = "color: #666; margin-bottom: 1.5rem;") {
                                "You need to be logged in to create posts. Please log in to continue."
                            }
                            div(style = "display: flex; gap: 1rem; justify-content: center;") {
                                Link(
                                    to = "/login",
                                    style = "display: inline-flex; align-items: center; padding: 0.75rem 1.5rem; background: #667eea; color: white; border-radius: 8px; text-decoration: none; font-weight: 500;"
                                ) { "Login" }
                                button(
                                    on:click = close_login_prompt,
                                    style = "width: auto; padding: 0.75rem 1.5rem; background: #6c757d; color: white; border: none; border-radius: 8px; cursor: pointer;"
                                ) { "Cancel" }
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
async fn create_post(
    title: &str,
    summary: &str,
    content: &str,
    content_delta: &serde_json::Value,
    meta_title: &str,
    meta_description: &str,
    meta_keywords: &str,
    og_image: &str,
    is_published: bool,
) -> Result<PostResponse, String> {
    use gloo_net::http::Request;

    // Generate slug from title
    let slug = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    // Get session credentials
    let (session_id, session_verifier) = get_session_credentials();

    let response = Request::post("/api/posts")
        .json(&serde_json::json!({
            "session_id": session_id,
            "session_verifier": session_verifier,
            "title": title,
            "slug": slug,
            "summary": summary,
            "content": content,
            "content_delta": content_delta,
            "meta_title": if meta_title.is_empty() { None } else { Some(meta_title) },
            "meta_description": if meta_description.is_empty() { None } else { Some(meta_description) },
            "meta_keywords": if meta_keywords.is_empty() { None } else { Some(meta_keywords) },
            "og_image": if og_image.is_empty() { None } else { Some(og_image) },
            "is_published": is_published
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<PostResponse>()
        .await
        .map_err(|e| e.to_string())
}

// Session management helpers (client-side)

#[cfg(client)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionData {
    session_id: i32,
    session_verifier: String,
}

#[cfg(client)]
fn get_session_credentials() -> (i32, String) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return (0, String::new()),
    };

    let storage = match window.local_storage() {
        Ok(Some(s)) => s,
        _ => return (0, String::new()),
    };

    let session_id = storage
        .get_item("session_id")
        .ok()
        .flatten()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let session_verifier = storage
        .get_item("session_verifier")
        .ok()
        .flatten()
        .unwrap_or_default();

    (session_id, session_verifier)
}

#[cfg(client)]
fn is_authenticated() -> bool {
    let (session_id, session_verifier) = get_session_credentials();
    session_id > 0 && !session_verifier.is_empty()
}

#[cfg(client)]
fn is_authenticated_fn() -> bool {
    is_authenticated()
}

// sycawysgy bridge helpers (client-side).
//
// sycawysgy's `EditorState` is provided via a *child* context inside the `Editor` component,
// which this template can't access (parents can't read child-provided context).
// For this demo, we use IndexedDB as the transfer mechanism for Delta and read editor HTML from
// the DOM for the rendered content.
#[cfg(client)]
fn get_editor_html(container_selector: &str) -> String {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return String::new(),
    };
    let selector = format!("{container_selector} .editor-content");
    match document.query_selector(&selector) {
        Ok(Some(el)) => el.inner_html(),
        _ => String::new(),
    }
}

#[cfg(client)]
async fn overwrite_editor_document(delta: Delta) -> Result<(), String> {
    let rexie = sycawysgy::storage::init_db().await?;
    let doc = sycawysgy::Document::with_content(
        "default".to_string(),
        "Untitled".to_string(),
        delta,
    );
    sycawysgy::storage::save_document(&rexie, &doc).await
}

#[cfg(client)]
async fn load_editor_delta() -> Result<Delta, String> {
    let rexie = sycawysgy::storage::init_db().await?;
    let doc = sycawysgy::storage::load_document(&rexie).await?;
    Ok(doc
        .map(|d| d.content)
        .unwrap_or_else(|| Delta::new().insert("\n")))
}

pub fn get_template() -> Template {
    Template::build("posts/create").view(create_post_page).build()
}
