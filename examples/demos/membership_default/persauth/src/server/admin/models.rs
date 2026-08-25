use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// Admin user view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserView {
    pub user_id: i32,
    pub email: String,
    pub roles: Vec<String>,
    pub otp_confirmed: bool,
    pub created_at: String,
}

/// Dashboard statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_posts: i64,
    pub published_posts: i64,
    pub total_categories: i64,
    pub total_tags: i64,
    pub total_support_messages: i64,
    pub unread_support_messages: i64,
    pub recent_users: Vec<RecentUser>,
}

/// Recent user for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentUser {
    pub user_id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
}

/// User list query
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListQuery {
    pub search: Option<String>,
    pub role: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// User list response
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    pub success: bool,
    pub message: String,
    pub users: Vec<AdminUserView>,
    pub total: i64,
}

/// Dashboard stats response
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStatsResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DashboardStats>,
}

/// Authenticated request (for handlers)
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedRequest {
    pub session_id: String,
}

/// User query request with session
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUserQueryRequest {
    pub session_id: String,
    pub search: Option<String>,
    pub role: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
