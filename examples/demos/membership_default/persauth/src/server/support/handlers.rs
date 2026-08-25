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

/// Create a new support message (public endpoint)
pub async fn create_message(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateSupportMessageRequest>,
) -> Result<AxumJson<ApiResponse<i32>>, ServiceError> {
    // Clone all values first
    let sender_name = req.sender_name.clone();
    let sender_email = req.sender_email.clone();
    let subject = req.subject.clone();
    let body = req.body.clone();

    // Validate session if present (optional - allows logged in users)
    let user_id = validate_session_optional(&state, &sender_email).await;

    // Create the message
    let data = CreateSupportMessageWithUser {
        sender_name: sender_name.clone(),
        sender_email: sender_email.clone(),
        user_id,
        subject: subject.clone(),
        body: body.clone(),
    };

    let id = create_message_db(&state.pool, data)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    // Send admin notification email
    let admin_email = state.config.srv_cnf.smtp_admin_email.clone();
    let email_subject = format!("New Support Message: {}", subject);
    let email_body = format!(
        r#"
        <html>
        <body>
            <h2>New Support Message</h2>
            <p><strong>From:</strong> {} ({})</p>
            <p><strong>Subject:</strong> {}</p>
            <p><strong>Message:</strong></p>
            <p>{}</p>
            <p><a href="http://{}:{}/admin">View in Admin Dashboard</a></p>
        </body>
        </html>
        "#,
        sender_name,
        sender_email,
        subject,
        body,
        state.config.srv_cnf.host,
        state.config.srv_cnf.port
    );

    let _ = crate::server::mail::send_email(&state.config.srv_cnf, &admin_email, &email_subject, &email_body)
        .map_err(|e| log::error!("Failed to send admin notification email: {}", e));

    Ok(AxumJson(ApiResponse {
        success: true,
        message: "Support message created successfully".to_string(),
        data: Some(id),
    }))
}

/// List all support messages (admin only)
pub async fn list_messages(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ListMessagesRequest>,
) -> Result<AxumJson<ListMessagesResponse>, ServiceError> {
    // Build session from request
    let session = Session {
        session_id: req.session_id,
        session_verifier: req.session_verifier,
        master_key_hash: None,
    };

    // Validate session and check permission
    let user = validate_session(&state, &session).await?;
    check_role(&user, &["admin", "editor"])?;

    let limit = req.limit.unwrap_or(50);
    let offset = req.offset.unwrap_or(0);
    let status = req.status.as_deref();

    let messages = list_messages_db(&state.pool, status, limit, offset)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;
    let (total, _new, _read, _replied, _closed) = count_by_status_db(&state.pool)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    Ok(AxumJson(ListMessagesResponse {
        success: true,
        message: "Messages retrieved successfully".to_string(),
        messages: messages.into_iter().map(|m| m.to_response()).collect(),
        total,
    }))
}

/// List messages for the authenticated user
pub async fn list_user_messages(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ListUserMessagesRequest>,
) -> Result<AxumJson<ListMessagesResponse>, ServiceError> {
    // Build session from request
    let session = Session {
        session_id: req.session_id,
        session_verifier: req.session_verifier,
        master_key_hash: None,
    };

    // Validate session
    let user = validate_session(&state, &session).await?;

    let messages = list_user_messages_db(&state.pool, user.user_id)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    let total = messages.len() as i64;
    Ok(AxumJson(ListMessagesResponse {
        success: true,
        message: "Messages retrieved successfully".to_string(),
        messages: messages.into_iter().map(|m| m.to_response()).collect(),
        total,
    }))
}

/// Get a single message by ID (admin or owner)
pub async fn get_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<GetMessageRequest>,
) -> Result<AxumJson<MessageResponse>, ServiceError> {
    // Build session from request
    let session = Session {
        session_id: req.session_id,
        session_verifier: req.session_verifier,
        master_key_hash: None,
    };

    // Validate session
    let user = validate_session(&state, &session).await?;

    // Get the message
    let message = get_message_by_id_db(&state.pool, id)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    match message {
        Some(m) => {
            // Check permission: admin can view all, users can only view their own
            let is_admin = check_role_simple(&user, &["admin", "editor"]);
            let is_owner = m.user_id == Some(user.user_id);

            if !is_admin && !is_owner {
                return Err(ServiceError::Forbidden("Access denied".to_string()));
            }

            // Mark as read if new and admin is viewing
            if is_admin && m.status == SupportStatus::New {
                let _ = update_message_status_db(&state.pool, id, "read")
                    .await
                    .map_err(|e| log::error!("Failed to update message status: {}", e));
            }

            Ok(AxumJson(MessageResponse {
                success: true,
                message: "Message retrieved successfully".to_string(),
                data: Some(m.to_response()),
            }))
        }
        None => {
            Ok(AxumJson(MessageResponse {
                success: false,
                message: "Message not found".to_string(),
                data: None,
            }))
        }
    }
}

/// Update a message (admin only - reply or status change)
pub async fn update_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateMessageRequest>,
) -> Result<AxumJson<MessageResponse>, ServiceError> {
    // Build session from request
    let session = Session {
        session_id: req.session_id,
        session_verifier: req.session_verifier,
        master_key_hash: None,
    };

    // Validate session and check permission
    let user = validate_session(&state, &session).await?;
    check_role(&user, &["admin", "editor"])?;

    // Get the message first
    let message = get_message_by_id_db(&state.pool, id)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    let message = match message {
        Some(m) => m,
        None => {
            return Ok(AxumJson(MessageResponse {
                success: false,
                message: "Message not found".to_string(),
                data: None,
            }));
        }
    };

    // Update status if provided
    if let Some(status) = &req.status {
        let _ = update_message_status_db(&state.pool, id, status)
            .await
            .map_err(|e| log::error!("Failed to update status: {}", e));
    }

    // Update reply if provided
    if let Some(reply) = &req.admin_reply {
        let reply_clone = reply.clone();
        let sender_email = message.sender_email.clone();
        let msg_subject = message.subject.clone();
        let msg_body = message.body.clone();

        update_message_reply_db(&state.pool, id, reply, user.user_id)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

        // Send reply email to sender
        let subject = format!("Re: {}", msg_subject);
        let body = format!(
            r#"
            <html>
            <body>
                <h2>Support Message Reply</h2>
                <p>Hi {},</p>
                <p>We've responded to your support message:</p>
                <blockquote style="border-left: 3px solid #667eea; padding-left: 15px; margin: 15px 0;">
                    <strong>Original Message:</strong><br/>
                    {}<br/><br/>
                    <strong>Our Response:</strong><br/>
                    {}
                </blockquote>
                <p>You can reply to this email or <a href="http://{}:{}/contact">submit a new message</a> if you have more questions.</p>
            </body>
            </html>
            "#,
            message.sender_name,
            msg_body,
            reply_clone,
            state.config.srv_cnf.host,
            state.config.srv_cnf.port
        );

        let _ = crate::server::mail::send_email(&state.config.srv_cnf, &sender_email, &subject, &body)
            .map_err(|e| log::error!("Failed to send reply email: {}", e));
    }

    // Get updated message
    let updated = get_message_by_id_db(&state.pool, id)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?
        .unwrap();

    Ok(AxumJson(MessageResponse {
        success: true,
        message: "Message updated successfully".to_string(),
        data: Some(updated.to_response()),
    }))
}

/// Delete a message (admin only)
pub async fn delete_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<DeleteMessageRequest>,
) -> Result<AxumJson<ApiResponse>, ServiceError> {
    // Build session from request
    let session = Session {
        session_id: req.session_id,
        session_verifier: req.session_verifier,
        master_key_hash: None,
    };

    // Validate session and check permission
    let user = validate_session(&state, &session).await?;
    check_role(&user, &["admin"])?;

    let deleted = delete_message_db(&state.pool, id)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    if deleted {
        Ok(AxumJson(ApiResponse {
            success: true,
            message: "Message deleted successfully".to_string(),
            data: None,
        }))
    } else {
        Ok(AxumJson(ApiResponse {
            success: false,
            message: "Message not found".to_string(),
            data: None,
        }))
    }
}

/// Helper: validate session optionally (returns user_id if valid session exists)
async fn validate_session_optional(
    state: &Arc<AppState>,
    email: &str,
) -> Option<i32> {
    let client = state.pool.get().await.ok()?;
    let row = client
        .query_one(
            "SELECT user_id FROM users WHERE email = $1",
            &[&email],
        )
        .await
        .ok()?;
    Some(row.get("user_id"))
}

/// Helper: check if user has allowed role
fn check_role(user: &AuthenticatedUser, allowed_roles: &[&str]) -> Result<(), ServiceError> {
    if !user.roles.iter().any(|r| allowed_roles.contains(&r.as_str())) {
        return Err(ServiceError::Forbidden("Access denied".to_string()));
    }
    Ok(())
}

/// Helper: check role without error
fn check_role_simple(user: &AuthenticatedUser, allowed_roles: &[&str]) -> bool {
    user.roles.iter().any(|r| allowed_roles.contains(&r.as_str()))
}
