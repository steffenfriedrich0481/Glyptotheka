use axum::{middleware, routing::get, Router};
use std::net::SocketAddr;
use std::path::PathBuf;
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

    // Initialize cache directory
    let cache_dir = std::env::var("CACHE_DIR").unwrap_or_else(|_| "cache".to_string());
    let cache_path = PathBuf::from(&cache_dir);
    std::fs::create_dir_all(&cache_path).expect("Failed to create cache directory");

    tracing::info!(cache_dir = %cache_dir, "Cache directory initialized");

    // Build application with routes and middleware
    let api_routes = api::routes::create_router(pool, cache_path);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(api_routes)
        .layer(middleware::from_fn(cors_middleware))
        .layer(middleware::from_fn(error_middleware));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!(address = %addr, "Starting HTTP server");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server ready to accept connections");
    axum::serve(listener, app).await.unwrap();
}
