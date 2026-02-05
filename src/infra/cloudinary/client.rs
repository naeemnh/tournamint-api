//! Cloudinary REST API client for authenticated uploads.

use crate::shared::errors::AppError;
use serde::Deserialize;
use sha1_smol::Sha1;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Parsed Cloudinary config from CLOUDINARY_URL
/// Format: cloudinary://api_key:api_secret@cloud_name
#[derive(Debug, Clone)]
pub struct CloudinaryConfig {
    pub api_key: String,
    pub api_secret: String,
    pub cloud_name: String,
}

impl CloudinaryConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let url = std::env::var("CLOUDINARY_URL")
            .map_err(|_| AppError::InternalError("CLOUDINARY_URL is not set".to_string()))?;

        let parsed = url::Url::parse(&url).map_err(|e| {
            AppError::InternalError(format!("Invalid CLOUDINARY_URL: {}", e))
        })?;

        if parsed.scheme() != "cloudinary" {
            return Err(AppError::InternalError(
                "CLOUDINARY_URL must use cloudinary:// scheme".to_string(),
            ));
        }

        let host = parsed.host_str().ok_or_else(|| {
            AppError::InternalError("CLOUDINARY_URL missing cloud name".to_string())
        })?;
        let cloud_name = host.to_string();

        let api_key = parsed.username();
        let api_secret = parsed.password().unwrap_or("");

        if api_key.is_empty() || api_secret.is_empty() {
            return Err(AppError::InternalError(
                "CLOUDINARY_URL must include api_key and api_secret".to_string(),
            ));
        }

        Ok(Self {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            cloud_name,
        })
    }
}

#[derive(Debug, Deserialize)]
struct CloudinaryApiResponse {
    secure_url: Option<String>,
    url: Option<String>,
    public_id: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    duration: Option<f64>,
    bytes: Option<i64>,
    error: Option<CloudinaryError>,
}

#[derive(Debug, Deserialize)]
struct CloudinaryError {
    message: String,
}

#[derive(Debug, Clone)]
pub struct CloudinaryUploadResult {
    pub secure_url: String,
    pub public_id: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<f64>,
    pub bytes: Option<i64>,
}

pub struct CloudinaryClient {
    config: CloudinaryConfig,
    http_client: reqwest::Client,
}

impl CloudinaryClient {
    pub fn new(config: CloudinaryConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Generate SHA1 signature for Cloudinary authenticated upload.
    /// Params must be sorted alphabetically, concatenated as key=value, then api_secret appended.
    fn signature(&self, params: &BTreeMap<String, String>) -> String {
        let to_sign: String = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("");
        let to_sign = format!("{}{}", to_sign, self.config.api_secret);
        let hash = Sha1::from(to_sign).digest();
        hex::encode(hash.bytes())
    }

    /// Upload bytes to Cloudinary.
    /// resource_type: "image" or "video"
    pub async fn upload(
        &self,
        bytes: &[u8],
        resource_type: &str,
        public_id: &str,
    ) -> Result<CloudinaryUploadResult, AppError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .as_secs()
            .to_string();

        let mut params = BTreeMap::new();
        params.insert("public_id".to_string(), public_id.to_string());
        params.insert("timestamp".to_string(), timestamp.clone());

        let signature = self.signature(&params);

        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/{}/upload",
            self.config.cloud_name, resource_type
        );

        let part = reqwest::multipart::Part::bytes(bytes.to_vec()).file_name("file");

        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("api_key", self.config.api_key.clone())
            .text("timestamp", timestamp)
            .text("signature", signature)
            .text("public_id", public_id.to_string());

        let resp = self
            .http_client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Cloudinary upload request failed: {}", e);
                AppError::InternalError(format!("Cloudinary upload failed: {}", e))
            })?;

        let body = resp
            .json::<CloudinaryApiResponse>()
            .await
            .map_err(|e| {
                eprintln!("Cloudinary response parse error: {}", e);
                AppError::InternalError("Failed to parse Cloudinary response".to_string())
            })?;

        if let Some(err) = body.error {
            eprintln!("Cloudinary API error: {}", err.message);
            return Err(AppError::InternalError(format!(
                "Cloudinary error: {}",
                err.message
            )));
        }

        let secure_url = body
            .secure_url
            .or(body.url)
            .ok_or_else(|| {
                AppError::InternalError("Cloudinary response missing URL".to_string())
            })?;

        let public_id = body
            .public_id
            .unwrap_or_else(|| public_id.to_string());

        Ok(CloudinaryUploadResult {
            secure_url,
            public_id,
            width: body.width,
            height: body.height,
            duration: body.duration,
            bytes: body.bytes,
        })
    }
}

