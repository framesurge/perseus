use super::models::*;
use crate::server::AppState;
use chrono::NaiveDateTime;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

/// Get dashboard statistics
pub async fn get_dashboard_stats(
    pool: &Pool,
) -> Result<DashboardStats, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let total_users: i64 = client
        .query_one("SELECT COUNT(*) FROM users", &[])
        .await?
        .get(0);

    let total_posts: i64 = client
        .query_one("SELECT COUNT(*) FROM posts", &[])
        .await?
        .get(0);

    let published_posts: i64 = client
        .query_one("SELECT COUNT(*) FROM posts WHERE is_published = TRUE", &[])
        .await?
        .get(0);

    let total_categories: i64 = client
        .query_one("SELECT COUNT(*) FROM categories", &[])
        .await?
        .get(0);

    let total_tags: i64 = client
        .query_one("SELECT COUNT(*) FROM tags", &[])
        .await?
        .get(0);

    let total_support_messages: i64 = client
        .query_one("SELECT COUNT(*) FROM support_messages", &[])
        .await?
        .get(0);

    let unread_support_messages: i64 = client
        .query_one("SELECT COUNT(*) FROM support_messages WHERE status = 'new'", &[])
        .await?
        .get(0);

    // Get recent users (last 5)
    let recent_users_rows: Vec<Row> = client
        .query(
            "SELECT user_id, email, created_at FROM users ORDER BY created_at DESC LIMIT 5",
            &[],
        )
        .await?;

    let recent_users: Vec<RecentUser> = recent_users_rows
        .into_iter()
        .map(|row| RecentUser {
            user_id: row.get("user_id"),
            email: row.get("email"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok(DashboardStats {
        total_users,
        total_posts,
        published_posts,
        total_categories,
        total_tags,
        total_support_messages,
        unread_support_messages,
        recent_users,
    })
}

/// List users for admin
pub async fn list_users_db(
    pool: &Pool,
    search: Option<&str>,
    role: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<(Vec<AdminUserView>, i64), Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    // Keep queries parameterized. Do not build SQL by concatenating user input.
    let search_pattern = search.map(|s| format!("{}%", s));

    let total: i64 = client
        .query_one(
            r#"
            SELECT COUNT(*) FROM users u
            WHERE ($1::text IS NULL OR u.email ILIKE $1)
              AND ($2::text IS NULL OR EXISTS (
                    SELECT 1
                    FROM user_roles ur
                    JOIN roles r ON r.id = ur.role_id
                    WHERE ur.user_id = u.user_id AND r.name = $2
              ))
            "#,
            &[&search_pattern, &role],
        )
        .await?
        .get(0);

    let rows: Vec<Row> = client
        .query(
            r#"
            SELECT
                u.user_id,
                u.email,
                u.created_at,
                COALESCE(bool_or(s.otp_code_confirmed), false) AS otp_confirmed,
                COALESCE(
                    array_agg(DISTINCT r.name) FILTER (WHERE r.name IS NOT NULL),
                    ARRAY[]::text[]
                ) AS roles
            FROM users u
            LEFT JOIN sessions s ON s.user_id = u.user_id
            LEFT JOIN user_roles ur ON ur.user_id = u.user_id
            LEFT JOIN roles r ON r.id = ur.role_id
            WHERE ($1::text IS NULL OR u.email ILIKE $1)
              AND ($2::text IS NULL OR EXISTS (
                    SELECT 1
                    FROM user_roles ur2
                    JOIN roles r2 ON r2.id = ur2.role_id
                    WHERE ur2.user_id = u.user_id AND r2.name = $2
              ))
            GROUP BY u.user_id, u.email, u.created_at
            ORDER BY u.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            &[&search_pattern, &role, &limit, &offset],
        )
        .await?;

    let users: Vec<AdminUserView> = rows
        .into_iter()
        .map(|row| {
            let created_at: chrono::NaiveDateTime = row.get("created_at");
            AdminUserView {
                user_id: row.get("user_id"),
                email: row.get("email"),
                roles: row.get("roles"),
                otp_confirmed: row.get("otp_confirmed"),
                created_at: created_at.to_string(),
            }
        })
        .collect();

    Ok((users, total))
}

/// Get user details with roles
pub async fn get_user_detail(
    pool: &Pool,
    user_id: i32,
) -> Result<Option<AdminUserView>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let row = client
        .query_one(
            "SELECT user_id, email, created_at FROM users WHERE user_id = $1",
            &[&user_id],
        )
        .await
        .ok();

    match row {
        Some(row) => {
            let user_id: i32 = row.get("user_id");
            let email: String = row.get("email");
            let created_at: NaiveDateTime = row.get("created_at");

            // Get roles for this user
            let roles_rows: Vec<Row> = client
                .query(
                    "SELECT r.name FROM roles r
                     JOIN user_roles ur ON r.id = ur.role_id
                     WHERE ur.user_id = $1",
                    &[&user_id],
                )
                .await?;

            let roles: Vec<String> = roles_rows
                .into_iter()
                .map(|r| r.get("name"))
                .collect();

            // Get OTP status
            let otp_confirmed: bool = client
                .query_one(
                    "SELECT bool_or(otp_code_confirmed) FROM sessions WHERE user_id = $1",
                    &[&user_id],
                )
                .await
                .map(|r| r.get(0))
                .unwrap_or(false);

            Ok(Some(AdminUserView {
                user_id,
                email,
                roles,
                otp_confirmed,
                created_at: created_at.to_string(),
            }))
        }
        None => Ok(None),
    }
}

/// Count total users
pub async fn count_users(pool: &Pool) -> Result<i64, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let count: i64 = client
        .query_one("SELECT COUNT(*) FROM users", &[])
        .await?
        .get(0);
    Ok(count)
}

/// Count posts
pub async fn count_posts(pool: &Pool) -> Result<i64, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let count: i64 = client
        .query_one("SELECT COUNT(*) FROM posts", &[])
        .await?
        .get(0);
    Ok(count)
}

/// Count categories
pub async fn count_categories(pool: &Pool) -> Result<i64, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let count: i64 = client
        .query_one("SELECT COUNT(*) FROM categories", &[])
        .await?
        .get(0);
    Ok(count)
}

/// Count tags
pub async fn count_tags(pool: &Pool) -> Result<i64, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let count: i64 = client
        .query_one("SELECT COUNT(*) FROM tags", &[])
        .await?
        .get(0);
    Ok(count)
}
