use super::models::*;
use super::db::*;
use crate::server::middleware::{validate_session, AuthenticatedUser};
use crate::server::AppState;
use crate::server::auth::Session;
use crate::server::errors::ServiceError;
use crate::types::ApiResponse;
use serde::{Deserialize, Serialize};
use axum::{
    extract::{Extension, Json, Path, State},
    response::Json as AxumJson,
};
use std::sync::Arc;

/// Dashboard stats response
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStatsResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DashboardStats>,
}

/// User list response
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    pub success: bool,
    pub message: String,
    pub users: Vec<AdminUserView>,
    pub total: i64,
}

/// Combined request for dashboard_stats
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStatsRequest {
    pub session_id: i32,
    pub session_verifier: String,
}

/// Combined request for list_users
#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersRequest {
    pub session_id: i32,
    pub session_verifier: String,
    pub search: Option<String>,
    pub role: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Combined request for get_user
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub session_id: i32,
    pub session_verifier: String,
}

/// Get dashboard statistics (admin only)
pub async fn dashboard_stats(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DashboardStatsRequest>,
) -> Result<AxumJson<DashboardStatsResponse>, ServiceError> {
    // Build session from request
    let session = Session {
        session_id: req.session_id,
        session_verifier: req.session_verifier,
        master_key_hash: None,
    };

    // Validate session and check permission
    let user = validate_session(&state, &session).await?;
    check_role(&user, &["admin", "editor"])?;

    let stats = get_dashboard_stats(&state.pool).await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    Ok(AxumJson(DashboardStatsResponse {
        success: true,
        message: "Dashboard statistics retrieved".to_string(),
        data: Some(stats),
    }))
}

/// List users (admin only)
pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ListUsersRequest>,
) -> Result<AxumJson<UserListResponse>, ServiceError> {
    // Build session from request
    let session = Session {
        session_id: req.session_id,
        session_verifier: req.session_verifier,
        master_key_hash: None,
    };

    // Validate session and check permission
    let user = validate_session(&state, &session).await?;
    check_role(&user, &["admin"])?;

    let limit = req.limit.unwrap_or(50);
    let offset = req.offset.unwrap_or(0);
    let search = req.search.as_deref();
    let role = req.role.as_deref();

    let (users, total) = list_users_db(&state.pool, search, role, limit, offset).await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    Ok(AxumJson(UserListResponse {
        success: true,
        message: "Users retrieved successfully".to_string(),
        users,
        total,
    }))
}

/// Get user details (admin only)
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(req): Json<GetUserRequest>,
) -> Result<AxumJson<ApiResponse<AdminUserView>>, ServiceError> {
    // Build session from request
    let session = Session {
        session_id: req.session_id,
        session_verifier: req.session_verifier,
        master_key_hash: None,
    };

    // Validate session and check permission
    let user = validate_session(&state, &session).await?;
    check_role(&user, &["admin"])?;

    let user_detail = get_user_detail(&state.pool, user_id).await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    match user_detail {
        Some(u) => Ok(AxumJson(ApiResponse {
            success: true,
            message: "User details retrieved".to_string(),
            data: Some(u),
        })),
        None => Ok(AxumJson(ApiResponse {
            success: false,
            message: "User not found".to_string(),
            data: None,
        })),
    }
}

/// Helper: check if user has allowed role
fn check_role(user: &AuthenticatedUser, allowed_roles: &[&str]) -> Result<(), ServiceError> {
    if !user.roles.iter().any(|r| allowed_roles.contains(&r.as_str())) {
        return Err(ServiceError::Forbidden("Access denied".to_string()));
    }
    Ok(())
}
