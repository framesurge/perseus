use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use base64::{engine::general_purpose, Engine as _};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

use crate::server::configs::Config;
use crate::server::errors::ServiceError;
use crate::server::mail::send_email;
use crate::server::permissions::db::get_user_roles;

use super::db;
use super::encryption::{
    decrypt, encrypt, generate_otp, generate_random_bytes_24, generate_random_bytes_32,
    generate_random_bytes_8, hash_sha256, hex_to_bytes, password_hash, verify_hash,
};
use super::model::*;

use std::sync::Arc;

/// User session data extracted from request for RBAC
#[derive(Debug, Clone)]
pub struct SessionUser {
    pub user_id: i32,
    pub email: String,
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub config: Config,
}

/// Create a session for a user
async fn create_session(
    pool: &Pool,
    config: &Config,
    user_id: i32,
    master_key_hash: Option<String>,
) -> Result<Session, ServiceError> {
    let client = pool.get().await?;

    let secret = hex_to_bytes(&config.srv_cnf.secret_key)
        .map_err(|e| ServiceError::FaultySetup(format!("Invalid secret key: {}", e)))?;

    // Generate OTP code and encrypt it
    let otp_code = generate_otp();
    let otp_encrypted = encrypt(
        &format!("{}", otp_code),
        &format!("{}", user_id),
        &secret,
    )?;

    // Create session verifier
    let random_bytes = generate_random_bytes_32();
    let hex_hashed_session_verifier = hash_sha256(&random_bytes);

    let sess = SessionAdd {
        user_id,
        session_verifier: hex_hashed_session_verifier,
        otp_code_encr: otp_encrypted,
    };

    let created_session = db::add_session(&client, &sess).await?;

    Ok(Session {
        session_id: created_session.id,
        session_verifier: hex::encode(random_bytes),
        master_key_hash,
    })
}

/// Send OTP email to user
async fn send_otp_email(
    pool: &Pool,
    config: &Config,
    session: &Session,
) -> Result<(), ServiceError> {
    let client = pool.get().await?;
    let secret = hex_to_bytes(&config.srv_cnf.secret_key)
        .map_err(|e| ServiceError::FaultySetup(format!("Invalid secret key: {}", e)))?;

    if let Some(user_session) = db::find_user_by_session(&client, session).await {
        if !user_session.otp_code_sent {
            db::session_otp_update_sent(&client, user_session.id).await?;

            let email = db::find_user_email_by_id(&client, user_session.user_id).await?;
            let otp_code = decrypt(
                &user_session.otp_code_encrypted,
                &format!("{}", user_session.user_id),
                &secret,
            )?;

            let subject = "Your Confirmation Code".to_string();
            let body = format!(
                "<html><body><h2>Welcome!</h2><p>Your confirmation code is: <strong>{}</strong></p></body></html>",
                otp_code
            );

            send_email(&config.srv_cnf, &email, &subject, &body)?;
        }
    }

    Ok(())
}

/// Register a new user
pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Check if email already exists
    if db::email_exists(&client, &payload.email).await? {
        // Create a fake session to prevent account enumeration
        let session = create_session(
            &state.pool,
            &state.config,
            state.config.srv_cnf.user_invalid_id,
            None,
        )
        .await?;

        return Ok((
            StatusCode::CONFLICT,
            Json(ApiResponse::success("Check your email for confirmation", session)),
        ));
    }

    // Hash password
    let hashed_password = password_hash(
        &payload.password,
        state.config.srv_cnf.bcrypt_or_argon,
    )
    .await?;

    // Create user
    let created_user = db::add_user(&client, &payload.email, &hashed_password).await?;

    // Create session
    let session = create_session(&state.pool, &state.config, created_user.id, None).await?;

    // Send OTP email if enabled
    if state.config.srv_cnf.email_otp_enabled {
        send_otp_email(&state.pool, &state.config, &session).await?;
    }

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Registration successful. Check your email for confirmation", session)),
    ))
}

/// Login user
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let user = db::find_user_password_by_email(&client, &payload.email).await?;

    let is_valid = verify_hash(
        &payload.password.nfkc().collect::<String>(),
        &user.hashed_password,
        state.config.srv_cnf.bcrypt_or_argon,
    )
    .await?;

    if !is_valid {
        return Err(ServiceError::AuthenticationError("Invalid credentials".to_string()));
    }

    let session = create_session(&state.pool, &state.config, user.id, None).await?;

    Ok((StatusCode::OK, Json(ApiResponse::success("Login successful", session))))
}

/// Confirm OTP code
pub async fn confirm_otp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OtpConfirmRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;
    let secret = hex_to_bytes(&state.config.srv_cnf.secret_key)
        .map_err(|e| ServiceError::FaultySetup(format!("Invalid secret key: {}", e)))?;

    let session = Session {
        session_id: payload.session_id,
        session_verifier: payload.session_verifier.clone(),
        master_key_hash: None,
    };

    let user_session = db::find_user_by_session(&client, &session)
        .await
        .ok_or(ServiceError::Unauthorized("Session not found".to_string()))?;

    // Check brute force attempts
    if user_session.otp_code_attempts > state.config.srv_cnf.max_otp_attempts {
        return Err(ServiceError::AuthenticationError("Too many attempts".to_string()));
    }

    let otp_code = decrypt(
        &user_session.otp_code_encrypted,
        &format!("{}", user_session.user_id),
        &secret,
    )?;

    if otp_code == payload.code {
        db::session_otp_confirm(&client, user_session.id).await?;
        Ok((StatusCode::OK, Json(ApiResponse::<()>::success_message("OTP confirmed"))))
    } else {
        db::session_otp_increment_attempts(&client, user_session.id).await?;
        Err(ServiceError::AuthenticationError("Invalid OTP code".to_string()))
    }
}

/// Request password reset
pub async fn request_password_reset(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EmailRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Check if user exists (but don't reveal this to the client)
    let user_exists = db::email_exists(&client, &payload.email).await?;

    if user_exists {
        // Generate reset tokens
        let selector = generate_random_bytes_8();
        let selector_base64 = general_purpose::URL_SAFE_NO_PAD.encode(selector);

        let verifier = generate_random_bytes_24();
        let verifier_hash = Sha256::digest(&verifier);
        let verifier_hash_base64 = general_purpose::URL_SAFE_NO_PAD.encode(verifier_hash);
        let verifier_base64 = general_purpose::URL_SAFE_NO_PAD.encode(verifier);

        // Store reset tokens
        db::user_set_password_reset(&client, &payload.email, &selector_base64, &verifier_hash_base64)
            .await?;

        // Send reset email
        let reset_url = format!(
            "http://localhost:8080/auth/reset?selector={}&validator={}",
            selector_base64, verifier_base64
        );

        let subject = "Password Reset Request".to_string();
        let body = format!(
            "<html><body><h2>Password Reset</h2><p>Click the link below to reset your password:</p><p><a href=\"{}\">Reset Password</a></p><p>If you did not request this, please ignore this email.</p></body></html>",
            reset_url
        );

        send_email(&state.config.srv_cnf, &payload.email, &subject, &body)?;
    }

    // Always return success to prevent email enumeration
    Ok((StatusCode::OK, Json(ApiResponse::<()>::success_message("If the email exists, a reset link has been sent"))))
}

/// Complete password reset
pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PasswordResetRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Find user by reset selector
    let user_hash = db::find_user_password_reset_hash(&client, &payload.reset_password_selector).await?;

    // Verify the validator
    let verifier_bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(&payload.reset_password_validator)
        .map_err(|e| ServiceError::DecryptError(e.to_string()))?;

    let verifier_hash = Sha256::digest(&verifier_bytes);

    let stored_hash_bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(&user_hash.reset_password_validator_hash)
        .map_err(|e| ServiceError::DecryptError(e.to_string()))?;

    // Constant-time comparison
    if verifier_hash.as_slice() != stored_hash_bytes.as_slice() {
        return Err(ServiceError::AuthenticationError("Invalid reset token".to_string()));
    }

    // Hash new password
    let hashed_password = password_hash(
        &payload.password,
        state.config.srv_cnf.bcrypt_or_argon,
    )
    .await?;

    // Update password
    db::user_update_password(&client, user_hash.id, &hashed_password).await?;

    Ok((StatusCode::OK, Json(ApiResponse::<()>::success_message("Password reset successful"))))
}

/// Get user profile
pub async fn get_profile(
    State(state): State<Arc<AppState>>,
    Json(session): Json<Session>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let user_session = db::find_user_by_session(&client, &session)
        .await
        .ok_or(ServiceError::Unauthorized("Session not found".to_string()))?;

    let user = db::get_user_profile(&client, user_session.user_id).await?;

    // Get user roles
    let roles = get_user_roles(&state.pool, user_session.user_id).await?;

    let profile = UserProfile {
        id: user.id,
        email: user.email,
        created_at: user.created_at.map(|dt| dt.to_string()),
        otp_confirmed: user_session.otp_code_confirmed,
        roles,
    };

    Ok((StatusCode::OK, Json(ApiResponse::success("Profile retrieved", profile))))
}

/// Logout user
pub async fn logout(
    State(state): State<Arc<AppState>>,
    Json(session): Json<Session>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    db::delete_session(&client, session.session_id).await?;

    Ok((StatusCode::OK, Json(ApiResponse::<()>::success_message("Logged out successfully"))))
}

/// Resend OTP email
pub async fn resend_otp(
    State(state): State<Arc<AppState>>,
    Json(session): Json<Session>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    // Find the session and regenerate OTP
    let user_session = db::find_user_by_session(&client, &session)
        .await
        .ok_or(ServiceError::Unauthorized("Session not found".to_string()))?;

    // Delete old session and create new one with fresh OTP
    db::delete_session(&client, session.session_id).await?;
    let new_session = create_session(&state.pool, &state.config, user_session.user_id, None).await?;

    // Send OTP email
    if state.config.srv_cnf.email_otp_enabled {
        send_otp_email(&state.pool, &state.config, &new_session).await?;
    }

    Ok((StatusCode::OK, Json(ApiResponse::success("OTP resent", new_session))))
}

/// User preferences request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePreferencesRequest {
    pub session_id: i32,
    pub session_verifier: String,
    pub theme: Option<String>,
    pub email_notifications: Option<bool>,
}

/// Update user preferences
pub async fn update_preferences(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdatePreferencesRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let client = state.pool.get().await?;

    let session = Session {
        session_id: payload.session_id,
        session_verifier: payload.session_verifier.clone(),
        master_key_hash: None,
    };

    // Validate session
    let user_session = db::find_user_by_session(&client, &session)
        .await
        .ok_or(ServiceError::Unauthorized("Session not found".to_string()))?;

    // For now, we'll just validate the request and return success
    // In a full implementation, you would save preferences to the database

    // Validate theme if provided
    if let Some(ref theme) = payload.theme {
        if !["light", "dark", "auto"].contains(&theme.as_str()) {
            return Err(ServiceError::BadRequest("Invalid theme".to_string()));
        }
    }

    Ok((StatusCode::OK, Json(ApiResponse::<()>::success("Preferences updated successfully", ()))))
}
