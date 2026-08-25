use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(client)]
use sycamore::futures::spawn_local;

#[cfg(client)]
use sycawysgy::{Delta, render_delta_to_html};

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PostViewStateRx")]
struct PostViewState {
    /// The slug portion after `/post/`.
    slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
struct PostData {
    id: i32,
    title: String,
    slug: String,
    summary: String,
    content: String,
    // Delta JSON content from sycawysgy editor
    content_delta: Option<serde_json::Value>,
    meta_title: Option<String>,
    meta_description: Option<String>,
    meta_keywords: Option<String>,
    og_image: Option<String>,
    canonical_url: Option<String>,
    is_published: bool,
    published_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PostApiResponse {
    success: bool,
    message: String,
    data: Option<PostData>,
}

// Enum to track the display state
#[derive(Clone, PartialEq)]
enum ViewState {
    Loading,
    Error(String),
    Loaded(PostData),
    Empty,
}

fn post_view_page(state: PostViewStateRx) -> View {
    let view_state = create_signal(ViewState::Loading);

    // Client-side fetching
    #[cfg(client)]
    {
        let view_state = view_state.clone();
        let slug = state.slug.get_clone();

        spawn_local(async move {
            if slug.is_empty() {
                view_state.set(ViewState::Error("Invalid post URL".to_string()));
                return;
            }

            match fetch_post_by_slug(&slug).await {
                Ok(response) => {
                    if response.success {
                        if let Some(post_data) = response.data {
                            if !post_data.is_published {
                                view_state.set(ViewState::Error("This post is not published yet".to_string()));
                            } else {
                                // Update document head with SEO tags
                                update_document_head(&post_data);
                                view_state.set(ViewState::Loaded(post_data));
                            }
                        } else {
                            view_state.set(ViewState::Error("Post not found".to_string()));
                        }
                    } else {
                        view_state.set(ViewState::Error(response.message));
                    }
                }
                Err(e) => {
                    view_state.set(ViewState::Error(format!("Failed to load post: {}", e)));
                }
            }
        });
    }

    #[cfg(engine)]
    {
        // Keep the initial `Loading` state so the SSR markup matches the initial client view.
    }

    view! {
        div(class = "container", style = "max-width: 800px;") {
            (match view_state.get_clone() {
                ViewState::Loading => view! {
                    div(class = "card") {
                        p(style = "text-align: center; color: #666; padding: 2rem;") {
                            "Loading post..."
                        }
                    }
                },
                ViewState::Error(err_msg) => view! {
                    div(class = "card") {
                        div(style = "text-align: center; padding: 2rem;") {
                            h1(style = "color: #dc3545;") { "Post Not Found" }
                            p(style = "color: #666;") { (err_msg) }
                            div(class = "nav-links", style = "margin-top: 1.5rem;") {
                                Link(to = "/") { "Back to Home" }
                            }
                        }
                    }
                },
                ViewState::Loaded(post_data) => {
                    let title = post_data.title.clone();
                    let published_at = post_data.published_at.clone();

                    // Render content: prefer Delta JSON, fall back to HTML
                    #[cfg(client)]
                    let content_html = {
                        if let Some(delta_json) = &post_data.content_delta {
                            if let Ok(delta) = serde_json::from_value::<Delta>(delta_json.clone()) {
                                render_delta_to_html(&delta)
                            } else {
                                post_data.content.clone()
                            }
                        } else {
                            post_data.content.clone()
                        }
                    };
                    #[cfg(client)]
                    {
                        let content = content_html;
                        view! {
                            article(class = "card") {
                                header(style = "margin-bottom: 2rem;") {
                                    h1(style = "font-size: 2rem; line-height: 1.3; margin-bottom: 0.5rem;") {
                                        (title)
                                    }
                                    (if let Some(date) = published_at {
                                        view! {
                                            p(style = "color: #666; font-size: 0.9rem;") {
                                                "Published: " (date)
                                            }
                                        }
                                    } else {
                                        view! {}
                                    })
                                }

                                // Post content (rendered as HTML from Delta or fallback to content)
                                div(
                                    class = "post-content",
                                    style = "line-height: 1.8; font-size: 1.1rem;",
                                    dangerously_set_inner_html = content
                                )

                                footer(style = "margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid #e0e0e0;") {
                                    div(class = "nav-links") {
                                        Link(to = "/") { "Back to Home" }
                                    }
                                }
                            }
                        }
                    }
                    #[cfg(engine)]
                    {
                        // On engine side, always render from content (HTML fallback)
                        let content = post_data.content.clone();
                        view! {
                            article(class = "card") {
                                header(style = "margin-bottom: 2rem;") {
                                    h1(style = "font-size: 2rem; line-height: 1.3; margin-bottom: 0.5rem;") {
                                        (title)
                                    }
                                    (if let Some(date) = published_at {
                                        view! {
                                            p(style = "color: #666; font-size: 0.9rem;") {
                                                "Published: " (date)
                                            }
                                        }
                                    } else {
                                        view! {}
                                    })
                                }

                                // Post content (rendered as HTML)
                                div(
                                    class = "post-content",
                                    style = "line-height: 1.8; font-size: 1.1rem;",
                                    dangerously_set_inner_html = content
                                )

                                footer(style = "margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid #e0e0e0;") {
                                    div(class = "nav-links") {
                                        Link(to = "/") { "Back to Home" }
                                    }
                                }
                            }
                        }
                    }
                },
                ViewState::Empty => view! {
                    div(class = "card") {
                        p(style = "text-align: center; color: #666; padding: 2rem;") {
                            "Loading..."
                        }
                    }
                },
            })
        }
    }
}

#[cfg(client)]
fn update_document_head(post: &PostData) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Update title
            let title = post.meta_title.as_ref()
                .unwrap_or(&post.title);
            let _ = document.set_title(&format!("{} | Perseus Blog", title));

            // Update meta description
            if let Some(desc) = &post.meta_description {
                update_or_create_meta(&document, "name", "description", desc);
            } else if !post.summary.is_empty() {
                update_or_create_meta(&document, "name", "description", &post.summary);
            }

            // Update meta keywords
            if let Some(keywords) = &post.meta_keywords {
                update_or_create_meta(&document, "name", "keywords", keywords);
            }

            // Update Open Graph tags
            update_or_create_meta(&document, "property", "og:type", "article");
            update_or_create_meta(&document, "property", "og:title", &post.title);

            if let Some(desc) = &post.meta_description {
                update_or_create_meta(&document, "property", "og:description", desc);
            } else if !post.summary.is_empty() {
                update_or_create_meta(&document, "property", "og:description", &post.summary);
            }

            if let Some(img) = &post.og_image {
                update_or_create_meta(&document, "property", "og:image", img);
            }

            // Twitter Card
            update_or_create_meta(&document, "name", "twitter:card", "summary_large_image");
            update_or_create_meta(&document, "name", "twitter:title", &post.title);

            if let Some(desc) = &post.meta_description {
                update_or_create_meta(&document, "name", "twitter:description", desc);
            }
        }
    }
}

#[cfg(client)]
fn update_or_create_meta(document: &web_sys::Document, attr_type: &str, attr_name: &str, content: &str) {
    let selector = format!("meta[{}=\"{}\"]", attr_type, attr_name);

    // Try to find existing meta tag
    if let Ok(Some(element)) = document.query_selector(&selector) {
        let _ = element.set_attribute("content", content);
        return;
    }

    // Create new meta tag
    if let Ok(meta) = document.create_element("meta") {
        let _ = meta.set_attribute(attr_type, attr_name);
        let _ = meta.set_attribute("content", content);
        if let Some(head) = document.head() {
            let _ = head.append_child(&meta);
        }
    }
}

#[cfg(client)]
async fn fetch_post_by_slug(slug: &str) -> Result<PostApiResponse, String> {
    use gloo_net::http::Request;

    let response = Request::get(&format!("/api/posts/slug/{}", slug))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json::<PostApiResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        // Build the template root once so it exists, and rely on incremental generation for
        // `/post/<slug>`.
        paths: vec![String::new()],
        extra: ().into(),
    }
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> Result<PostViewState, BlamedError<anyhow::Error>> {
    Ok(PostViewState { slug: path })
}

pub fn get_template() -> Template {
    Template::build("post")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .incremental_generation()
        .view_with_state(post_view_page)
        .build()
}
