use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(client)]
use sycawysgy::{Editor, Delta, render_delta_to_html};

#[cfg(client)]
use sycamore::futures::spawn_local;

#[cfg(client)]
use wasm_bindgen::JsValue;

#[cfg(client)]
use gloo_storage::Storage;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
struct Post {
    id: i32,
    title: String,
    slug: String,
    summary: String,
    content: String,
    // Delta JSON content from sycawysgy editor
    content_delta: Option<serde_json::Value>,
    // SEO fields
    meta_title: Option<String>,
    meta_description: Option<String>,
    meta_keywords: Option<String>,
    og_image: Option<String>,
    canonical_url: Option<String>,
    // Publishing
    is_published: bool,
    published_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PostsResponse {
    success: bool,
    message: String,
    data: Option<Vec<Post>>,
    total: Option<i64>,
    page: Option<i64>,
    per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PostResponse {
    success: bool,
    message: String,
    data: Option<Post>,
}

fn posts_page() -> View {
    let posts = create_signal(Vec::<Post>::new());
    let loading = create_signal(true);
    let error_message = create_signal(String::new());
    let success_message = create_signal(String::new());

    // Auth state
    let is_authenticated = create_signal(false);
    let show_login_prompt = create_signal(false);

    // Editor state
    let show_editor = create_signal(false);
    let editing_post = create_signal(Option::<Post>::None);
    let editor_title = create_signal(String::new());
    let editor_summary = create_signal(String::new());
    let editor_content = create_signal(String::new());
    // SEO fields
    let editor_meta_title = create_signal(String::new());
    let editor_meta_description = create_signal(String::new());
    let editor_meta_keywords = create_signal(String::new());
    let editor_og_image = create_signal(String::new());
    let editor_is_published = create_signal(false);
    let show_seo_section = create_signal(false);
    let saving = create_signal(false);

    // Check auth on mount
    #[cfg(client)]
    {
        let is_authenticated = is_authenticated.clone();

        spawn_local(async move {
            is_authenticated.set(is_authenticated_fn());
        });
    }

    // Load posts on mount
    #[cfg(client)]
    {
        let posts = posts.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();

        spawn_local(async move {
            match fetch_posts().await {
                Ok(response) => {
                    if response.success {
                        if let Some(data) = response.data {
                            posts.set(data);
                        }
                    } else {
                        error_message.set(response.message);
                    }
                }
                Err(e) => {
                    error_message.set(format!("Failed to load posts: {}", e));
                }
            }
            loading.set(false);
        });
    }

    #[cfg(engine)]
    {
        loading.set(false);
    }

    let open_new_post = {
        let show_editor = show_editor.clone();
        let editing_post = editing_post.clone();
        let editor_title = editor_title.clone();
        let editor_summary = editor_summary.clone();
        let editor_content = editor_content.clone();
        let editor_meta_title = editor_meta_title.clone();
        let editor_meta_description = editor_meta_description.clone();
        let editor_meta_keywords = editor_meta_keywords.clone();
        let editor_og_image = editor_og_image.clone();
        let editor_is_published = editor_is_published.clone();
        let show_seo_section = show_seo_section.clone();
        let show_login_prompt = show_login_prompt.clone();
        let is_authenticated = is_authenticated.clone();

        move |_| {
            // Check authentication first
            #[cfg(client)]
            {
                if !is_authenticated_fn() {
                    show_login_prompt.set(true);
                    return;
                }
            }
            editing_post.set(None);
            editor_title.set(String::new());
            editor_summary.set(String::new());
            editor_content.set(String::new());
            editor_meta_title.set(String::new());
            editor_meta_description.set(String::new());
            editor_meta_keywords.set(String::new());
            editor_og_image.set(String::new());
            editor_is_published.set(false);
            show_seo_section.set(false);
            #[cfg(client)]
            {
                let show_editor = show_editor.clone();
                spawn_local(async move {
                    let _ = overwrite_editor_document(Delta::new().insert("\n")).await;
                    show_editor.set(true);
                });
            }
        }
    };

    let open_edit_post = {
        let show_editor = show_editor.clone();
        let editing_post = editing_post.clone();
        let editor_title = editor_title.clone();
        let editor_summary = editor_summary.clone();
        let editor_content = editor_content.clone();
        let editor_meta_title = editor_meta_title.clone();
        let editor_meta_description = editor_meta_description.clone();
        let editor_meta_keywords = editor_meta_keywords.clone();
        let editor_og_image = editor_og_image.clone();
        let editor_is_published = editor_is_published.clone();
        let show_seo_section = show_seo_section.clone();
        let show_login_prompt = show_login_prompt.clone();

        move |post: Post| {
            // Check authentication first
            #[cfg(client)]
            {
                if !is_authenticated_fn() {
                    show_login_prompt.set(true);
                    return;
                }
            }
            editor_title.set(post.title.clone());
            editor_summary.set(post.summary.clone());
            editor_content.set(post.content.clone());
            editor_meta_title.set(post.meta_title.clone().unwrap_or_default());
            editor_meta_description.set(post.meta_description.clone().unwrap_or_default());
            editor_meta_keywords.set(post.meta_keywords.clone().unwrap_or_default());
            editor_og_image.set(post.og_image.clone().unwrap_or_default());
            editor_is_published.set(post.is_published);
            show_seo_section.set(false);
            editing_post.set(Some(post));
            #[cfg(client)]
            {
                let show_editor = show_editor.clone();
                let delta = editing_post
                    .get_clone()
                    .and_then(|p| p.content_delta)
                    .and_then(|d| serde_json::from_value::<Delta>(d).ok())
                    .unwrap_or_else(|| Delta::new().insert("\n"));
                spawn_local(async move {
                    let _ = overwrite_editor_document(delta).await;
                    show_editor.set(true);
                });
            }
        }
    };

    let close_editor = {
        let show_editor = show_editor.clone();
        move |_| {
            show_editor.set(false);
        }
    };

    let toggle_seo_section = {
        let show_seo_section = show_seo_section.clone();
        move |_| {
            show_seo_section.set(!show_seo_section.get());
        }
    };

    let save_post = {
        let posts = posts.clone();
        let editing_post = editing_post.clone();
        let editor_title = editor_title.clone();
        let editor_summary = editor_summary.clone();
        let editor_meta_title = editor_meta_title.clone();
        let editor_meta_description = editor_meta_description.clone();
        let editor_meta_keywords = editor_meta_keywords.clone();
        let editor_og_image = editor_og_image.clone();
        let editor_is_published = editor_is_published.clone();
        let show_editor = show_editor.clone();
        let saving = saving.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();

        move |_| {
            #[cfg(client)]
            {
                let posts = posts.clone();
                let editing_post = editing_post.clone();
                let editor_title = editor_title.clone();
                let editor_meta_title = editor_meta_title.clone();
                let editor_meta_description = editor_meta_description.clone();
                let editor_meta_keywords = editor_meta_keywords.clone();
                let editor_og_image = editor_og_image.clone();
                let editor_is_published = editor_is_published.clone();
                let show_editor = show_editor.clone();
                let saving = saving.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    saving.set(true);
                    error_message.set(String::new());
                    success_message.set(String::new());

                    // sycawysgy keeps its internal `EditorState` in a child context (provided by
                    // the `Editor` component). Since this template is the parent, we can't read
                    // that context here. Instead, we read the HTML directly from the editor DOM,
                    // and attempt to load the latest Delta from IndexedDB (best-effort).
                    let delta = load_editor_delta().await.unwrap_or_else(|_| Delta::new().insert("\n"));
                    let content_delta: serde_json::Value =
                        serde_json::to_value(&delta).unwrap_or_default();
                    let content_html = get_editor_html("#posts-editor");
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

                    let result = if let Some(post) = editing_post.get_clone() {
                        update_post(
                            post.id,
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
                        .await
                    } else {
                        create_post(
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
                        .await
                    };

                    match result {
                        Ok(response) => {
                            if response.success {
                                // Close editor and reset saving state first
                                saving.set(false);
                                show_editor.set(false);
                                success_message.set(response.message);

                                // Refresh posts list after modal is closed
                                if let Ok(posts_response) = fetch_posts().await {
                                    if let Some(data) = posts_response.data {
                                        posts.set(data);
                                    }
                                }
                            } else {
                                saving.set(false);
                                error_message.set(response.message);
                            }
                        }
                        Err(e) => {
                            saving.set(false);
                            error_message.set(format!("Failed to save post: {}", e));
                        }
                    }
                });
            }
        }
    };

    let delete_post_handler = {
        let posts = posts.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();

        move |post_id: i32| {
            #[cfg(client)]
            {
                let posts = posts.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    match delete_post(post_id).await {
                        Ok(response) => {
                            if response.success {
                                success_message.set("Post deleted successfully".to_string());
                                // Remove from list
                                let current = posts.get_clone();
                                posts
                                    .set(current.into_iter().filter(|p| p.id != post_id).collect());
                            } else {
                                error_message.set(response.message);
                            }
                        }
                        Err(e) => {
                            error_message.set(format!("Failed to delete post: {}", e));
                        }
                    }
                });
            }
        }
    };

    view! {
        div(class = "container", style = "max-width: 900px;") {
            div(class = "card") {
                div(style = "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;") {
                    h1(style = "margin: 0;") { "Posts" }
                    (if is_authenticated.get() {
                        let open_new = open_new_post.clone();
                        view! {
                            button(on:click = open_new, style = "width: auto; padding: 0.75rem 1.5rem;") {
                                "+ New Post"
                            }
                        }
                    } else {
                        view! {
                            Link(to = "/login", style = "display: inline-flex; align-items: center; padding: 0.75rem 1.5rem; background: #667eea; color: white; border-radius: 8px; text-decoration: none; font-weight: 500;") {
                                "Login to Create Posts"
                            }
                        }
                    })
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
                        p(style = "text-align: center; color: #666;") { "Loading posts..." }
                    }
                } else {
                    view! {}
                })

                // Posts list
                (if !loading.get() && posts.get_clone().is_empty() {
                    view! {
                        div(style = "text-align: center; padding: 2rem; color: #666;") {
                            p { "No posts yet. Create your first post!" }
                        }
                    }
                } else {
                    view! {}
                })

                div(style = "display: flex; flex-direction: column; gap: 1rem;") {
                    Indexed(
                        list = posts,
                        view = {
                            let open_edit_post = open_edit_post.clone();
                            let delete_post_handler = delete_post_handler.clone();
                            move |post| {
                                let post_for_edit = post.clone();
                                let post_id = post.id;
                                let open_edit = open_edit_post.clone();
                                let delete_handler = delete_post_handler.clone();
                                let is_published = post.is_published;
                                let slug = post.slug.clone();

                                view! {
                                    div(style = "border: 1px solid #e0e0e0; border-radius: 8px; padding: 1rem;") {
                                        div(style = "display: flex; justify-content: space-between; align-items: start;") {
                                            div {
                                                div(style = "display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.5rem;") {
                                                    h3(style = "margin: 0;") { (post.title.clone()) }
                                                    (if is_published {
                                                        view! {
                                                            span(style = "display: inline-block; padding: 0.125rem 0.5rem; background: #28a745; color: white; border-radius: 1rem; font-size: 0.7rem;") {
                                                                "Published"
                                                            }
                                                        }
                                                    } else {
                                                        view! {
                                                            span(style = "display: inline-block; padding: 0.125rem 0.5rem; background: #ffc107; color: #333; border-radius: 1rem; font-size: 0.7rem;") {
                                                                "Draft"
                                                            }
                                                        }
                                                    })
                                                }
                                                p(style = "color: #666; margin: 0; font-size: 0.9rem;") { (post.summary.clone()) }
                                                (if let Some(created) = &post.created_at {
                                                    let date = created.clone();
                                                    view! {
                                                        p(style = "color: #999; margin: 0.5rem 0 0 0; font-size: 0.8rem;") {
                                                            "Created: " (date)
                                                        }
                                                    }
                                                } else {
                                                    view! {}
                                                })
                                            }
                                            (if is_authenticated.get() {
                                                let post_for_edit_clone = post_for_edit.clone();
                                                let post_id_clone = post_id;
                                                let open_edit_clone = open_edit.clone();
                                                let delete_handler_clone = delete_handler.clone();
                                                let slug_for_view = slug.clone();
                                                view! {
                                                    div(style = "display: flex; gap: 0.5rem;") {
                                                        (if is_published {
                                                            view! {
                                                                a(
                                                                    href = format!("/post/{}", slug_for_view),
                                                                    target = "_blank",
                                                                    style = "display: inline-flex; align-items: center; padding: 0.5rem 1rem; font-size: 0.875rem; background: #28a745; color: white; border-radius: 8px; text-decoration: none;"
                                                                ) { "View" }
                                                            }
                                                        } else {
                                                            view! {}
                                                        })
                                                        button(
                                                            on:click = move |_| {
                                                                open_edit_clone(post_for_edit_clone.clone());
                                                            },
                                                            style = "width: auto; padding: 0.5rem 1rem; font-size: 0.875rem; background: #667eea;"
                                                        ) { "Edit" }
                                                        button(
                                                            on:click = move |_| {
                                                                delete_handler_clone(post_id_clone);
                                                            },
                                                            style = "width: auto; padding: 0.5rem 1rem; font-size: 0.875rem; background: #dc3545;"
                                                        ) { "Delete" }
                                                    }
                                                }
                                            } else {
                                                let slug_for_view = slug.clone();
                                                view! {
                                                    (if is_published {
                                                        view! {
                                                            div(style = "display: flex; gap: 0.5rem;") {
                                                                a(
                                                                    href = format!("/post/{}", slug_for_view),
                                                                    target = "_blank",
                                                                    style = "display: inline-flex; align-items: center; padding: 0.5rem 1rem; font-size: 0.875rem; background: #28a745; color: white; border-radius: 8px; text-decoration: none;"
                                                                ) { "View" }
                                                            }
                                                        }
                                                    } else {
                                                        view! {}
                                                    })
                                                }
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    )
                }
            }

            // Login Prompt Modal
            (if show_login_prompt.get() {
                let close_login_prompt = {
                    let show_login_prompt = show_login_prompt.clone();
                    move |_| {
                        show_login_prompt.set(false);
                    }
                };

                view! {
                    div(class = "link-modal-overlay") {
                        div(class = "card", style = "max-width: 400px; text-align: center;") {
                            h2(style = "margin-bottom: 1rem;") { "Authentication Required" }
                            p(style = "color: #666; margin-bottom: 1.5rem;") {
                                "You need to be logged in to create or edit posts. Please log in to continue."
                            }
                            div(style = "display: flex; gap: 1rem; justify-content: center;") {
                                Link(
                                    to = "/login",
                                    style = "display: inline-flex; align-items: center; padding: 0.75rem 1.5rem; background: #667eea; color: white; border-radius: 8px; text-decoration: none; font-weight: 500;"
                                ) { "Login" }
                                button(
                                    on:click = close_login_prompt,
                                    style = "width: auto; padding: 0.75rem 1.5rem; background: #6c757d;"
                                ) { "Cancel" }
                            }
                        }
                    }
                }
            } else {
                view! {}
            })

            // Editor Modal
            (if show_editor.get() {
                let is_editing = editing_post.get_clone().is_some();
                let title = if is_editing { "Edit Post" } else { "New Post" };
                let close_editor = close_editor.clone();
                let save_post = save_post.clone();
                let toggle_seo = toggle_seo_section.clone();
                let is_saving = saving.get();
                let seo_expanded = show_seo_section.get();
                let editing_post_clone = editing_post.clone();

                view! {
                    div(class = "link-modal-overlay") {
                        div(class = "card", style = "min-width: 700px; max-width: 900px; max-height: 90vh; overflow-y: auto;") {
                            div(style = "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;") {
                                h2(style = "margin: 0;") { (title) }
                                button(
                                    on:click = close_editor,
                                    style = "width: auto; padding: 0.5rem 1rem; background: #6c757d;"
                                ) { "Close" }
                            }

                            div(class = "form-group") {
                                label { "Title" }
                                input(
                                    r#type = "text",
                                    bind:value = editor_title,
                                    placeholder = "Post title..."
                                )
                            }

                            div(class = "form-group") {
                                label { "Summary" }
                                input(
                                    r#type = "text",
                                    bind:value = editor_summary,
                                    placeholder = "Brief summary for previews..."
                                )
                            }

                            div(class = "form-group") {
                                label { "Content" }

                                // sycawysgy Editor (client-side only)
                                ({
                                    #[cfg(client)]
                                    {
                                        view! {
                                            div(id = "posts-editor") {
                                                Editor {}
                                            }
                                        }
                                    }
                                    #[cfg(engine)]
                                    {
                                        view! {
                                            div(style = "padding: 1rem; background: #f5f5f5; border-radius: 8px;") {
                                                "Editor loading..."
                                            }
                                        }
                                    }
                                })
                            }

                            // SEO Section (collapsible)
                            div(style = "border-top: 1px solid #e0e0e0; margin-top: 1rem; padding-top: 1rem;") {
                                button(
                                    on:click = toggle_seo,
                                    style = "width: auto; padding: 0.5rem 1rem; background: transparent; color: #667eea; border: 1px solid #667eea; margin-bottom: 1rem;"
                                ) {
                                    (if seo_expanded { "Hide SEO Settings" } else { "Show SEO Settings" })
                                }

                                (if seo_expanded {
                                    view! {
                                        div {
                                            div(class = "form-group") {
                                                label { "Meta Title (for search engines)" }
                                                input(
                                                    r#type = "text",
                                                    bind:value = editor_meta_title,
                                                    placeholder = "Leave empty to use post title..."
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
                                                    placeholder = "Description for search results..."
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
                                                    placeholder = "keyword1, keyword2, keyword3..."
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
                                                    placeholder = "https://example.com/image.jpg"
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
                            }

                            div(style = "display: flex; gap: 1rem; justify-content: flex-end;") {
                                button(
                                    on:click = save_post,
                                    disabled = is_saving
                                ) {
                                    (if is_saving { "Saving..." } else { "Save Post" })
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
async fn fetch_posts() -> Result<PostsResponse, String> {
    use gloo_net::http::Request;

    let response = Request::get("/api/posts")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<PostsResponse>()
        .await
        .map_err(|e| e.to_string())
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

#[cfg(client)]
async fn update_post(
    id: i32,
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

    // Get session credentials
    let (session_id, session_verifier) = get_session_credentials();

    let response = Request::patch(&format!("/api/posts/{}", id))
        .json(&serde_json::json!({
            "session_id": session_id,
            "session_verifier": session_verifier,
            "title": title,
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

#[cfg(client)]
async fn delete_post(id: i32) -> Result<PostResponse, String> {
    use gloo_net::http::Request;

    // Get session credentials
    let (session_id, session_verifier) = get_session_credentials();

    let response = Request::delete(&format!("/api/posts/{}", id))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "session_id": session_id,
            "session_verifier": session_verifier
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

#[cfg(client)]
async fn login_and_get_session(email: &str, password: &str) -> Result<SessionData, String> {
    use gloo_net::http::Request;

    #[derive(Debug, Serialize, Deserialize)]
    struct LoginRequest {
        email: String,
        password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct LoginResponse {
        success: bool,
        message: String,
        data: Option<SessionData>,
    }

    let response = Request::post("/api/auth/login")
        .json(&LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let login_response: LoginResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    if login_response.success {
        if let Some(session) = login_response.data {
            // Store session in localStorage (same as login.rs)
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("session_id", &session.session_id.to_string());
                    let _ = storage.set_item("session_verifier", &session.session_verifier);
                }
            }
            Ok(session)
        } else {
            Err("No session data returned".to_string())
        }
    } else {
        Err(login_response.message)
    }
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
    Template::build("posts").view(posts_page).build()
}
