use super::models::*;
use crate::server::AppState;
use chrono::NaiveDateTime;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

/// Create a new support message
pub async fn create_message_db(
    pool: &Pool,
    data: CreateSupportMessageWithUser,
) -> Result<i32, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row: Row = client
        .query_one(
            r#"
            INSERT INTO support_messages (sender_name, sender_email, user_id, subject, body)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            &[&data.sender_name, &data.sender_email, &data.user_id, &data.subject, &data.body],
        )
        .await?;
    Ok(row.get(0))
}

/// List all messages (admin)
pub async fn list_messages_db(
    pool: &Pool,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<SupportMessage>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let rows: Vec<Row> = if let Some(status) = status {
        client
            .query(
                r#"
                SELECT id, sender_name, sender_email, user_id, subject, body,
                       status, admin_reply, replied_at, replied_by,
                       created_at, updated_at
                FROM support_messages
                WHERE status = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                &[&status, &limit, &offset],
            )
            .await?
    } else {
        client
            .query(
                r#"
                SELECT id, sender_name, sender_email, user_id, subject, body,
                       status, admin_reply, replied_at, replied_by,
                       created_at, updated_at
                FROM support_messages
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                &[&limit, &offset],
            )
            .await?
    };
    Ok(rows.into_iter().map(|row| support_message_from_row(&row)).collect())
}

/// List messages for a specific user
pub async fn list_user_messages_db(
    pool: &Pool,
    user_id: i32,
) -> Result<Vec<SupportMessage>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let rows: Vec<Row> = client
        .query(
            r#"
            SELECT id, sender_name, sender_email, user_id, subject, body,
                   status, admin_reply, replied_at, replied_by,
                   created_at, updated_at
            FROM support_messages
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            &[&user_id],
        )
        .await?;
    Ok(rows.into_iter().map(|row| support_message_from_row(&row)).collect())
}

/// Get a single message by ID
pub async fn get_message_by_id_db(
    pool: &Pool,
    id: i32,
) -> Result<Option<SupportMessage>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client
        .query_one(
            r#"
            SELECT id, sender_name, sender_email, user_id, subject, body,
                   status, admin_reply, replied_at, replied_by,
                   created_at, updated_at
            FROM support_messages
            WHERE id = $1
            "#,
            &[&id],
        )
        .await
        .ok();
    Ok(row.map(|r| support_message_from_row(&r)))
}

/// Update message status
pub async fn update_message_status_db(
    pool: &Pool,
    id: i32,
    status: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let result = client
        .execute(
            r#"
            UPDATE support_messages
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            &[&status, &id],
        )
        .await?;
    Ok(result > 0)
}

/// Update message with admin reply
pub async fn update_message_reply_db(
    pool: &Pool,
    id: i32,
    reply: &str,
    replied_by: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let result = client
        .execute(
            r#"
            UPDATE support_messages
            SET admin_reply = $1,
                replied_at = NOW(),
                replied_by = $2,
                status = 'replied',
                updated_at = NOW()
            WHERE id = $3
            "#,
            &[&reply, &replied_by, &id],
        )
        .await?;
    Ok(result > 0)
}

/// Delete a message
pub async fn delete_message_db(
    pool: &Pool,
    id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let result = client
        .execute(
            "DELETE FROM support_messages WHERE id = $1",
            &[&id],
        )
        .await?;
    Ok(result > 0)
}

/// Count messages by status
pub async fn count_by_status_db(
    pool: &Pool,
) -> Result<(i64, i64, i64, i64, i64), Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let total: i64 = client
        .query_one("SELECT COUNT(*) FROM support_messages", &[])
        .await?
        .get(0);
    let new: i64 = client
        .query_one("SELECT COUNT(*) FROM support_messages WHERE status = 'new'", &[])
        .await?
        .get(0);
    let read: i64 = client
        .query_one("SELECT COUNT(*) FROM support_messages WHERE status = 'read'", &[])
        .await?
        .get(0);
    let replied: i64 = client
        .query_one("SELECT COUNT(*) FROM support_messages WHERE status = 'replied'", &[])
        .await?
        .get(0);
    let closed: i64 = client
        .query_one("SELECT COUNT(*) FROM support_messages WHERE status = 'closed'", &[])
        .await?
        .get(0);
    Ok((total, new, read, replied, closed))
}

/// Helper to convert row to SupportMessage
fn support_message_from_row(row: &Row) -> SupportMessage {
    SupportMessage {
        id: row.get("id"),
        sender_name: row.get("sender_name"),
        sender_email: row.get("sender_email"),
        user_id: row.get("user_id"),
        subject: row.get("subject"),
        body: row.get("body"),
        status: SupportStatus::from(row.get::<_, &str>("status")),
        admin_reply: row.get("admin_reply"),
        replied_at: row.get("replied_at"),
        replied_by: row.get("replied_by"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

/// Helper to convert SupportMessage to Response format
impl SupportMessage {
    pub fn to_response(&self) -> SupportMessageResponse {
        SupportMessageResponse {
            id: self.id,
            sender_name: self.sender_name.clone(),
            sender_email: self.sender_email.clone(),
            user_id: self.user_id,
            subject: self.subject.clone(),
            body: self.body.clone(),
            status: self.status.to_string(),
            admin_reply: self.admin_reply.clone(),
            replied_at: self.replied_at.map(|dt| dt.to_string()),
            created_at: self.created_at.to_string(),
        }
    }
}
