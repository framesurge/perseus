use axum::{
    extract::{Path, State},
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

/// List all roles with their permissions
pub async fn list_roles(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServiceError> {
    let roles = db::list_roles(&state.pool).await.map_err(|e| {
        ServiceError::InternalServerError(format!("Failed to list roles: {}", e))
    })?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Roles retrieved successfully", roles)),
    ))
}

/// Get a specific role by ID
pub async fn get_role(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let role = db::get_role_with_permissions(&state.pool, role_id)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to get role: {}", e)))?
        .ok_or_else(|| ServiceError::NotFound("Role not found".to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Role retrieved successfully", role)),
    ))
}

/// Create a new role
pub async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateRole>,
) -> Result<impl IntoResponse, ServiceError> {
    let role = db::create_role(&state.pool, &input.name, &input.description, false)
        .await
        .map_err(|e| ServiceError::BadRequest(format!("Failed to create role: {}", e)))?;

    // Get the role with permissions
    let role_with_perms = db::get_role_with_permissions(&state.pool, role.id)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to get role: {}", e)))?
        .unwrap_or(RoleWithPermissions {
            id: role.id,
            name: role.name,
            description: role.description,
            is_system: role.is_system,
            permissions: vec![],
        });

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Role created successfully", role_with_perms)),
    ))
}

/// Update an existing role
pub async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<i32>,
    Json(input): Json<UpdateRole>,
) -> Result<impl IntoResponse, ServiceError> {
    db::update_role(
        &state.pool,
        role_id,
        input.name.as_deref(),
        input.description.as_deref(),
    )
    .await
    .map_err(|e| ServiceError::BadRequest(format!("Failed to update role: {}", e)))?
    .ok_or_else(|| ServiceError::NotFound("Role not found".to_string()))?;

    // Get updated role with permissions
    let role = db::get_role_with_permissions(&state.pool, role_id)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to get role: {}", e)))?
        .ok_or_else(|| ServiceError::NotFound("Role not found".to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Role updated successfully", role)),
    ))
}

/// Delete a role
pub async fn delete_role(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let deleted = db::delete_role(&state.pool, role_id)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to delete role: {}", e)))?;

    if deleted {
        Ok((
            StatusCode::OK,
            Json(ApiResponse::<()>::success_message("Role deleted successfully")),
        ))
    } else {
        Err(ServiceError::BadRequest(
            "Cannot delete system role or role not found".to_string(),
        ))
    }
}

/// Set permissions for a role
pub async fn set_role_permissions(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<i32>,
    Json(input): Json<SetPermissionsRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    // Validate permissions
    let valid_permissions = Permission::all();
    for perm in &input.permissions {
        if !valid_permissions.contains(&perm.as_str()) {
            return Err(ServiceError::BadRequest(format!(
                "Invalid permission: {}",
                perm
            )));
        }
    }

    db::set_role_permissions(&state.pool, role_id, &input.permissions)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to set permissions: {}", e)))?;

    // Get updated role with permissions
    let role = db::get_role_with_permissions(&state.pool, role_id)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to get role: {}", e)))?
        .ok_or_else(|| ServiceError::NotFound("Role not found".to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Permissions updated successfully", role)),
    ))
}

/// Assign a role to a user
pub async fn assign_role(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AssignRoleRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    db::assign_role_to_user(&state.pool, input.user_id, input.role_id)
        .await
        .map_err(|e| ServiceError::BadRequest(format!("Failed to assign role: {}", e)))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::success_message("Role assigned successfully")),
    ))
}

/// Remove a role from a user
pub async fn remove_role(
    State(state): State<Arc<AppState>>,
    Path((user_id, role_id)): Path<(i32, i32)>,
) -> Result<impl IntoResponse, ServiceError> {
    let removed = db::remove_role_from_user(&state.pool, user_id, role_id)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to remove role: {}", e)))?;

    if removed {
        Ok((
            StatusCode::OK,
            Json(ApiResponse::<()>::success_message("Role removed successfully")),
        ))
    } else {
        Err(ServiceError::NotFound(
            "User role assignment not found".to_string(),
        ))
    }
}

/// Get user's permissions
pub async fn get_user_permissions(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, ServiceError> {
    let permissions = db::get_user_permissions(&state.pool, user_id)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to get user permissions: {}", e)))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "User permissions retrieved",
            PermissionCheckResponse {
                success: true,
                has_permission: !permissions.is_empty(),
                permissions,
            },
        )),
    ))
}

/// Check if user has a specific permission
pub async fn check_permission(
    State(state): State<Arc<AppState>>,
    Path((user_id, permission)): Path<(i32, String)>,
) -> Result<impl IntoResponse, ServiceError> {
    let has_perm = db::user_has_permission(&state.pool, user_id, &permission)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to check permission: {}", e)))?;

    let permissions = if has_perm {
        vec![permission]
    } else {
        vec![]
    };

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Permission check complete",
            PermissionCheckResponse {
                success: true,
                has_permission: has_perm,
                permissions,
            },
        )),
    ))
}

/// Get all available permissions
pub async fn list_permissions() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::success("Available permissions", Permission::all())),
    )
}
