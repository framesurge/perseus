use serde::{Deserialize, Serialize};

/// Tag entity stored in the database
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Request to create a new tag
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTag {
    pub name: String,
    pub slug: String,
}

/// Request to update an existing tag
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub slug: Option<String>,
}

/// Tag response for API (public fields only)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
            slug: tag.slug,
            created_at: tag.created_at.map(|dt| dt.to_string()),
            updated_at: tag.updated_at.map(|dt| dt.to_string()),
        }
    }
}

/// Search/filter parameters for tags
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TagQuery {
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Request to associate tags with a post
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostTagsRequest {
    pub tag_ids: Vec<i32>,
}
