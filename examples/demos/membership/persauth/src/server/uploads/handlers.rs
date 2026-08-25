use axum::{
    body::Bytes,
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use super::models::{UploadConfig, UploadResponse};

/// Handle image upload
pub async fn upload_image(mut multipart: Multipart) -> impl IntoResponse {
    let config = UploadConfig::default();

    // Ensure upload directory exists
    if let Err(e) = fs::create_dir_all(&config.upload_dir).await {
        log::error!("Failed to create upload directory: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UploadResponse::error("Failed to create upload directory")),
        );
    }

    // Process the multipart form
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        if name != "file" {
            continue;
        }

        // Get content type
        let content_type = field
            .content_type()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Validate content type
        if !config.allowed_types.contains(&content_type) {
            return (
                StatusCode::BAD_REQUEST,
                Json(UploadResponse::error(&format!(
                    "Invalid file type: {}. Allowed types: {:?}",
                    content_type, config.allowed_types
                ))),
            );
        }

        // Get original filename
        let original_filename = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Read file data
        let data: Bytes = match field.bytes().await {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to read upload data: {}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(UploadResponse::error("Failed to read upload data")),
                );
            }
        };

        // Check file size
        let file_size = data.len() as u64;
        if file_size > config.max_size {
            return (
                StatusCode::BAD_REQUEST,
                Json(UploadResponse::error(&format!(
                    "File too large. Maximum size: {} bytes",
                    config.max_size
                ))),
            );
        }

        // Generate unique filename
        let extension = Path::new(&original_filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("bin");

        let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);
        let file_path = format!("{}/{}", config.upload_dir, unique_filename);

        // Save file
        let mut file = match fs::File::create(&file_path).await {
            Ok(file) => file,
            Err(e) => {
                log::error!("Failed to create file: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(UploadResponse::error("Failed to save file")),
                );
            }
        };

        if let Err(e) = file.write_all(&data).await {
            log::error!("Failed to write file: {}", e);
            // Try to clean up
            let _ = fs::remove_file(&file_path).await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UploadResponse::error("Failed to write file")),
            );
        }

        // Generate URL
        let url = format!("{}/{}", config.base_url, unique_filename);

        log::info!(
            "Uploaded file: {} -> {} ({} bytes, {})",
            original_filename,
            unique_filename,
            file_size,
            content_type
        );

        return (
            StatusCode::OK,
            Json(UploadResponse::success(
                url,
                unique_filename,
                file_size,
                content_type,
            )),
        );
    }

    (
        StatusCode::BAD_REQUEST,
        Json(UploadResponse::error("No file provided")),
    )
}

/// Delete an uploaded file
pub async fn delete_image(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> impl IntoResponse {
    let config = UploadConfig::default();

    // Validate filename (prevent directory traversal)
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return (
            StatusCode::BAD_REQUEST,
            Json(UploadResponse::error("Invalid filename")),
        );
    }

    let file_path = format!("{}/{}", config.upload_dir, filename);

    // Check if file exists
    if !Path::new(&file_path).exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(UploadResponse::error("File not found")),
        );
    }

    // Delete file
    match fs::remove_file(&file_path).await {
        Ok(_) => {
            log::info!("Deleted file: {}", filename);
            (
                StatusCode::OK,
                Json(UploadResponse {
                    success: true,
                    message: "File deleted".to_string(),
                    url: None,
                    filename: Some(filename),
                    size: None,
                    content_type: None,
                }),
            )
        }
        Err(e) => {
            log::error!("Failed to delete file: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UploadResponse::error("Failed to delete file")),
            )
        }
    }
}

/// List uploaded files (for admin purposes)
pub async fn list_images() -> impl IntoResponse {
    let config = UploadConfig::default();

    let mut files = Vec::new();

    match fs::read_dir(&config.upload_dir).await {
        Ok(mut entries) => {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_file() {
                        let filename = entry.file_name().to_string_lossy().to_string();
                        let url = format!("{}/{}", config.base_url, filename);
                        files.push(serde_json::json!({
                            "filename": filename,
                            "url": url,
                            "size": metadata.len(),
                        }));
                    }
                }
            }
        }
        Err(e) => {
            log::error!("Failed to read upload directory: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to list files",
                    "files": []
                })),
            );
        }
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "message": "Files listed",
            "files": files
        })),
    )
}
