use axum::{
    body::Body,
    extract::{Path as AxumPath, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::path::PathBuf;
use crate::api::routes::AppState;
use crate::utils::error::AppError;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[derive(Debug, Deserialize)]
pub struct FileQueryParams {
    #[serde(rename = "type")]
    pub file_type: String,
}

pub async fn serve_image(
    State(state): State<AppState>,
    AxumPath(hash): AxumPath<String>,
) -> Result<impl IntoResponse, AppError> {
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

pub async fn download_file(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<i64>,
    Query(params): Query<FileQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let conn = state.pool.get()?;

    let (file_path, filename, content_type) = match params.file_type.as_str() {
        "stl" => {
            let mut stmt = conn.prepare(
                "SELECT file_path, filename FROM stl_files WHERE id = ?1"
            )?;
            
            let (path, name): (String, String) = stmt.query_row([id], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;
            
            (path, name, "model/stl".to_string())
        }
        "image" => {
            let mut stmt = conn.prepare(
                "SELECT file_path, filename FROM image_files WHERE id = ?1"
            )?;
            
            let (path, name): (String, String) = stmt.query_row([id], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;
            
            // Determine content type from filename extension
            let mime_type = match std::path::Path::new(&name).extension().and_then(|e| e.to_str()) {
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("png") => "image/png",
                Some("gif") => "image/gif",
                Some("webp") => "image/webp",
                _ => "application/octet-stream",
            }.to_string();
            
            (path, name, mime_type)
        }
        _ => return Err(AppError::BadRequest("Invalid file type".to_string())),
    };

    drop(conn);

    let file = File::open(&file_path).await?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(body)
        .unwrap())
}

pub async fn download_project_zip(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<i64>,
) -> Result<impl IntoResponse, AppError> {
    let conn = state.pool.get()?;

    // Get project name
    let project_name: String = conn.query_row(
        "SELECT name FROM projects WHERE id = ?1",
        [project_id],
        |row| row.get(0),
    )?;

    drop(conn);

    // Create temporary ZIP file
    let temp_dir = std::env::temp_dir();
    let zip_filename = format!("{}.zip", project_name.replace("/", "_"));
    let zip_path = temp_dir.join(&zip_filename);

    // Create ZIP
    state.download_service
        .create_project_zip(project_id, &zip_path)
        .await?;

    // Stream the ZIP file
    let file = File::open(&zip_path).await?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    // Clean up temp file after streaming (in a separate task)
    let zip_path_clone = zip_path.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let _ = tokio::fs::remove_file(zip_path_clone).await;
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/zip")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", zip_filename),
        )
        .body(body)
        .unwrap())
}
