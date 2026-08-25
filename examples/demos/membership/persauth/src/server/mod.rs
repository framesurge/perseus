pub mod auth;
pub mod categories;
pub mod configs;
pub mod errors;
pub mod mail;
pub mod permissions;
pub mod posts;
pub mod tags;
pub mod uploads;

pub use auth::AppState;
pub use configs::Config;

use axum::{
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
    resend_otp, reset_password,
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

    // Create application state
    let state = Arc::new(AppState {
        pool,
        config: config.clone(),
    });

    // Create auth API router
    let auth_router = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/confirm", post(confirm_otp))
        .route("/resend-otp", post(resend_otp))
        .route("/profile", post(get_profile))
        .route("/request-reset", post(request_password_reset))
        .route("/reset", post(reset_password))
        .with_state(state.clone());

    // Create posts API router
    let posts_router = Router::new()
        .route("/", get(posts::list_posts).post(posts::create_post))
        .route("/{id}", get(posts::get_post).patch(posts::update_post).delete(posts::delete_post))
        .route("/slug/{slug}", get(posts::get_post_by_slug))
        .with_state(state.clone());

    // Create categories API router
    let categories_router = Router::new()
        .route("/", get(categories::list_categories).post(categories::create_category))
        .route("/{id}", get(categories::get_category).patch(categories::update_category).delete(categories::delete_category))
        .route("/slug/{slug}", get(categories::get_category_by_slug))
        .with_state(state.clone());

    // Create tags API router
    let tags_router = Router::new()
        .route("/", get(tags::list_tags).post(tags::create_tag))
        .route("/{id}", get(tags::get_tag).patch(tags::update_tag).delete(tags::delete_tag))
        .route("/slug/{slug}", get(tags::get_tag_by_slug))
        .route("/post/{post_id}", get(tags::get_post_tags).put(tags::set_post_tags))
        .with_state(state.clone());

    // Create uploads API router
    let uploads_router = Router::new()
        .route("/", post(uploads::upload_image).get(uploads::list_images))
        .route("/{filename}", delete(uploads::delete_image));

    // Create permissions API router
    let permissions_router = Router::new()
        .route("/roles", get(permissions::list_roles).post(permissions::create_role))
        .route("/roles/{id}", get(permissions::get_role).patch(permissions::update_role).delete(permissions::delete_role))
        .route("/roles/{id}/permissions", put(permissions::set_role_permissions))
        .route("/user-roles", post(permissions::assign_role))
        .route("/user-roles/{user_id}/{role_id}", delete(permissions::remove_role))
        .route("/users/{user_id}/permissions", get(permissions::get_user_permissions))
        .route("/users/{user_id}/permissions/{permission}", get(permissions::check_permission))
        .route("/permissions", get(permissions::list_permissions))
        .with_state(state.clone());

    // Create main API router
    // Note: Posts, categories, and tags are under /api/ to avoid conflicts with Perseus template routes
    let api_router = Router::new()
        .route("/health", get(health_check))
        .nest("/auth", auth_router)
        .nest("/api/posts", posts_router)
        .nest("/api/categories", categories_router)
        .nest("/api/tags", tags_router)
        .nest("/api/uploads", uploads_router)
        .nest("/api/permissions", permissions_router)
        // Serve uploaded files statically
        .nest_service("/uploads", ServeDir::new("./dist/uploads"));

    // Get Perseus router and merge
    let perseus_router = perseus_axum::get_router(turbine, opts).await;
    let app = api_router.merge(perseus_router);

    // Start server
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    log::info!("Server running on http://{}", addr);
    log::info!("API endpoints available:");
    log::info!("  Auth:        /auth/*");
    log::info!("  Posts:       /posts/*");
    log::info!("  Categories:  /categories/*");
    log::info!("  Tags:        /tags/*");
    log::info!("  Uploads:     /api/uploads/*");
    log::info!("  Permissions: /api/permissions/*");
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

    log::info!("Database tables initialized successfully");

    // Note: Default roles initialization is disabled by default.
    // Uncomment the following lines to auto-create default roles:
    // if let Err(e) = permissions::initialize_default_roles(pool).await {
    //     log::warn!("Failed to initialize default roles: {}", e);
    // }

    Ok(())
}
