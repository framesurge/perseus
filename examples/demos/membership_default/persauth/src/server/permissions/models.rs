use serde::{Deserialize, Serialize};

/// Permission actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    // Post permissions
    CreatePosts,
    ReadPosts,
    UpdatePosts,
    DeletePosts,
    PublishPosts,

    // Category permissions
    CreateCategories,
    ReadCategories,
    UpdateCategories,
    DeleteCategories,

    // Tag permissions
    CreateTags,
    ReadTags,
    UpdateTags,
    DeleteTags,

    // User management
    ManageUsers,
    ManageRoles,

    // Media
    UploadMedia,
    DeleteMedia,

    // Settings
    ManageSettings,

    // Support & Dashboard
    ManageSupport,
    ViewDashboard,
}

impl Permission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::CreatePosts => "create_posts",
            Permission::ReadPosts => "read_posts",
            Permission::UpdatePosts => "update_posts",
            Permission::DeletePosts => "delete_posts",
            Permission::PublishPosts => "publish_posts",
            Permission::CreateCategories => "create_categories",
            Permission::ReadCategories => "read_categories",
            Permission::UpdateCategories => "update_categories",
            Permission::DeleteCategories => "delete_categories",
            Permission::CreateTags => "create_tags",
            Permission::ReadTags => "read_tags",
            Permission::UpdateTags => "update_tags",
            Permission::DeleteTags => "delete_tags",
            Permission::ManageUsers => "manage_users",
            Permission::ManageRoles => "manage_roles",
            Permission::UploadMedia => "upload_media",
            Permission::DeleteMedia => "delete_media",
            Permission::ManageSettings => "manage_settings",
            Permission::ManageSupport => "manage_support",
            Permission::ViewDashboard => "view_dashboard",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "create_posts" => Some(Permission::CreatePosts),
            "read_posts" => Some(Permission::ReadPosts),
            "update_posts" => Some(Permission::UpdatePosts),
            "delete_posts" => Some(Permission::DeletePosts),
            "publish_posts" => Some(Permission::PublishPosts),
            "create_categories" => Some(Permission::CreateCategories),
            "read_categories" => Some(Permission::ReadCategories),
            "update_categories" => Some(Permission::UpdateCategories),
            "delete_categories" => Some(Permission::DeleteCategories),
            "create_tags" => Some(Permission::CreateTags),
            "read_tags" => Some(Permission::ReadTags),
            "update_tags" => Some(Permission::UpdateTags),
            "delete_tags" => Some(Permission::DeleteTags),
            "manage_users" => Some(Permission::ManageUsers),
            "manage_roles" => Some(Permission::ManageRoles),
            "upload_media" => Some(Permission::UploadMedia),
            "delete_media" => Some(Permission::DeleteMedia),
            "manage_settings" => Some(Permission::ManageSettings),
            "manage_support" => Some(Permission::ManageSupport),
            "view_dashboard" => Some(Permission::ViewDashboard),
            _ => None,
        }
    }

    /// Get all permissions as strings
    pub fn all() -> Vec<&'static str> {
        vec![
            "create_posts", "read_posts", "update_posts", "delete_posts", "publish_posts",
            "create_categories", "read_categories", "update_categories", "delete_categories",
            "create_tags", "read_tags", "update_tags", "delete_tags",
            "manage_users", "manage_roles",
            "upload_media", "delete_media",
            "manage_settings",
            "manage_support", "view_dashboard",
        ]
    }
}

/// Role with associated permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_system: bool, // System roles can't be deleted
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Create a new role
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRole {
    pub name: String,
    pub description: String,
}

/// Update role
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRole {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Role with permissions list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleWithPermissions {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_system: bool,
    pub permissions: Vec<String>,
}

/// User role assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
    pub assigned_at: Option<chrono::NaiveDateTime>,
}

/// User with roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithRoles {
    pub user_id: i32,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

/// Response for role operations
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<RoleWithPermissions>,
}

/// Response for roles list
#[derive(Debug, Serialize, Deserialize)]
pub struct RolesListResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<RoleWithPermissions>,
}

/// Response for user permissions check
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionCheckResponse {
    pub success: bool,
    pub has_permission: bool,
    pub permissions: Vec<String>,
}

/// Assign role to user request
#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: i32,
    pub role_id: i32,
}

/// Set role permissions request
#[derive(Debug, Serialize, Deserialize)]
pub struct SetPermissionsRequest {
    pub permissions: Vec<String>,
}

/// Default roles
pub fn default_roles() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
    vec![
        (
            "admin",
            "Administrator with full access",
            Permission::all(),
        ),
        (
            "editor",
            "Can manage all content",
            vec![
                "create_posts", "read_posts", "update_posts", "delete_posts", "publish_posts",
                "create_categories", "read_categories", "update_categories", "delete_categories",
                "create_tags", "read_tags", "update_tags", "delete_tags",
                "upload_media", "delete_media",
                "manage_support", "view_dashboard",
            ],
        ),
        (
            "author",
            "Can create and manage own content",
            vec![
                "create_posts", "read_posts", "update_posts",
                "read_categories", "read_tags", "create_tags",
                "upload_media",
            ],
        ),
        (
            "subscriber",
            "Basic read access",
            vec!["read_posts", "read_categories", "read_tags"],
        ),
    ]
}
