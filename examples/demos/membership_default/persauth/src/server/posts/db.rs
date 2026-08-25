use deadpool_postgres::Client;

use crate::server::errors::ServiceError;

use super::models::*;

/// Helper to build a Post from a row
fn post_from_row(row: &tokio_postgres::Row) -> Post {
    Post {
        id: row.get("id"),
        title: row.get("title"),
        slug: row.get("slug"),
        summary: row.get("summary"),
        content: row.get("content"),
        content_delta: row.get("content_delta"),
        meta_title: row.get("meta_title"),
        meta_description: row.get("meta_description"),
        meta_keywords: row.get("meta_keywords"),
        og_image: row.get("og_image"),
        canonical_url: row.get("canonical_url"),
        is_published: row.get("is_published"),
        published_at: row.get("published_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

const POST_COLUMNS: &str = "id, title, slug, summary, content, content_delta, meta_title, meta_description, meta_keywords, og_image, canonical_url, is_published, published_at, created_at, updated_at";

/// Add a new post to the database
pub async fn add_post(client: &Client, post: &CreatePost) -> Result<Post, ServiceError> {
    let statement = client
        .prepare(&format!(
            "INSERT INTO posts (title, slug, summary, content, content_delta, meta_title, meta_description, meta_keywords, og_image, canonical_url, is_published, published_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, CASE WHEN $11 THEN NOW() ELSE NULL END)
             RETURNING {}", POST_COLUMNS
        ))
        .await?;

    let row = client
        .query_one(
            &statement,
            &[
                &post.title,
                &post.slug,
                &post.summary,
                &post.content,
                &post.content_delta,
                &post.meta_title,
                &post.meta_description,
                &post.meta_keywords,
                &post.og_image,
                &post.canonical_url,
                &post.is_published,
            ],
        )
        .await?;

    Ok(post_from_row(&row))
}

/// Get all posts with optional pagination
pub async fn list_posts(
    client: &Client,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Post>, ServiceError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let statement = client
        .prepare(&format!(
            "SELECT {}
             FROM posts
             ORDER BY created_at DESC
             LIMIT $1 OFFSET $2", POST_COLUMNS
        ))
        .await?;

    let rows = client.query(&statement, &[&limit, &offset]).await?;

    Ok(rows.iter().map(post_from_row).collect())
}

/// Get published posts only (for public view)
pub async fn list_published_posts(
    client: &Client,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Post>, ServiceError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let statement = client
        .prepare(&format!(
            "SELECT {}
             FROM posts
             WHERE is_published = true
             ORDER BY published_at DESC
             LIMIT $1 OFFSET $2", POST_COLUMNS
        ))
        .await?;

    let rows = client.query(&statement, &[&limit, &offset]).await?;

    Ok(rows.iter().map(post_from_row).collect())
}

/// Get a single post by ID
pub async fn get_post_by_id(client: &Client, id: i32) -> Result<Post, ServiceError> {
    let statement = client
        .prepare(&format!(
            "SELECT {}
             FROM posts WHERE id = $1", POST_COLUMNS
        ))
        .await?;

    let row = client
        .query_opt(&statement, &[&id])
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Post with id {} not found", id)))?;

    Ok(post_from_row(&row))
}

/// Get a single post by slug
pub async fn get_post_by_slug(client: &Client, slug: &str) -> Result<Post, ServiceError> {
    let statement = client
        .prepare(&format!(
            "SELECT {}
             FROM posts WHERE slug = $1", POST_COLUMNS
        ))
        .await?;

    let row = client
        .query_opt(&statement, &[&slug])
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Post with slug '{}' not found", slug)))?;

    Ok(post_from_row(&row))
}

/// Update an existing post
pub async fn update_post(
    client: &Client,
    id: i32,
    update: &UpdatePost,
) -> Result<Post, ServiceError> {
    // First get the existing post
    let existing = get_post_by_id(client, id).await?;

    let title = update.title.as_ref().unwrap_or(&existing.title);
    let slug = update.slug.as_ref().unwrap_or(&existing.slug);
    let summary = update.summary.as_ref().unwrap_or(&existing.summary);
    let content = update.content.as_ref().unwrap_or(&existing.content);
    // Get content_delta: use update's if present, else keep existing
    let content_delta = update.content_delta.as_ref().or(existing.content_delta.as_ref()).cloned();
    let meta_title = update.meta_title.as_ref().or(existing.meta_title.as_ref());
    let meta_description = update.meta_description.as_ref().or(existing.meta_description.as_ref());
    let meta_keywords = update.meta_keywords.as_ref().or(existing.meta_keywords.as_ref());
    let og_image = update.og_image.as_ref().or(existing.og_image.as_ref());
    let canonical_url = update.canonical_url.as_ref().or(existing.canonical_url.as_ref());
    let is_published = update.is_published.unwrap_or(existing.is_published);

    // Set published_at when first published
    let published_at = if is_published && existing.published_at.is_none() {
        Some(chrono::Utc::now().naive_utc())
    } else {
        existing.published_at
    };

    let statement = client
        .prepare(&format!(
            "UPDATE posts
             SET title = $1, slug = $2, summary = $3, content = $4, content_delta = $5,
                 meta_title = $6, meta_description = $7, meta_keywords = $8,
                 og_image = $9, canonical_url = $10, is_published = $11,
                 published_at = $12, updated_at = NOW()
             WHERE id = $13
             RETURNING {}", POST_COLUMNS
        ))
        .await?;

    let row = client
        .query_one(&statement, &[
            title,
            slug,
            summary,
            content,
            &content_delta,
            &meta_title,
            &meta_description,
            &meta_keywords,
            &og_image,
            &canonical_url,
            &is_published,
            &published_at,
            &id,
        ])
        .await?;

    Ok(post_from_row(&row))
}

/// Delete a post by ID
pub async fn delete_post(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client.prepare("DELETE FROM posts WHERE id = $1").await?;

    let result = client.execute(&statement, &[&id]).await?;

    if result == 0 {
        return Err(ServiceError::NotFound(format!(
            "Post with id {} not found",
            id
        )));
    }

    Ok(())
}

/// Search posts by title or content
pub async fn search_posts(
    client: &Client,
    search: &str,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Post>, ServiceError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    let search_pattern = format!("%{}%", search);

    let statement = client
        .prepare(&format!(
            "SELECT {}
             FROM posts
             WHERE title ILIKE $1 OR content ILIKE $1 OR summary ILIKE $1 OR content_delta::text ILIKE $1
             ORDER BY created_at DESC
             LIMIT $2 OFFSET $3", POST_COLUMNS
        ))
        .await?;

    let rows = client
        .query(&statement, &[&search_pattern, &limit, &offset])
        .await?;

    Ok(rows.iter().map(post_from_row).collect())
}

/// Check if a slug already exists
pub async fn slug_exists(client: &Client, slug: &str) -> Result<bool, ServiceError> {
    let statement = client
        .prepare("SELECT 1 FROM posts WHERE slug = $1")
        .await?;

    let row = client.query_opt(&statement, &[&slug]).await?;
    Ok(row.is_some())
}
