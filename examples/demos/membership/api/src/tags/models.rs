use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};
use utoipa::{IntoParams, ToResponse, ToSchema};

//To be added based on special query
//To be added based on special query
#[derive(
    Serialize, Debug, ToSchema, Clone, ToResponse, IntoParams, Deserialize, PostgresMapper, Default,
)]
#[schema(example = json!({"class": "post inline"}))]
#[response(description = "Category Lists")]
#[pg_mapper(table = "tags")]
pub struct Tags {
    pub id: i32,
    pub name: String,
}

#[derive(
    Serialize, Debug, ToSchema, Clone, ToResponse, IntoParams, Deserialize, PostgresMapper, Default,
)]
#[schema(example = json!({"class": "form inline"}))]
#[response(description = "Add a new category")]
#[pg_mapper(table = "tags")]
pub struct CreateTags {
    pub name: String,
}
