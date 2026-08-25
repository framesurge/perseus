use serde::{Deserialize, Serialize};

/// Generic API response used across the application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T = ()> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn success_message(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

/// Session data for client-side auth
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionData {
    pub session_id: i32,
    pub session_verifier: String,
}
