use deadpool_postgres::Client;

use crate::server::errors::ServiceError;

use super::models::*;

/// Add a new tag to the database
pub async fn add_tag(client: &Client, tag: &CreateTag) -> Result<Tag, ServiceError> {
    let statement = client
        .prepare(
            "INSERT INTO tags (name, slug)
             VALUES ($1, $2)
             RETURNING id, name, slug, created_at, updated_at",
        )
        .await?;

    let row = client
        .query_one(&statement, &[&tag.name, &tag.slug])
        .await?;

    Ok(Tag {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Get all tags with optional pagination
pub async fn list_tags(
    client: &Client,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Tag>, ServiceError> {
    let limit = limit.unwrap_or(100);
    let offset = offset.unwrap_or(0);

    let statement = client
        .prepare(
            "SELECT id, name, slug, created_at, updated_at
             FROM tags
             ORDER BY name ASC
             LIMIT $1 OFFSET $2",
        )
        .await?;

    let rows = client.query(&statement, &[&limit, &offset]).await?;

    let tags = rows
        .iter()
        .map(|row| Tag {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(tags)
}

/// Get a single tag by ID
pub async fn get_tag_by_id(client: &Client, id: i32) -> Result<Tag, ServiceError> {
    let statement = client
        .prepare(
            "SELECT id, name, slug, created_at, updated_at
             FROM tags WHERE id = $1",
        )
        .await?;

    let row = client
        .query_opt(&statement, &[&id])
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Tag with id {} not found", id)))?;

    Ok(Tag {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Get a single tag by slug
pub async fn get_tag_by_slug(client: &Client, slug: &str) -> Result<Tag, ServiceError> {
    let statement = client
        .prepare(
            "SELECT id, name, slug, created_at, updated_at
             FROM tags WHERE slug = $1",
        )
        .await?;

    let row = client
        .query_opt(&statement, &[&slug])
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Tag with slug '{}' not found", slug)))?;

    Ok(Tag {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Update an existing tag
pub async fn update_tag(client: &Client, id: i32, update: &UpdateTag) -> Result<Tag, ServiceError> {
    // First get the existing tag
    let existing = get_tag_by_id(client, id).await?;

    let name = update.name.as_ref().unwrap_or(&existing.name);
    let slug = update.slug.as_ref().unwrap_or(&existing.slug);

    let statement = client
        .prepare(
            "UPDATE tags
             SET name = $1, slug = $2, updated_at = NOW()
             WHERE id = $3
             RETURNING id, name, slug, created_at, updated_at",
        )
        .await?;

    let row = client.query_one(&statement, &[name, slug, &id]).await?;

    Ok(Tag {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Delete a tag by ID
pub async fn delete_tag(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client.prepare("DELETE FROM tags WHERE id = $1").await?;

    let result = client.execute(&statement, &[&id]).await?;

    if result == 0 {
        return Err(ServiceError::NotFound(format!(
            "Tag with id {} not found",
            id
        )));
    }

    Ok(())
}

/// Search tags by name
pub async fn search_tags(
    client: &Client,
    search: &str,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Tag>, ServiceError> {
    let limit = limit.unwrap_or(100);
    let offset = offset.unwrap_or(0);
    let search_pattern = format!("%{}%", search);

    let statement = client
        .prepare(
            "SELECT id, name, slug, created_at, updated_at
             FROM tags
             WHERE name ILIKE $1
             ORDER BY name ASC
             LIMIT $2 OFFSET $3",
        )
        .await?;

    let rows = client
        .query(&statement, &[&search_pattern, &limit, &offset])
        .await?;

    let tags = rows
        .iter()
        .map(|row| Tag {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(tags)
}

/// Check if a slug already exists
pub async fn slug_exists(client: &Client, slug: &str) -> Result<bool, ServiceError> {
    let statement = client
        .prepare("SELECT 1 FROM tags WHERE slug = $1")
        .await?;

    let row = client.query_opt(&statement, &[&slug]).await?;
    Ok(row.is_some())
}

/// Get all tags for a specific post
pub async fn get_tags_for_post(client: &Client, post_id: i32) -> Result<Vec<Tag>, ServiceError> {
    let statement = client
        .prepare(
            "SELECT t.id, t.name, t.slug, t.created_at, t.updated_at
             FROM tags t
             INNER JOIN posts_tags pt ON t.id = pt.tag_id
             WHERE pt.post_id = $1
             ORDER BY t.name ASC",
        )
        .await?;

    let rows = client.query(&statement, &[&post_id]).await?;

    let tags = rows
        .iter()
        .map(|row| Tag {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(tags)
}

/// Add tags to a post
pub async fn add_tags_to_post(
    client: &Client,
    post_id: i32,
    tag_ids: &[i32],
) -> Result<(), ServiceError> {
    // First remove all existing tags for this post
    let delete_stmt = client
        .prepare("DELETE FROM posts_tags WHERE post_id = $1")
        .await?;
    client.execute(&delete_stmt, &[&post_id]).await?;

    // Add new tags
    let insert_stmt = client
        .prepare("INSERT INTO posts_tags (post_id, tag_id) VALUES ($1, $2)")
        .await?;

    for tag_id in tag_ids {
        client.execute(&insert_stmt, &[&post_id, tag_id]).await?;
    }

    Ok(())
}

/// Remove a specific tag from a post
pub async fn remove_tag_from_post(
    client: &Client,
    post_id: i32,
    tag_id: i32,
) -> Result<(), ServiceError> {
    let statement = client
        .prepare("DELETE FROM posts_tags WHERE post_id = $1 AND tag_id = $2")
        .await?;

    client.execute(&statement, &[&post_id, &tag_id]).await?;

    Ok(())
}
