use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;
use utoipa::{IntoParams, ToResponse, ToSchema};

//To be added based on special query
#[derive(
    Serialize, Debug, ToSchema, Clone, ToResponse, IntoParams, Deserialize, PostgresMapper, Default,
)]
#[schema(example = json!({"class": "post inline"}))]
#[response(description = "Category Lists")]
#[pg_mapper(table = "categories")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(
    Serialize, Debug, ToSchema, Clone, ToResponse, IntoParams, Deserialize, PostgresMapper, Default,
)]
#[schema(example = json!({"class": "form inline"}))]
#[response(description = "Add a new category")]
#[pg_mapper(table = "categories")]
pub struct CreateCategory {
    pub name: String,
    pub slug: String,
    pub description: String,
}

/// Search todos Query

#[derive(Deserialize, Debug, Clone, IntoParams, PostgresMapper)]
#[pg_mapper(table = "categories")]
pub struct SearchCategory {
    /// Content that should be found from Todo's value field
    pub name: String,
}
