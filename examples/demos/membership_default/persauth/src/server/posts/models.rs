use serde::{Deserialize, Serialize};

/// Post entity stored in the database
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    // Delta JSON content from sycawysgy editor (Quill Delta format)
    #[serde(default)]
    pub content_delta: Option<serde_json::Value>,
    // SEO fields
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_image: Option<String>,
    pub canonical_url: Option<String>,
    // Publishing
    pub is_published: bool,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Request to create a new post
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePost {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    // Delta JSON content from sycawysgy editor (Quill Delta format)
    #[serde(default)]
    pub content_delta: Option<serde_json::Value>,
    // SEO fields (optional on create)
    #[serde(default)]
    pub meta_title: Option<String>,
    #[serde(default)]
    pub meta_description: Option<String>,
    #[serde(default)]
    pub meta_keywords: Option<String>,
    #[serde(default)]
    pub og_image: Option<String>,
    #[serde(default)]
    pub canonical_url: Option<String>,
    #[serde(default)]
    pub is_published: bool,
}

/// Request to update an existing post
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    // Delta JSON content from sycawysgy editor (Quill Delta format)
    pub content_delta: Option<serde_json::Value>,
    // SEO fields
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_image: Option<String>,
    pub canonical_url: Option<String>,
    pub is_published: Option<bool>,
}

/// Post response for API (public fields only)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    // Delta JSON content from sycawysgy editor (Quill Delta format)
    pub content_delta: Option<serde_json::Value>,
    // SEO fields
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_image: Option<String>,
    pub canonical_url: Option<String>,
    // Publishing
    pub is_published: bool,
    pub published_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            slug: post.slug,
            summary: post.summary,
            content: post.content,
            content_delta: post.content_delta,
            meta_title: post.meta_title,
            meta_description: post.meta_description,
            meta_keywords: post.meta_keywords,
            og_image: post.og_image,
            canonical_url: post.canonical_url,
            is_published: post.is_published,
            published_at: post.published_at.map(|dt| dt.to_string()),
            created_at: post.created_at.map(|dt| dt.to_string()),
            updated_at: post.updated_at.map(|dt| dt.to_string()),
        }
    }
}

/// Search/filter parameters for posts
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PostQuery {
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub published_only: Option<bool>,
}

/// Authenticated create post request (includes session)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePostRequest {
    #[serde(default)]
    pub session_id: i32,
    #[serde(default)]
    pub session_verifier: String,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    #[serde(default)]
    pub content_delta: Option<serde_json::Value>,
    #[serde(default)]
    pub meta_title: Option<String>,
    #[serde(default)]
    pub meta_description: Option<String>,
    #[serde(default)]
    pub meta_keywords: Option<String>,
    #[serde(default)]
    pub og_image: Option<String>,
    #[serde(default)]
    pub canonical_url: Option<String>,
    #[serde(default)]
    pub is_published: bool,
}

/// Authenticated update post request (includes session)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePostRequest {
    #[serde(default)]
    pub session_id: i32,
    #[serde(default)]
    pub session_verifier: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub content_delta: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_image: Option<String>,
    pub canonical_url: Option<String>,
    pub is_published: Option<bool>,
}

/// Authenticated delete post request (includes session)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeletePostRequest {
    #[serde(default)]
    pub session_id: i32,
    #[serde(default)]
    pub session_verifier: String,
}
