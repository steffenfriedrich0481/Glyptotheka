use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use tracing::error;

pub async fn error_middleware(request: Request<axum::body::Body>, next: Next) -> Response {
    let uri = request.uri().clone();
    let method = request.method().clone();

    let response = next.run(request).await;

    if response.status().is_server_error() {
        error!(
            method = %method,
            uri = %uri,
            status = %response.status(),
            "Request resulted in server error"
        );
    }

    response
}

#[allow(dead_code)]
pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
