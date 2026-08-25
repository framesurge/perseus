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

/// List all tags
pub async fn list_tags(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TagQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let tags = if let Some(search) = query.search {
        db::search_tags(&client, &search, query.limit, query.offset).await?
    } else {
        db::list_tags(&client, query.limit, query.offset).await?
    };

    let responses: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Tags retrieved", responses)),
    ))
}

/// Get a single tag by ID
pub async fn get_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let tag = db::get_tag_by_id(&client, id).await?;
    let response = TagResponse::from(tag);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Tag retrieved", response)),
    ))
}

/// Get a single tag by slug
pub async fn get_tag_by_slug(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let tag = db::get_tag_by_slug(&client, &slug).await?;
    let response = TagResponse::from(tag);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Tag retrieved", response)),
    ))
}

/// Create a new tag
pub async fn create_tag(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTag>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Check if slug already exists
    if db::slug_exists(&client, &payload.slug).await? {
        return Err(ServiceError::Conflict(format!(
            "Tag with slug '{}' already exists",
            payload.slug
        )));
    }

    let tag = db::add_tag(&client, &payload).await?;
    let response = TagResponse::from(tag);

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Tag created", response)),
    ))
}

/// Update an existing tag
pub async fn update_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTag>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Check if new slug conflicts with existing tag
    if let Some(ref new_slug) = payload.slug {
        let existing = db::get_tag_by_id(&client, id).await?;
        if new_slug != &existing.slug && db::slug_exists(&client, new_slug).await? {
            return Err(ServiceError::Conflict(format!(
                "Tag with slug '{}' already exists",
                new_slug
            )));
        }
    }

    let tag = db::update_tag(&client, id, &payload).await?;
    let response = TagResponse::from(tag);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Tag updated", response)),
    ))
}

/// Delete a tag
pub async fn delete_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    db::delete_tag(&client, id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::success_message("Tag deleted")),
    ))
}

/// Get tags for a specific post
pub async fn get_post_tags(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let tags = db::get_tags_for_post(&client, post_id).await?;
    let responses: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Post tags retrieved", responses)),
    ))
}

/// Set tags for a post (replaces all existing tags)
pub async fn set_post_tags(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
    Json(payload): Json<PostTagsRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    db::add_tags_to_post(&client, post_id, &payload.tag_ids).await?;

    let tags = db::get_tags_for_post(&client, post_id).await?;
    let responses: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Post tags updated", responses)),
    ))
}
