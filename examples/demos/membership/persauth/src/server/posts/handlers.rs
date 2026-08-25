use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::server::auth::AppState;
use crate::server::auth::model::ApiResponse;
use crate::server::errors::ServiceError;

use super::db;
use super::models::*;

/// List all posts
pub async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PostQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let posts = if let Some(search) = query.search {
        db::search_posts(&client, &search, query.limit, query.offset).await?
    } else {
        db::list_posts(&client, query.limit, query.offset).await?
    };

    let responses: Vec<PostResponse> = posts.into_iter().map(PostResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Posts retrieved", responses)),
    ))
}

/// Get a single post by ID
pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let post = db::get_post_by_id(&client, id).await?;
    let response = PostResponse::from(post);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Post retrieved", response)),
    ))
}

/// Get a single post by slug
pub async fn get_post_by_slug(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let post = db::get_post_by_slug(&client, &slug).await?;
    let response = PostResponse::from(post);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Post retrieved", response)),
    ))
}

/// Create a new post
pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePost>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Check if slug already exists
    if db::slug_exists(&client, &payload.slug).await? {
        return Err(ServiceError::Conflict(format!(
            "Post with slug '{}' already exists",
            payload.slug
        )));
    }

    let post = db::add_post(&client, &payload).await?;
    let response = PostResponse::from(post);

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Post created", response)),
    ))
}

/// Update an existing post
pub async fn update_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePost>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Check if new slug conflicts with existing post
    if let Some(ref new_slug) = payload.slug {
        let existing = db::get_post_by_id(&client, id).await?;
        if new_slug != &existing.slug && db::slug_exists(&client, new_slug).await? {
            return Err(ServiceError::Conflict(format!(
                "Post with slug '{}' already exists",
                new_slug
            )));
        }
    }

    let post = db::update_post(&client, id, &payload).await?;
    let response = PostResponse::from(post);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Post updated", response)),
    ))
}

/// Delete a post
pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    db::delete_post(&client, id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::success_message("Post deleted")),
    ))
}
