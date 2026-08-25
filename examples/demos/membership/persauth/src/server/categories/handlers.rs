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

/// List all categories
pub async fn list_categories(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CategoryQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let categories = if let Some(search) = query.search {
        db::search_categories(&client, &search, query.limit, query.offset).await?
    } else {
        db::list_categories(&client, query.limit, query.offset).await?
    };

    let responses: Vec<CategoryResponse> = categories.into_iter().map(CategoryResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Categories retrieved", responses)),
    ))
}

/// Get a single category by ID
pub async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let category = db::get_category_by_id(&client, id).await?;
    let response = CategoryResponse::from(category);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Category retrieved", response)),
    ))
}

/// Get a single category by slug
pub async fn get_category_by_slug(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let category = db::get_category_by_slug(&client, &slug).await?;
    let response = CategoryResponse::from(category);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Category retrieved", response)),
    ))
}

/// Create a new category
pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategory>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Check if slug already exists
    if db::slug_exists(&client, &payload.slug).await? {
        return Err(ServiceError::Conflict(format!(
            "Category with slug '{}' already exists",
            payload.slug
        )));
    }

    let category = db::add_category(&client, &payload).await?;
    let response = CategoryResponse::from(category);

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Category created", response)),
    ))
}

/// Update an existing category
pub async fn update_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCategory>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Check if new slug conflicts with existing category
    if let Some(ref new_slug) = payload.slug {
        let existing = db::get_category_by_id(&client, id).await?;
        if new_slug != &existing.slug && db::slug_exists(&client, new_slug).await? {
            return Err(ServiceError::Conflict(format!(
                "Category with slug '{}' already exists",
                new_slug
            )));
        }
    }

    let category = db::update_category(&client, id, &payload).await?;
    let response = CategoryResponse::from(category);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Category updated", response)),
    ))
}

/// Delete a category
pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    db::delete_category(&client, id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::success_message("Category deleted")),
    ))
}
