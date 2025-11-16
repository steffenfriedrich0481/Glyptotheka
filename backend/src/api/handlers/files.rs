use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
};
use crate::api::routes::AppState;
use crate::utils::error::AppError;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn serve_image(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Response, AppError> {
    let cache_path = state.image_cache_service
        .get_image_by_hash(&hash)?
        .ok_or_else(|| AppError::NotFound(format!("Image not found: {}", hash)))?;

    let file = File::open(&cache_path).await?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_type = match cache_path.extension().and_then(|e| e.to_str()) {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(body)
        .unwrap())
}

