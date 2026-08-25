//! Casbin RBAC Model Definition
//!
//! This module defines the RBAC (Role-Based Access Control) model used by casbin.
//! The model uses a (sub, obj, act) pattern where:
//! - sub (subject): role name or user ID
//! - obj (object): resource being accessed (e.g., "posts", "categories")
//! - act (action): action being performed (e.g., "create", "read", "update", "delete")

/// The casbin RBAC model definition string
pub const CASBIN_RBAC_MODEL: &str = r#"
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act
"#;

/// Maps application permissions to casbin (obj, act) tuples
pub fn permission_to_resource(permission: &str) -> (&str, &str) {
    match permission {
        // Posts permissions
        "create_posts" => ("posts", "create"),
        "read_posts" => ("posts", "read"),
        "update_posts" => ("posts", "update"),
        "delete_posts" => ("posts", "delete"),
        "publish_posts" => ("posts", "publish"),
        // Categories permissions
        "create_categories" => ("categories", "create"),
        "read_categories" => ("categories", "read"),
        "update_categories" => ("categories", "update"),
        "delete_categories" => ("categories", "delete"),
        // Tags permissions
        "create_tags" => ("tags", "create"),
        "read_tags" => ("tags", "read"),
        "update_tags" => ("tags", "update"),
        "delete_tags" => ("tags", "delete"),
        // User/Role management
        "manage_users" => ("users", "manage"),
        "manage_roles" => ("roles", "manage"),
        // Media/Upload permissions
        "upload_media" => ("media", "upload"),
        "delete_media" => ("media", "delete"),
        // Settings
        "manage_settings" => ("settings", "manage"),
        // Fallback
        _ => ("unknown", "unknown"),
    }
}

/// Default role permissions mapping
pub const DEFAULT_ROLE_POLICIES: &[(&str, &[&str])] = &[
    // Admin - full access to everything
    ("admin", &[
        "posts:create", "posts:read", "posts:update", "posts:delete", "posts:publish",
        "categories:create", "categories:read", "categories:update", "categories:delete",
        "tags:create", "tags:read", "tags:update", "tags:delete",
        "users:manage", "roles:manage",
        "media:upload", "media:delete",
        "settings:manage",
    ]),
    // Editor - manage all content
    ("editor", &[
        "posts:create", "posts:read", "posts:update", "posts:delete", "posts:publish",
        "categories:create", "categories:read", "categories:update", "categories:delete",
        "tags:create", "tags:read", "tags:update", "tags:delete",
        "media:upload", "media:delete",
    ]),
    // Author - manage own posts and media
    ("author", &[
        "posts:create", "posts:read", "posts:update", "posts:delete",
        "tags:read",
        "media:upload",
    ]),
    // Subscriber - read-only access
    ("subscriber", &[
        "posts:read",
        "categories:read",
        "tags:read",
    ]),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_mapping() {
        assert_eq!(permission_to_resource("create_posts"), ("posts", "create"));
        assert_eq!(permission_to_resource("read_posts"), ("posts", "read"));
        assert_eq!(permission_to_resource("manage_users"), ("users", "manage"));
        assert_eq!(permission_to_resource("upload_media"), ("media", "upload"));
    }

    #[test]
    fn test_default_role_policies_exist() {
        assert!(DEFAULT_ROLE_POLICIES.iter().any(|(name, _)| *name == "admin"));
        assert!(DEFAULT_ROLE_POLICIES.iter().any(|(name, _)| *name == "editor"));
        assert!(DEFAULT_ROLE_POLICIES.iter().any(|(name, _)| *name == "author"));
        assert!(DEFAULT_ROLE_POLICIES.iter().any(|(name, _)| *name == "subscriber"));
    }
}
