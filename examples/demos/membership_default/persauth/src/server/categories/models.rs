use serde::{Deserialize, Serialize};

/// Category entity stored in the database
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Request to create a new category
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCategory {
    pub name: String,
    pub slug: String,
    pub description: String,
}

/// Request to update an existing category
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
}

/// Category response for API (public fields only)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Category> for CategoryResponse {
    fn from(cat: Category) -> Self {
        Self {
            id: cat.id,
            name: cat.name,
            slug: cat.slug,
            description: cat.description,
            created_at: cat.created_at.map(|dt| dt.to_string()),
            updated_at: cat.updated_at.map(|dt| dt.to_string()),
        }
    }
}

/// Search/filter parameters for categories
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CategoryQuery {
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
