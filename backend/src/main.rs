use axum::{
    middleware,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing_subscriber;

mod config;
mod models;
mod db;
mod services;
mod api;
mod utils;

use api::middleware::{cors::cors_middleware, error::error_middleware};
use db::connection::create_pool;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize database
    let db_path = std::env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "glyptotheka.db".to_string());
    
    let pool = create_pool(&db_path)
        .expect("Failed to create database pool");
    
    tracing::info!("Database initialized at {}", db_path);

    // Run migrations
    db::migrations::run_migrations(&pool)
        .expect("Failed to run migrations");
    
    tracing::info!("Migrations completed");

    // Initialize cache directory
    let cache_dir = std::env::var("CACHE_DIR")
        .unwrap_or_else(|_| "cache".to_string());
    let cache_path = PathBuf::from(&cache_dir);
    std::fs::create_dir_all(&cache_path)
        .expect("Failed to create cache directory");
    
    tracing::info!("Cache directory initialized at {}", cache_dir);

    // Build application with routes and middleware
    let api_routes = api::routes::create_router(pool, cache_path);
    
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(api_routes)
        .layer(middleware::from_fn(cors_middleware))
        .layer(middleware::from_fn(error_middleware));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

