use axum::{
    middleware,
    routing::get,
    Router,
};
use std::net::SocketAddr;
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

    // Build application with middleware
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .layer(middleware::from_fn(cors_middleware))
        .layer(middleware::from_fn(error_middleware));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
