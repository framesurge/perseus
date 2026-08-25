use serde::{Deserialize, Serialize};

/// Response for image upload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl UploadResponse {
    pub fn success(url: String, filename: String, size: u64, content_type: String) -> Self {
        Self {
            success: true,
            message: "Upload successful".to_string(),
            url: Some(url),
            filename: Some(filename),
            size: Some(size),
            content_type: Some(content_type),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            url: None,
            filename: None,
            size: None,
            content_type: None,
        }
    }
}

/// Upload constraints
pub struct UploadConfig {
    /// Maximum file size in bytes (default: 10MB)
    pub max_size: u64,
    /// Allowed MIME types
    pub allowed_types: Vec<String>,
    /// Upload directory path
    pub upload_dir: String,
    /// Base URL for serving files
    pub base_url: String,
}

impl Default for UploadConfig {
    fn default() -> Self {
        Self {
            max_size: 10 * 1024 * 1024, // 10MB
            allowed_types: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "image/webp".to_string(),
                "image/svg+xml".to_string(),
            ],
            upload_dir: "./dist/uploads".to_string(),
            base_url: "/uploads".to_string(),
        }
    }
}
