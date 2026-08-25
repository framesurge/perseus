use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use sycamore::prelude::*;

#[cfg(client)]
use crate::components::WysiwygEditor;

#[cfg(client)]
use sycamore::futures::spawn_local;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
struct Post {
    id: i32,
    title: String,
    slug: String,
    summary: String,
    content: String,
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

        move |_| {
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
            show_editor.set(true);
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

        move |post: Post| {
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
            show_editor.set(true);
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

    // Store content in a RefCell to avoid re-renders during typing
    let content_ref = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let handle_content_change: Rc<dyn Fn(String) + 'static> = {
        let content_ref = content_ref.clone();
        let editor_content = editor_content.clone();
        Rc::new(move |content: String| {
            // Store in ref (doesn't trigger re-render)
            *content_ref.borrow_mut() = content.clone();
            // Only update signal when content is non-empty to sync for save
            if !content.is_empty() {
                editor_content.set(content);
            }
        })
    };

    let save_post = {
        let posts = posts.clone();
        let editing_post = editing_post.clone();
        let editor_title = editor_title.clone();
        let editor_summary = editor_summary.clone();
        let editor_content = editor_content.clone();
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
                let editor_summary = editor_summary.clone();
                let editor_content = editor_content.clone();
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

                    let title = editor_title.get_clone();
                    let summary = editor_summary.get_clone();
                    let content = editor_content.get_clone();
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
                    button(on:click = open_new_post, style = "width: auto; padding: 0.75rem 1.5rem;") {
                        "+ New Post"
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
                                            div(style = "display: flex; gap: 0.5rem;") {
                                                (if is_published {
                                                    let slug = post.slug.clone();
                                                    view! {
                                                        a(
                                                            href = format!("/post/{}", slug),
                                                            target = "_blank",
                                                            style = "display: inline-flex; align-items: center; padding: 0.5rem 1rem; font-size: 0.875rem; background: #28a745; color: white; border-radius: 8px; text-decoration: none;"
                                                        ) { "View" }
                                                    }
                                                } else {
                                                    view! {}
                                                })
                                                button(
                                                    on:click = move |_| {
                                                        open_edit(post_for_edit.clone());
                                                    },
                                                    style = "width: auto; padding: 0.5rem 1rem; font-size: 0.875rem; background: #667eea;"
                                                ) { "Edit" }
                                                button(
                                                    on:click = move |_| {
                                                        delete_handler(post_id);
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
                let is_editing = editing_post.get_clone().is_some();
                let title = if is_editing { "Edit Post" } else { "New Post" };
                let editor_content_val = editor_content.get_clone();
                let on_content_change = handle_content_change.clone();
                let close_editor = close_editor.clone();
                let save_post = save_post.clone();
                let toggle_seo = toggle_seo_section.clone();
                let is_saving = saving.get();
                let seo_expanded = show_seo_section.get();

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

                                // WYSIWYG Editor (client-side only)
                                ({
                                    #[cfg(client)]
                                    {
                                        view! {
                                            WysiwygEditor(
                                                initial_content = editor_content_val,
                                                placeholder = "Write your post content here...".to_string(),
                                                on_change = on_content_change,
                                                min_height = 300,
                                                enable_images = true,
                                                upload_url = "/api/uploads".to_string()
                                            )
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
                a(href = "/") { "Back to Home" }
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

    let response = Request::post("/api/posts")
        .json(&serde_json::json!({
            "title": title,
            "slug": slug,
            "summary": summary,
            "content": content,
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
    meta_title: &str,
    meta_description: &str,
    meta_keywords: &str,
    og_image: &str,
    is_published: bool,
) -> Result<PostResponse, String> {
    use gloo_net::http::Request;

    let response = Request::patch(&format!("/api/posts/{}", id))
        .json(&serde_json::json!({
            "title": title,
            "summary": summary,
            "content": content,
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

    let response = Request::delete(&format!("/api/posts/{}", id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<PostResponse>()
        .await
        .map_err(|e| e.to_string())
}

pub fn get_template() -> Template {
    Template::build("posts").view(posts_page).build()
}
