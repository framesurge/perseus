use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

// extern crate chrono;
//use chrono::prelude::*;
//use chrono::{DateTime, Duration, Utc};
use chrono::Utc;
use utoipa::ToSchema;
//To be added based on special query

#[derive(Serialize, Debug, Clone, Deserialize, ToSchema, PostgresMapper, Default)]
#[pg_mapper(table = "posts")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    // pub date_created: chrono::DateTime<Utc>,
    // pub date_published: chrono::DateTime<Utc>,
}

#[derive(Serialize, Debug, Clone, Deserialize, ToSchema, PostgresMapper, Default)]
#[pg_mapper(table = "posts")]
pub struct CreatePost {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
}
