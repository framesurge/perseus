use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// Support message status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SupportStatus {
    New,
    Read,
    Replied,
    Closed,
}

impl std::fmt::Display for SupportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SupportStatus::New => write!(f, "new"),
            SupportStatus::Read => write!(f, "read"),
            SupportStatus::Replied => write!(f, "replied"),
            SupportStatus::Closed => write!(f, "closed"),
        }
    }
}

impl From<&str> for SupportStatus {
    fn from(s: &str) -> Self {
        match s {
            "new" => SupportStatus::New,
            "read" => SupportStatus::Read,
            "replied" => SupportStatus::Replied,
            "closed" => SupportStatus::Closed,
            _ => SupportStatus::New,
        }
    }
}

/// Support message model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportMessage {
    pub id: i32,
    pub sender_name: String,
    pub sender_email: String,
    pub user_id: Option<i32>,  // NULL for anonymous users
    pub subject: String,
    pub body: String,
    pub status: SupportStatus,
    pub admin_reply: Option<String>,
    pub replied_at: Option<NaiveDateTime>,
    pub replied_by: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Support message response (for API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportMessageResponse {
    pub id: i32,
    pub sender_name: String,
    pub sender_email: String,
    pub user_id: Option<i32>,
    pub subject: String,
    pub body: String,
    pub status: String,
    pub admin_reply: Option<String>,
    pub replied_at: Option<String>,
    pub created_at: String,
}

/// Create support message request (public)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSupportMessageRequest {
    pub sender_name: String,
    pub sender_email: String,
    pub subject: String,
    pub body: String,
    // user_id is captured from session if logged in (not in request body)
}

/// Create support message with user_id (internal, from handlers)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSupportMessageWithUser {
    pub sender_name: String,
    pub sender_email: String,
    pub user_id: Option<i32>,
    pub subject: String,
    pub body: String,
}

/// Update support message request (admin)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSupportMessageRequest {
    pub status: Option<String>,
    pub admin_reply: Option<String>,
}

/// List messages query params
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessagesQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// List messages response
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessagesResponse {
    pub success: bool,
    pub message: String,
    pub messages: Vec<SupportMessageResponse>,
    pub total: i64,
}

/// Single message response
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SupportMessageResponse>,
}

/// Create message response
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<i32>,
}

/// Count by status response
#[derive(Debug, Serialize, Deserialize)]
pub struct CountByStatusResponse {
    pub success: bool,
    pub total: i64,
    pub new: i64,
    pub read: i64,
    pub replied: i64,
    pub closed: i64,
}

/// Combined request for list_messages (session + query params)
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessagesRequest {
    pub session_id: i32,
    pub session_verifier: String,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Combined request for update_message (session + update params)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageRequest {
    pub session_id: i32,
    pub session_verifier: String,
    pub status: Option<String>,
    pub admin_reply: Option<String>,
}

/// Combined request for delete_message (session only)
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMessageRequest {
    pub session_id: i32,
    pub session_verifier: String,
}

/// Combined request for get_message (session only)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMessageRequest {
    pub session_id: i32,
    pub session_verifier: String,
}

/// Combined request for list_user_messages (session only)
#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserMessagesRequest {
    pub session_id: i32,
    pub session_verifier: String,
}
