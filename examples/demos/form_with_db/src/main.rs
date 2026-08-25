use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

mod templates;

// Shared types (used by both client and server)
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// Server-only code
#[cfg(engine)]
mod server {
    use super::*;
    use axum::{
        extract::State,
        http::StatusCode,
        routing::{get, post},
        Json, Router,
    };
    use perseus::i18n::TranslationsManager;
    use perseus::server::ServerOptions;
    use perseus::turbine::Turbine;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
    use std::sync::Arc;

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        id: Option<i64>,
        name: String,
        email: String,
        created_at: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct CreateUserRequest {
        name: String,
        email: String,
    }

    // API Handlers
    async fn create_user(
        State(pool): State<Arc<SqlitePool>>,
        Json(payload): Json<CreateUserRequest>,
    ) -> Result<Json<ApiResponse>, (StatusCode, String)> {
        // Validate input
        if payload.name.is_empty() || payload.email.is_empty() {
            return Ok(Json(ApiResponse {
                success: false,
                message: "Name and email are required".to_string(),
                data: None,
            }));
        }

        // Check if email already exists
        let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE email = ?")
            .bind(&payload.email)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if existing.is_some() {
            return Ok(Json(ApiResponse {
                success: false,
                message: "Email already exists".to_string(),
                data: None,
            }));
        }

        // Insert user
        let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
            .bind(&payload.name)
            .bind(&payload.email)
            .execute(pool.as_ref())
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(ApiResponse {
            success: true,
            message: "User created successfully".to_string(),
            data: Some(serde_json::json!({ "id": result.last_insert_rowid() })),
        }))
    }

    async fn get_users(
        State(pool): State<Arc<SqlitePool>>,
    ) -> Result<Json<ApiResponse>, (StatusCode, String)> {
        let rows = sqlx::query_as::<_, (Option<i64>, String, String, Option<String>)>(
            "SELECT id, name, email, created_at FROM users ORDER BY created_at DESC",
        )
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let users: Vec<User> = rows
            .into_iter()
            .map(|(id, name, email, created_at)| User {
                id,
                name,
                email,
                created_at,
            })
            .collect();

        Ok(Json(ApiResponse {
            success: true,
            message: "Users retrieved successfully".to_string(),
            data: Some(serde_json::to_value(&users).unwrap()),
        }))
    }

    async fn delete_user(
        State(pool): State<Arc<SqlitePool>>,
        axum::extract::Path(id): axum::extract::Path<i64>,
    ) -> Result<Json<ApiResponse>, (StatusCode, String)> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(pool.as_ref())
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if result.rows_affected() == 0 {
            return Ok(Json(ApiResponse {
                success: false,
                message: "User not found".to_string(),
                data: None,
            }));
        }

        Ok(Json(ApiResponse {
            success: true,
            message: "User deleted successfully".to_string(),
            data: None,
        }))
    }

    pub async fn custom_server<M, T>(
        turbine: &'static Turbine<M, T>,
        opts: ServerOptions,
        (host, port): (String, u16),
    ) where
        M: perseus::stores::MutableStore + 'static,
        T: TranslationsManager + 'static,
    {
        // Initialize database
        let pool = init_database()
            .await
            .expect("Failed to initialize database");
        let pool = Arc::new(pool);

        // Create API router with database pool state
        let api_router = Router::new()
            .route("/api/users", get(get_users))
            .route("/api/users", post(create_user))
            .route("/api/users/{id}", axum::routing::delete(delete_user))
            .with_state(pool.clone());

        // Get Perseus router and merge
        let perseus_router = perseus_axum::get_router(turbine, opts).await;
        let app = api_router.merge(perseus_router);

        // Start server
        let addr = format!("{}:{}", host, port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        println!("Server running on http://{}", addr);
        axum::serve(listener, app).await.unwrap();
    }

    async fn init_database() -> Result<SqlitePool, sqlx::Error> {
        // Create database file if it doesn't exist
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:users.db?mode=rwc")
            .await?;

        // Create table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&pool)
        .await?;

        println!("Database initialized successfully");
        Ok(pool)
    }
}

#[cfg(not(target_arch = "wasm32"))]
use server::custom_server;

#[perseus::main(custom_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(templates::index::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
        .index_view(|| {
            view! {
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        title { "Perseus Form with Database" }
                        style {
                            r#"
                                * { margin: 0; padding: 0; box-sizing: border-box; }
                                body {
                                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                                    background: #f5f5f5;
                                    padding: 2rem;
                                }
                                .container { max-width: 800px; margin: 0 auto; }
                                h1 { color: #333; margin-bottom: 2rem; }
                                .form-card, .users-card {
                                    background: white;
                                    padding: 2rem;
                                    border-radius: 8px;
                                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                                    margin-bottom: 2rem;
                                }
                                .form-group { margin-bottom: 1.5rem; }
                                label {
                                    display: block;
                                    margin-bottom: 0.5rem;
                                    color: #555;
                                    font-weight: 500;
                                }
                                input {
                                    width: 100%;
                                    padding: 0.75rem;
                                    border: 1px solid #ddd;
                                    border-radius: 4px;
                                    font-size: 1rem;
                                }
                                input:focus {
                                    outline: none;
                                    border-color: #4CAF50;
                                }
                                button {
                                    background: #4CAF50;
                                    color: white;
                                    padding: 0.75rem 2rem;
                                    border: none;
                                    border-radius: 4px;
                                    font-size: 1rem;
                                    cursor: pointer;
                                    transition: background 0.2s;
                                }
                                button:hover:not(:disabled) {
                                    background: #45a049;
                                }
                                button:disabled {
                                    background: #ccc;
                                    cursor: not-allowed;
                                }
                                .message {
                                    padding: 1rem;
                                    border-radius: 4px;
                                    margin-top: 1rem;
                                }
                                .message.success {
                                    background: #d4edda;
                                    color: #155724;
                                    border: 1px solid #c3e6cb;
                                }
                                .message.error {
                                    background: #f8d7da;
                                    color: #721c24;
                                    border: 1px solid #f5c6cb;
                                }
                                .users-list { list-style: none; }
                                .user-item {
                                    padding: 1rem;
                                    border: 1px solid #eee;
                                    border-radius: 4px;
                                    margin-bottom: 0.5rem;
                                    display: flex;
                                    justify-content: space-between;
                                    align-items: center;
                                }
                                .user-info { flex: 1; }
                                .user-name { font-weight: 600; color: #333; }
                                .user-email { color: #666; font-size: 0.9rem; margin-top: 0.25rem; }
                                .delete-btn {
                                    background: #dc3545;
                                    padding: 0.5rem 1rem;
                                    font-size: 0.875rem;
                                }
                                .delete-btn:hover:not(:disabled) {
                                    background: #c82333;
                                }
                                .empty-state {
                                    text-align: center;
                                    color: #999;
                                    padding: 2rem;
                                }
                            "#
                        }
                    }
                    body {
                        PerseusRoot()
                    }
                }
            }
        })
}
