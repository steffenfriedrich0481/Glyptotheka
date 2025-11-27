use axum::{middleware, routing::get, Router};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Re-export library modules
use glyptotheka_backend::{config, db, models, services, utils};

mod api;

use api::middleware::{cors::cors_middleware, error::error_middleware};
use db::connection::create_pool;

#[tokio::main]
async fn main() {
    // Initialize structured logging with environment filter
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,glyptotheka_backend=debug")),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    tracing::info!("Starting Glyptotheka 3D Print Library backend");

    // Initialize database
    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "glyptotheka.db".to_string());

    let pool = create_pool(&db_path).expect("Failed to create database pool");

    tracing::info!(database_path = %db_path, "Database connection pool created");

    // Run migrations
    db::migrations::run_migrations(&pool).expect("Failed to run migrations");

    tracing::info!("Database migrations completed successfully");

    // Initialize root_path from environment variable if not already set
    if let Ok(root_path) = std::env::var("ROOT_PATH") {
        let config_service = config::ConfigService::new(pool.clone());
        if let Ok(config) = config_service.get_config() {
            if config.root_path.is_none() {
                tracing::info!(root_path = %root_path, "Initializing root_path from environment");
                let update = config::UpdateConfigRequest {
                    root_path: Some(root_path.clone()),
                    cache_max_size_mb: None,
                    images_per_page: None,
                };
                config_service
                    .update_config(&update)
                    .expect("Failed to initialize root_path");
            }
        }
    }

    // Initialize cache directory
    let cache_dir = std::env::var("CACHE_DIR").unwrap_or_else(|_| "cache".to_string());
    let cache_path = PathBuf::from(&cache_dir);
    std::fs::create_dir_all(&cache_path).expect("Failed to create cache directory");

    tracing::info!(cache_dir = %cache_dir, "Cache directory initialized");

    // Initialize ignored keywords for search
    let ignored_keywords_str = std::env::var("IGNORED_KEYWORDS")
        .unwrap_or_else(|_| "PRESUPPORTED_STL,STL,UNSUPPORTED_STL,Unsupported".to_string());

    let ignored_keywords: Vec<String> = ignored_keywords_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    tracing::info!(keywords = ?ignored_keywords, "Initialized ignored keywords for search");

    // Build application with routes and middleware
    let api_routes = api::routes::create_router(pool, cache_path, ignored_keywords);

    let frontend_path = std::env::var("FRONTEND_PATH").unwrap_or_else(|_| "frontend".to_string());
    let serve_dir = ServeDir::new(&frontend_path)
        .not_found_service(ServeFile::new(format!("{}/index.html", frontend_path)));

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(api_routes)
        .fallback_service(serve_dir)
        .layer(middleware::from_fn(cors_middleware))
        .layer(middleware::from_fn(error_middleware));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!(address = %addr, "Starting HTTP server");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server ready to accept connections");
    axum::serve(listener, app).await.unwrap();
}
