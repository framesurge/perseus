use deadpool_postgres::Pool;
use crate::server::errors::ServiceError;
use super::models::*;

/// Create a new role
pub async fn create_role(
    pool: &Pool,
    name: &str,
    description: &str,
    is_system: bool,
) -> Result<Role, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let row = client
        .query_one(
            r#"
            INSERT INTO roles (name, description, is_system)
            VALUES ($1, $2, $3)
            RETURNING id, name, description, is_system, created_at
            "#,
            &[&name, &description, &is_system],
        )
        .await?;

    Ok(Role {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        is_system: row.get("is_system"),
        created_at: row.get("created_at"),
    })
}

/// Get role by ID
pub async fn get_role(pool: &Pool, role_id: i32) -> Result<Option<Role>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let row = client
        .query_opt(
            "SELECT id, name, description, is_system, created_at FROM roles WHERE id = $1",
            &[&role_id],
        )
        .await?;

    Ok(row.map(|r| Role {
        id: r.get("id"),
        name: r.get("name"),
        description: r.get("description"),
        is_system: r.get("is_system"),
        created_at: r.get("created_at"),
    }))
}

/// Get role by name
pub async fn get_role_by_name(pool: &Pool, name: &str) -> Result<Option<Role>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let row = client
        .query_opt(
            "SELECT id, name, description, is_system, created_at FROM roles WHERE name = $1",
            &[&name],
        )
        .await?;

    Ok(row.map(|r| Role {
        id: r.get("id"),
        name: r.get("name"),
        description: r.get("description"),
        is_system: r.get("is_system"),
        created_at: r.get("created_at"),
    }))
}

/// List all roles with their permissions
pub async fn list_roles(pool: &Pool) -> Result<Vec<RoleWithPermissions>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT r.id, r.name, r.description, r.is_system,
                   COALESCE(array_agg(rp.permission) FILTER (WHERE rp.permission IS NOT NULL), '{}') as permissions
            FROM roles r
            LEFT JOIN role_permissions rp ON r.id = rp.role_id
            GROUP BY r.id, r.name, r.description, r.is_system
            ORDER BY r.name
            "#,
            &[],
        )
        .await?;

    Ok(rows
        .iter()
        .map(|r| {
            let permissions: Vec<String> = r.get("permissions");
            RoleWithPermissions {
                id: r.get("id"),
                name: r.get("name"),
                description: r.get("description"),
                is_system: r.get("is_system"),
                permissions,
            }
        })
        .collect())
}

/// Get role with permissions
pub async fn get_role_with_permissions(
    pool: &Pool,
    role_id: i32,
) -> Result<Option<RoleWithPermissions>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT r.id, r.name, r.description, r.is_system,
                   COALESCE(array_agg(rp.permission) FILTER (WHERE rp.permission IS NOT NULL), '{}') as permissions
            FROM roles r
            LEFT JOIN role_permissions rp ON r.id = rp.role_id
            WHERE r.id = $1
            GROUP BY r.id, r.name, r.description, r.is_system
            "#,
            &[&role_id],
        )
        .await?;

    Ok(row.map(|r| {
        let permissions: Vec<String> = r.get("permissions");
        RoleWithPermissions {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            is_system: r.get("is_system"),
            permissions,
        }
    }))
}

/// Update role
pub async fn update_role(
    pool: &Pool,
    role_id: i32,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<Option<Role>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    // Check if it's a system role
    let existing = get_role(pool, role_id).await?;
    if let Some(ref role) = existing {
        if role.is_system && name.is_some() {
            return Err("Cannot rename system roles".into());
        }
    }

    let row = client
        .query_opt(
            r#"
            UPDATE roles
            SET name = COALESCE($2, name),
                description = COALESCE($3, description)
            WHERE id = $1
            RETURNING id, name, description, is_system, created_at
            "#,
            &[&role_id, &name, &description],
        )
        .await?;

    Ok(row.map(|r| Role {
        id: r.get("id"),
        name: r.get("name"),
        description: r.get("description"),
        is_system: r.get("is_system"),
        created_at: r.get("created_at"),
    }))
}

/// Delete role (only non-system roles)
pub async fn delete_role(pool: &Pool, role_id: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let result = client
        .execute(
            "DELETE FROM roles WHERE id = $1 AND is_system = false",
            &[&role_id],
        )
        .await?;

    Ok(result > 0)
}

/// Set permissions for a role (replace all)
pub async fn set_role_permissions(
    pool: &Pool,
    role_id: i32,
    permissions: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    // Remove existing permissions
    transaction
        .execute("DELETE FROM role_permissions WHERE role_id = $1", &[&role_id])
        .await?;

    // Add new permissions
    for permission in permissions {
        transaction
            .execute(
                "INSERT INTO role_permissions (role_id, permission) VALUES ($1, $2)",
                &[&role_id, &permission],
            )
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}

/// Assign role to user
pub async fn assign_role_to_user(
    pool: &Pool,
    user_id: i32,
    role_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    client
        .execute(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#,
            &[&user_id, &role_id],
        )
        .await?;

    Ok(())
}

/// Remove role from user
pub async fn remove_role_from_user(
    pool: &Pool,
    user_id: i32,
    role_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let result = client
        .execute(
            "DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2",
            &[&user_id, &role_id],
        )
        .await?;

    Ok(result > 0)
}

/// Get user's roles
pub async fn get_user_roles(pool: &Pool, user_id: i32) -> Result<Vec<String>, ServiceError> {
    let client = pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT r.name
            FROM roles r
            JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            ORDER BY r.name
            "#,
            &[&user_id],
        )
        .await?;

    Ok(rows.iter().map(|r| r.get("name")).collect())
}

/// Get user's permissions (combined from all roles)
pub async fn get_user_permissions(pool: &Pool, user_id: i32) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT DISTINCT rp.permission
            FROM role_permissions rp
            JOIN user_roles ur ON rp.role_id = ur.role_id
            WHERE ur.user_id = $1
            ORDER BY rp.permission
            "#,
            &[&user_id],
        )
        .await?;

    Ok(rows.iter().map(|r| r.get("permission")).collect())
}

/// Check if user has a specific permission
pub async fn user_has_permission(
    pool: &Pool,
    user_id: i32,
    permission: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT 1
            FROM role_permissions rp
            JOIN user_roles ur ON rp.role_id = ur.role_id
            WHERE ur.user_id = $1 AND rp.permission = $2
            LIMIT 1
            "#,
            &[&user_id, &permission],
        )
        .await?;

    Ok(row.is_some())
}

/// Check if user has any of the specified permissions
pub async fn user_has_any_permission(
    pool: &Pool,
    user_id: i32,
    permissions: &[&str],
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT 1
            FROM role_permissions rp
            JOIN user_roles ur ON rp.role_id = ur.role_id
            WHERE ur.user_id = $1 AND rp.permission = ANY($2)
            LIMIT 1
            "#,
            &[&user_id, &permissions],
        )
        .await?;

    Ok(row.is_some())
}

/// Get user with roles and permissions
pub async fn get_user_with_permissions(
    pool: &Pool,
    user_id: i32,
) -> Result<Option<UserWithRoles>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    // Get user email
    let user_row = client
        .query_opt(
            "SELECT user_id, email FROM users WHERE user_id = $1",
            &[&user_id],
        )
        .await?;

    if let Some(user) = user_row {
        let roles = get_user_roles(pool, user_id).await?;
        let permissions = get_user_permissions(pool, user_id).await?;

        Ok(Some(UserWithRoles {
            user_id: user.get("user_id"),
            email: user.get("email"),
            roles,
            permissions,
        }))
    } else {
        Ok(None)
    }
}

/// Initialize default roles if they don't exist
pub async fn initialize_default_roles(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    for (name, description, permissions) in default_roles() {
        // Check if role exists
        if get_role_by_name(pool, name).await?.is_none() {
            // Create role
            let role = create_role(pool, name, description, true).await?;

            // Set permissions
            let perms: Vec<String> = permissions.iter().map(|s| s.to_string()).collect();
            set_role_permissions(pool, role.id, &perms).await?;

            log::info!("Created default role: {} with {} permissions", name, permissions.len());
        }
    }

    Ok(())
}

/// Assign default role to new user
pub async fn assign_default_role(pool: &Pool, user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(role) = get_role_by_name(pool, "subscriber").await? {
        assign_role_to_user(pool, user_id, role.id).await?;
    }
    Ok(())
}
