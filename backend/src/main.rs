use axum::{
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(|| async { "OK" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
