pub mod auth;
pub mod admin;
pub mod categories;
pub mod configs;
pub mod errors;
pub mod mail;
pub mod middleware;
pub mod permissions;
pub mod posts;
pub mod support;
pub mod tags;
pub mod uploads;
pub mod casbin_model;

pub use auth::AppState;
pub use configs::Config;
// Re-export shared types for server modules (import from crate root)
pub use crate::types::{ApiResponse, SessionData};

use axum::{
    body::Body,
    routing::{delete, get, patch, post, put},
    Router,
};
use deadpool_postgres::Runtime;
use perseus::i18n::TranslationsManager;
use perseus::server::ServerOptions;
use perseus::turbine::Turbine;
use std::sync::Arc;
use tower_http::services::ServeDir;

use auth::{
    confirm_otp, get_profile, login, logout, register_user, request_password_reset,
    resend_otp, reset_password, update_preferences,
};

async fn health_check() -> &'static str {
    "OK"
}

pub async fn custom_server<M, T>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) where
    M: perseus::stores::MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    // Initialize environment
    dotenv::dotenv().ok();
    env_logger::init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    log::info!(
        "Starting server with config: {}:{}",
        config.srv_cnf.host,
        config.srv_cnf.port
    );

    // Create database pool
    let pool = config
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .expect("Failed to create database pool");

    // Initialize database tables
    if let Err(e) = init_database(&pool).await {
        log::error!("Failed to initialize database: {}", e);
    }

    // Create application state (casbin enforcer temporarily disabled due to async trait complexity)
    let state = Arc::new(AppState {
        pool,
        config: config.clone(),
    });

    // Create auth API router
    // Public routes: register, login, request-reset, reset
    // Protected routes: logout, confirm, resend-otp, profile (handlers check auth)
    let auth_router = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login))
        .route("/request-reset", post(request_password_reset))
        .route("/reset", post(reset_password))
        .route("/logout", post(logout))
        .route("/confirm", post(confirm_otp))
        .route("/resend-otp", post(resend_otp))
        .route("/profile", post(get_profile))
        .route("/preferences", post(update_preferences))
        .with_state(state.clone());

    // Create posts API router
    // Public: list_posts, get_post, get_post_by_slug
    // Protected: create_post, update_post, delete_post (handlers check auth)
    let posts_router = Router::new()
        // Public routes
        .route("/", get(posts::list_posts))
        .route("/{id}", get(posts::get_post))
        .route("/slug/{slug}", get(posts::get_post_by_slug))
        // Protected routes
        .route("/", post(posts::create_post))
        .route("/{id}", patch(posts::update_post))
        .route("/{id}", delete(posts::delete_post))
        .with_state(state.clone());

    // Create categories API router
    // Public: list, get, get_by_slug
    // Protected: create, update, delete (handlers check auth)
    let categories_router = Router::new()
        // Public routes
        .route("/", get(categories::list_categories))
        .route("/{id}", get(categories::get_category))
        .route("/slug/{slug}", get(categories::get_category_by_slug))
        // Protected routes
        .route("/", post(categories::create_category))
        .route("/{id}", patch(categories::update_category))
        .route("/{id}", delete(categories::delete_category))
        .with_state(state.clone());

    // Create tags API router
    // Public: list, get, get_by_slug, get_post_tags
    // Protected: create, update, delete, set_post_tags (handlers check auth)
    let tags_router = Router::new()
        // Public routes
        .route("/", get(tags::list_tags))
        .route("/{id}", get(tags::get_tag))
        .route("/slug/{slug}", get(tags::get_tag_by_slug))
        .route("/post/{post_id}", get(tags::get_post_tags))
        // Protected routes
        .route("/", post(tags::create_tag))
        .route("/{id}", patch(tags::update_tag))
        .route("/{id}", delete(tags::delete_tag))
        .route("/post/{post_id}", put(tags::set_post_tags))
        .with_state(state.clone());

    // Create uploads API router
    // All routes require auth (handlers check auth)
    let uploads_router = Router::new()
        .route("/", get(uploads::list_images))
        .route("/", post(uploads::upload_image))
        .route("/{filename}", delete(uploads::delete_image))
        .with_state(state.clone());

    // Create permissions API router - all require auth
    let permissions_router = Router::new()
        // All routes require authentication (handlers check auth)
        .route("/roles", get(permissions::list_roles))
        .route("/roles", post(permissions::create_role))
        .route("/roles/{id}", get(permissions::get_role))
        .route("/roles/{id}", patch(permissions::update_role))
        .route("/roles/{id}", delete(permissions::delete_role))
        .route("/roles/{id}/permissions", put(permissions::set_role_permissions))
        .route("/user-roles", post(permissions::assign_role))
        .route("/user-roles/{user_id}/{role_id}", delete(permissions::remove_role))
        .route("/users/{user_id}/permissions", get(permissions::get_user_permissions))
        .route("/users/{user_id}/permissions/{permission}", get(permissions::check_permission))
        .route("/permissions", get(permissions::list_permissions))
        .with_state(state.clone());

    // Create support API router
    // Public: create_message
    // Protected: list_messages, my-messages, get_message, update_message, delete_message
    let support_router = Router::new()
        // Public route - no auth required
        .route("/", post(support::create_message))
        // Protected routes
        .route("/list", post(support::list_messages))
        .route("/my-messages", post(support::list_user_messages))
        .route("/{id}", get(support::get_message))
        .route("/{id}", patch(support::update_message))
        .route("/{id}", delete(support::delete_message))
        .with_state(state.clone());

    // Create admin API router
    let admin_router = Router::new()
        // All routes require authentication and admin permissions
        .route("/stats", post(admin::dashboard_stats))
        .route("/users", post(admin::list_users))
        .route("/users/{user_id}", get(admin::get_user))
        // Support message management
        .route("/messages", post(support::list_messages))
        .route("/messages/{id}", get(support::get_message))
        .route("/messages/{id}/reply", patch(support::update_message))
        .route("/messages/{id}", delete(support::delete_message))
        .with_state(state.clone());

    // Create main API router
    // Note: All API routes are under /api/ to avoid conflicts with Perseus template routes
    let api_router = Router::new()
        .route("/health", get(health_check))
        .nest("/api/auth", auth_router)
        .nest("/api/posts", posts_router)
        .nest("/api/categories", categories_router)
        .nest("/api/tags", tags_router)
        .nest("/api/uploads", uploads_router)
        .nest("/api/permissions", permissions_router)
        .nest("/api/support", support_router)
        .nest("/api/admin", admin_router)
        // Serve uploaded files statically
        .nest_service("/uploads", ServeDir::new("./dist/uploads"));

    // Get Perseus router with API routes merged (API routes take precedence before fallback)
    let app = perseus_axum::get_router_with_api(turbine, opts, Some(api_router)).await;

    // Start server
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    log::info!("Server running on http://{}", addr);
    log::info!("API endpoints available:");
    log::info!("  Auth:        /api/auth/*");
    log::info!("  Posts:       /api/posts/*");
    log::info!("  Categories:  /api/categories/*");
    log::info!("  Tags:        /api/tags/*");
    log::info!("  Uploads:     /api/uploads/*");
    log::info!("  Permissions: /api/permissions/*");
    log::info!("  Support:     /api/support/*");
    log::info!("  Admin:       /api/admin/*");
    log::info!("  Static:      /uploads/* (uploaded files)");
    axum::serve(listener, app).await.unwrap();
}

async fn init_database(pool: &deadpool_postgres::Pool) -> Result<(), Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    // Create users table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                user_id SERIAL PRIMARY KEY,
                email VARCHAR(255) NOT NULL UNIQUE,
                hashed_password VARCHAR(255) NOT NULL,
                reset_password_selector VARCHAR(255),
                reset_password_sent_at TIMESTAMP,
                reset_password_validator_hash VARCHAR(255),
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
            &[],
        )
        .await?;

    // Create sessions table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                session_verifier VARCHAR(255) NOT NULL,
                otp_code_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
                otp_code_encrypted VARCHAR(255) NOT NULL,
                otp_code_attempts INTEGER NOT NULL DEFAULT 0,
                otp_code_sent BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
            &[],
        )
        .await?;

    // Create categories table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS categories (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                slug VARCHAR(255) NOT NULL UNIQUE,
                description TEXT NOT NULL DEFAULT '',
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
            &[],
        )
        .await?;

    // Create posts table with SEO fields
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS posts (
                id SERIAL PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                slug VARCHAR(255) NOT NULL UNIQUE,
                summary TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                meta_title VARCHAR(255),
                meta_description TEXT,
                meta_keywords TEXT,
                og_image VARCHAR(500),
                canonical_url VARCHAR(500),
                is_published BOOLEAN NOT NULL DEFAULT FALSE,
                published_at TIMESTAMP,
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
            &[],
        )
        .await?;

    // Add SEO columns to existing posts table if they don't exist
    // This handles migrations for existing databases
    let add_columns = vec![
        "ALTER TABLE posts ADD COLUMN IF NOT EXISTS meta_title VARCHAR(255)",
        "ALTER TABLE posts ADD COLUMN IF NOT EXISTS meta_description TEXT",
        "ALTER TABLE posts ADD COLUMN IF NOT EXISTS meta_keywords TEXT",
        "ALTER TABLE posts ADD COLUMN IF NOT EXISTS og_image VARCHAR(500)",
        "ALTER TABLE posts ADD COLUMN IF NOT EXISTS canonical_url VARCHAR(500)",
        "ALTER TABLE posts ADD COLUMN IF NOT EXISTS is_published BOOLEAN DEFAULT FALSE",
        "ALTER TABLE posts ADD COLUMN IF NOT EXISTS published_at TIMESTAMP",
        // Delta JSON content for sycawysgy editor (stores Quill Delta format)
        "ALTER TABLE posts ADD COLUMN IF NOT EXISTS content_delta JSONB",
    ];

    for sql in add_columns {
        if let Err(e) = client.execute(sql, &[]).await {
            log::debug!("Column might already exist: {}", e);
        }
    }

    // Create tags table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS tags (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                slug VARCHAR(255) NOT NULL UNIQUE,
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
            &[],
        )
        .await?;

    // Create posts_tags junction table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS posts_tags (
                id SERIAL PRIMARY KEY,
                post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
                tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                UNIQUE(post_id, tag_id)
            )
            "#,
            &[],
        )
        .await?;

    // Create indexes for better query performance
    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_posts_slug ON posts(slug)",
            &[],
        )
        .await?;

    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_categories_slug ON categories(slug)",
            &[],
        )
        .await?;

    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_tags_slug ON tags(slug)",
            &[],
        )
        .await?;

    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_posts_tags_post_id ON posts_tags(post_id)",
            &[],
        )
        .await?;

    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_posts_tags_tag_id ON posts_tags(tag_id)",
            &[],
        )
        .await?;

    // Create roles table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS roles (
                id SERIAL PRIMARY KEY,
                name VARCHAR(100) NOT NULL UNIQUE,
                description TEXT NOT NULL DEFAULT '',
                is_system BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
            &[],
        )
        .await?;

    // Create role_permissions table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS role_permissions (
                id SERIAL PRIMARY KEY,
                role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
                permission VARCHAR(100) NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                UNIQUE(role_id, permission)
            )
            "#,
            &[],
        )
        .await?;

    // Create user_roles table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS user_roles (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
                assigned_at TIMESTAMP NOT NULL DEFAULT NOW(),
                UNIQUE(user_id, role_id)
            )
            "#,
            &[],
        )
        .await?;

    // Create indexes for roles and permissions
    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_role_permissions_role_id ON role_permissions(role_id)",
            &[],
        )
        .await?;

    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_user_roles_user_id ON user_roles(user_id)",
            &[],
        )
        .await?;

    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_user_roles_role_id ON user_roles(role_id)",
            &[],
        )
        .await?;

    // Create support_messages table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS support_messages (
                id SERIAL PRIMARY KEY,
                sender_name VARCHAR(255) NOT NULL,
                sender_email VARCHAR(255) NOT NULL,
                user_id INTEGER REFERENCES users(user_id),
                subject VARCHAR(500) NOT NULL,
                body TEXT NOT NULL,
                status VARCHAR(50) NOT NULL DEFAULT 'new',
                admin_reply TEXT,
                replied_at TIMESTAMP,
                replied_by INTEGER REFERENCES users(user_id),
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
            &[],
        )
        .await?;

    // Create indexes for support_messages
    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_support_messages_status ON support_messages(status)",
            &[],
        )
        .await?;

    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_support_messages_user_id ON support_messages(user_id)",
            &[],
        )
        .await?;

    client
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_support_messages_created ON support_messages(created_at DESC)",
            &[],
        )
        .await?;

    log::info!("Database tables initialized successfully");

    // Note: Default roles initialization is disabled by default.
    // Uncomment the following lines to auto-create default roles:
    // if let Err(e) = permissions::initialize_default_roles(pool).await {
    //     log::warn!("Failed to initialize default roles: {}", e);
    // }

    Ok(())
}
