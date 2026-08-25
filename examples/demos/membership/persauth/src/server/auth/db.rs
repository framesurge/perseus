use deadpool_postgres::Client;
use sha2::{Digest, Sha256};

use crate::server::errors::ServiceError;

use super::encryption::constant_time_compare;
use super::model::*;

/// Add a new user to the database
pub async fn add_user(client: &Client, email: &str, hashed_password: &str) -> Result<CreatedUser, ServiceError> {
    let statement = client
        .prepare("INSERT INTO users (email, hashed_password) VALUES ($1, $2) RETURNING user_id")
        .await?;

    let row = client
        .query_one(&statement, &[&email, &hashed_password])
        .await?;

    Ok(CreatedUser {
        id: row.get("user_id"),
    })
}

/// Add a new session to the database
pub async fn add_session(client: &Client, sess: &SessionAdd) -> Result<CreatedSession, ServiceError> {
    let statement = client
        .prepare(
            "INSERT INTO sessions (user_id, session_verifier, otp_code_encrypted)
             VALUES ($1, $2, $3) RETURNING id",
        )
        .await?;

    let row = client
        .query_one(
            &statement,
            &[&sess.user_id, &sess.session_verifier, &sess.otp_code_encr],
        )
        .await?;

    Ok(CreatedSession { id: row.get("id") })
}

/// Delete a session from the database
pub async fn delete_session(client: &Client, session_id: i32) -> Result<(), ServiceError> {
    let statement = client
        .prepare("DELETE FROM sessions WHERE id = $1")
        .await?;

    client.execute(&statement, &[&session_id]).await?;
    Ok(())
}

/// Find a user session by session data (with verification)
pub async fn find_user_by_session(client: &Client, session: &Session) -> Option<UserSession> {
    let statement = client
        .prepare(
            "SELECT id, user_id, session_verifier, otp_code_confirmed,
             otp_code_encrypted, otp_code_attempts, otp_code_sent
             FROM sessions WHERE id = $1",
        )
        .await
        .ok()?;

    let row = client
        .query_opt(&statement, &[&session.session_id])
        .await
        .ok()??;

    let user_session = UserSession {
        id: row.get("id"),
        user_id: row.get("user_id"),
        session_verifier: row.get("session_verifier"),
        otp_code_confirmed: row.get("otp_code_confirmed"),
        otp_code_encrypted: row.get("otp_code_encrypted"),
        otp_code_attempts: row.get("otp_code_attempts"),
        otp_code_sent: row.get("otp_code_sent"),
    };

    // Verify session verifier using constant-time comparison
    let decoded_hash = hex::decode(&session.session_verifier).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&decoded_hash);
    let ver_hash = hex::encode(hasher.finalize());

    if constant_time_compare(&ver_hash, &user_session.session_verifier) {
        Some(user_session)
    } else {
        None
    }
}

/// Find user password by email
pub async fn find_user_password_by_email(client: &Client, email: &str) -> Result<UserPw, ServiceError> {
    let statement = client
        .prepare("SELECT user_id, hashed_password FROM users WHERE email = $1")
        .await?;

    let row = client
        .query_opt(&statement, &[&email])
        .await?
        .ok_or_else(|| ServiceError::NotFound("User not found".to_string()))?;

    Ok(UserPw {
        id: row.get("user_id"),
        hashed_password: row.get("hashed_password"),
    })
}

/// Find user email by ID
pub async fn find_user_email_by_id(client: &Client, id: i32) -> Result<String, ServiceError> {
    let statement = client
        .prepare("SELECT email FROM users WHERE user_id = $1")
        .await?;

    let row = client
        .query_opt(&statement, &[&id])
        .await?
        .ok_or(ServiceError::BadId)?;

    Ok(row.get("email"))
}

/// Update session to mark OTP as sent
pub async fn session_otp_update_sent(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client
        .prepare("UPDATE sessions SET otp_code_sent = true WHERE id = $1")
        .await?;

    let result = client.execute(&statement, &[&id]).await?;

    if result == 1 {
        Ok(())
    } else {
        Err(ServiceError::DatabaseError("Failed to update session".to_string()))
    }
}

/// Update session to confirm OTP
pub async fn session_otp_confirm(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client
        .prepare("UPDATE sessions SET otp_code_confirmed = true, otp_code_attempts = 0 WHERE id = $1")
        .await?;

    let result = client.execute(&statement, &[&id]).await?;

    if result == 1 {
        Ok(())
    } else {
        Err(ServiceError::DatabaseError("Session confirmation not updated".to_string()))
    }
}

/// Increment OTP attempts counter
pub async fn session_otp_increment_attempts(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client
        .prepare("UPDATE sessions SET otp_code_attempts = otp_code_attempts + 1 WHERE id = $1")
        .await?;

    let result = client.execute(&statement, &[&id]).await?;

    if result == 1 {
        Ok(())
    } else {
        Err(ServiceError::DatabaseError("Attempt update failed".to_string()))
    }
}

/// Update user with password reset tokens
pub async fn user_set_password_reset(
    client: &Client,
    email: &str,
    selector: &str,
    validator_hash: &str,
) -> Result<(), ServiceError> {
    let statement = client
        .prepare(
            "UPDATE users SET reset_password_selector = $1,
             reset_password_validator_hash = $2, reset_password_sent_at = now()
             WHERE email = $3",
        )
        .await?;

    let result = client
        .execute(&statement, &[&selector, &validator_hash, &email])
        .await?;

    if result == 1 {
        Ok(())
    } else {
        Err(ServiceError::NotFound("User not found".to_string()))
    }
}

/// Find user password reset validation hash by selector
pub async fn find_user_password_reset_hash(
    client: &Client,
    selector: &str,
) -> Result<UserValidateHash, ServiceError> {
    let statement = client
        .prepare(
            "SELECT user_id, reset_password_validator_hash FROM users
             WHERE reset_password_selector = $1",
        )
        .await?;

    let row = client
        .query_opt(&statement, &[&selector])
        .await?
        .ok_or(ServiceError::BadId)?;

    Ok(UserValidateHash {
        id: row.get("user_id"),
        reset_password_validator_hash: row.get("reset_password_validator_hash"),
    })
}

/// Update user password hash and clear reset tokens
pub async fn user_update_password(client: &Client, id: i32, hashed_password: &str) -> Result<(), ServiceError> {
    let statement = client
        .prepare(
            "UPDATE users SET hashed_password = $1,
             reset_password_selector = NULL, reset_password_validator_hash = NULL
             WHERE user_id = $2",
        )
        .await?;

    let result = client.execute(&statement, &[&hashed_password, &id]).await?;

    if result == 1 {
        Ok(())
    } else {
        Err(ServiceError::DatabaseError("Password update failed".to_string()))
    }
}

/// Check if email already exists
pub async fn email_exists(client: &Client, email: &str) -> Result<bool, ServiceError> {
    let statement = client
        .prepare("SELECT 1 FROM users WHERE email = $1")
        .await?;

    let row = client.query_opt(&statement, &[&email]).await?;
    Ok(row.is_some())
}

/// Get user profile by ID
pub async fn get_user_profile(client: &Client, user_id: i32) -> Result<User, ServiceError> {
    let statement = client
        .prepare("SELECT user_id, email, hashed_password, created_at FROM users WHERE user_id = $1")
        .await?;

    let row = client
        .query_opt(&statement, &[&user_id])
        .await?
        .ok_or(ServiceError::NotFound("User not found".to_string()))?;

    Ok(User {
        id: row.get("user_id"),
        email: row.get("email"),
        hashed_password: row.get("hashed_password"),
        created_at: row.get("created_at"),
    })
}
