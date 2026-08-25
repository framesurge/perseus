use serde::{Deserialize, Serialize};

/// Request to create a new user (register)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

/// Request for login
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// User data stored in the database
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// User with just ID and password hash (for password verification)
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserPw {
    pub id: i32,
    pub hashed_password: String,
}

/// User with password validation hash (for password reset)
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserValidateHash {
    pub id: i32,
    pub reset_password_validator_hash: String,
}

/// User session data stored in the database
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserSession {
    pub id: i32,
    pub user_id: i32,
    pub session_verifier: String,
    pub otp_code_confirmed: bool,
    pub otp_code_encrypted: String,
    pub otp_code_attempts: i32,
    pub otp_code_sent: bool,
}

/// Data for creating a new session
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SessionAdd {
    pub user_id: i32,
    pub session_verifier: String,
    pub otp_code_encr: String,
}

/// Session data returned to the client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: i32,
    pub session_verifier: String,
    pub master_key_hash: Option<String>,
}

/// Created user response
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreatedUser {
    pub id: i32,
}

/// Created session response
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreatedSession {
    pub id: i32,
}

/// OTP confirmation request
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OtpConfirmRequest {
    pub code: String,
    pub session_id: i32,
    pub session_verifier: String,
}

/// Password reset request parameters
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PasswordResetParams {
    pub reset_password_selector: String,
    pub reset_password_validator: String,
}

/// Password reset with new password
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PasswordResetRequest {
    pub password: String,
    pub reset_password_selector: String,
    pub reset_password_validator: String,
}

/// Email-only request (for password reset request)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailRequest {
    pub email: String,
}

/// User profile response (public data)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: i32,
    pub email: String,
    pub created_at: Option<String>,
    pub otp_confirmed: bool,
    pub roles: Vec<String>,
}

// Re-export ApiResponse from shared types module
pub use crate::types::ApiResponse;
