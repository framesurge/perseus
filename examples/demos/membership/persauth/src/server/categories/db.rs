use deadpool_postgres::Client;

use crate::server::errors::ServiceError;

use super::models::*;

/// Add a new category to the database
pub async fn add_category(client: &Client, category: &CreateCategory) -> Result<Category, ServiceError> {
    let statement = client
        .prepare(
            "INSERT INTO categories (name, slug, description)
             VALUES ($1, $2, $3)
             RETURNING id, name, slug, description, created_at, updated_at",
        )
        .await?;

    let row = client
        .query_one(
            &statement,
            &[&category.name, &category.slug, &category.description],
        )
        .await?;

    Ok(Category {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Get all categories with optional pagination
pub async fn list_categories(
    client: &Client,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Category>, ServiceError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let statement = client
        .prepare(
            "SELECT id, name, slug, description, created_at, updated_at
             FROM categories
             ORDER BY name ASC
             LIMIT $1 OFFSET $2",
        )
        .await?;

    let rows = client.query(&statement, &[&limit, &offset]).await?;

    let categories = rows
        .iter()
        .map(|row| Category {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(categories)
}

/// Get a single category by ID
pub async fn get_category_by_id(client: &Client, id: i32) -> Result<Category, ServiceError> {
    let statement = client
        .prepare(
            "SELECT id, name, slug, description, created_at, updated_at
             FROM categories WHERE id = $1",
        )
        .await?;

    let row = client
        .query_opt(&statement, &[&id])
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Category with id {} not found", id)))?;

    Ok(Category {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Get a single category by slug
pub async fn get_category_by_slug(client: &Client, slug: &str) -> Result<Category, ServiceError> {
    let statement = client
        .prepare(
            "SELECT id, name, slug, description, created_at, updated_at
             FROM categories WHERE slug = $1",
        )
        .await?;

    let row = client
        .query_opt(&statement, &[&slug])
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Category with slug '{}' not found", slug)))?;

    Ok(Category {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Update an existing category
pub async fn update_category(
    client: &Client,
    id: i32,
    update: &UpdateCategory,
) -> Result<Category, ServiceError> {
    // First get the existing category
    let existing = get_category_by_id(client, id).await?;

    let name = update.name.as_ref().unwrap_or(&existing.name);
    let slug = update.slug.as_ref().unwrap_or(&existing.slug);
    let description = update.description.as_ref().unwrap_or(&existing.description);

    let statement = client
        .prepare(
            "UPDATE categories
             SET name = $1, slug = $2, description = $3, updated_at = NOW()
             WHERE id = $4
             RETURNING id, name, slug, description, created_at, updated_at",
        )
        .await?;

    let row = client
        .query_one(&statement, &[name, slug, description, &id])
        .await?;

    Ok(Category {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Delete a category by ID
pub async fn delete_category(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client
        .prepare("DELETE FROM categories WHERE id = $1")
        .await?;

    let result = client.execute(&statement, &[&id]).await?;

    if result == 0 {
        return Err(ServiceError::NotFound(format!(
            "Category with id {} not found",
            id
        )));
    }

    Ok(())
}

/// Search categories by name or description
pub async fn search_categories(
    client: &Client,
    search: &str,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Category>, ServiceError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    let search_pattern = format!("%{}%", search);

    let statement = client
        .prepare(
            "SELECT id, name, slug, description, created_at, updated_at
             FROM categories
             WHERE name ILIKE $1 OR description ILIKE $1
             ORDER BY name ASC
             LIMIT $2 OFFSET $3",
        )
        .await?;

    let rows = client
        .query(&statement, &[&search_pattern, &limit, &offset])
        .await?;

    let categories = rows
        .iter()
        .map(|row| Category {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(categories)
}

/// Check if a slug already exists
pub async fn slug_exists(client: &Client, slug: &str) -> Result<bool, ServiceError> {
    let statement = client
        .prepare("SELECT 1 FROM categories WHERE slug = $1")
        .await?;

    let row = client.query_opt(&statement, &[&slug]).await?;
    Ok(row.is_some())
}
