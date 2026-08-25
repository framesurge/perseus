pub mod db;
pub mod handlers;
pub mod models;

// Re-export commonly used items explicitly
pub use db::initialize_default_roles;
pub use handlers::{
    list_roles, get_role, create_role, update_role, delete_role,
    set_role_permissions, assign_role, remove_role,
    get_user_permissions, check_permission, list_permissions,
};
pub use models::{
    Permission, Role, RoleWithPermissions, UserWithRoles,
    CreateRole, UpdateRole, RoleResponse, RolesListResponse,
    PermissionCheckResponse, AssignRoleRequest, SetPermissionsRequest,
    default_roles,
};
