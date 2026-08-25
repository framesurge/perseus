//! Permission Checking Utilities
//!
//! This module provides utilities for enforcing RBAC policies
//! on API routes using the existing PostgreSQL database.

use crate::server::auth::{AppState, Session};
use crate::server::errors::ServiceError;
use crate::server::permissions::db::{get_user_permissions, get_user_roles};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Session extracted from request for authentication
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthSession {
    pub session_id: i32,
    pub session_verifier: String,
}

/// Authentication result attached to request extensions
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

/// Validate session and get authenticated user
pub async fn validate_session(
    state: &Arc<AppState>,
    session: &Session,
) -> Result<AuthenticatedUser, ServiceError> {
    // Validate session against database
    let client = state.pool.get().await?;
    let user_session = crate::server::auth::db::find_user_by_session(&client, session)
        .await
        .ok_or_else(|| ServiceError::Unauthorized("Invalid or expired session".to_string()))?;

    // Get user email
    let email = crate::server::auth::db::find_user_email_by_id(&client, user_session.user_id)
        .await
        .map_err(|_| ServiceError::Unauthorized("User not found".to_string()))?;

    // Get user roles and permissions
    let roles = get_user_roles(&state.pool, user_session.user_id)
        .await
        .unwrap_or_default();

    let permissions = get_user_permissions(&state.pool, user_session.user_id)
        .await
        .unwrap_or_default();

    Ok(AuthenticatedUser {
        user_id: user_session.user_id,
        email,
        roles,
        permissions,
    })
}

/// Check if user has required permission (admin bypasses all checks)
pub fn check_permission(user: &AuthenticatedUser, obj: &str, act: &str) -> bool {
    // Admin bypass
    if user.roles.contains(&"admin".to_string()) {
        return true;
    }

    let required = format!("{}:{}", obj, act);
    user.permissions.contains(&required)
}

/// Check if user has any of the allowed roles
pub fn check_role(user: &AuthenticatedUser, allowed_roles: &[&str]) -> bool {
    user.roles
        .iter()
        .any(|role| allowed_roles.contains(&role.as_str()))
}
