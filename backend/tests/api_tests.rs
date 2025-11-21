use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware,
};
use glyptotheka_backend::config::Config;
use glyptotheka_backend::db::connection::create_pool;
use glyptotheka_backend::api::middleware::cors::cors_middleware;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use tower::util::ServiceExt;

async fn setup_test_app() -> (axum::Router, TempDir, Config) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let cache_dir = temp_dir.path().join("cache");

    let config = Config {
        database_path: db_path.to_str().unwrap().to_string(),
        cache_dir: cache_dir.to_str().unwrap().to_string(),
    };

    fs::create_dir_all(&cache_dir).unwrap();
    fs::create_dir_all(temp_dir.path().join("projects")).unwrap();

    let pool = create_pool(&config.database_path).unwrap();
    
    // Run migrations
    glyptotheka_backend::db::migrations::run_migrations(&pool).unwrap();
    
    let app = glyptotheka_backend::api::routes::create_router(pool, cache_dir)
        .layer(middleware::from_fn(cors_middleware));

    (app, temp_dir, config)
}

#[tokio::test]
async fn test_get_config() {
    let (app, _temp_dir, _config) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/config")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let _json: Value = serde_json::from_slice(&body).unwrap();

    // assert_eq!(json["rootPath"], config.root_path.unwrap());
}

#[tokio::test]
async fn test_update_config() {
    let (app, temp_dir, _config) = setup_test_app().await;

    let new_root = temp_dir.path().join("new_root");
    fs::create_dir_all(&new_root).unwrap();

    let request_body = serde_json::json!({
        "rootPath": new_root.to_str().unwrap()
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/config")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_root_projects() {
    let (app, _temp_dir, _config) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/projects")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["projects"].is_array());
}

#[tokio::test]
async fn test_search_projects_empty_query() {
    let (app, _temp_dir, _config) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/search?q=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_search_projects_valid_query() {
    let (app, _temp_dir, _config) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/search?q=test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["data"].is_array());
}

#[tokio::test]
async fn test_get_nonexistent_project() {
    let (app, _temp_dir, _config) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/projects/99999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_tags() {
    let (app, _temp_dir, _config) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/tags")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["data"].is_array());
}

#[tokio::test]
async fn test_cors_headers() {
    let (app, _temp_dir, _config) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/api/config")
                .header("origin", "http://localhost:5173")
                .header("access-control-request-method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::NO_CONTENT);
}
