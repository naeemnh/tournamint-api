//! Multipart form-data extraction helpers.

use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures_util::StreamExt;

use crate::shared::ApiResponse;

/// Extracts file bytes from a multipart payload.
/// Looks for a field named "file", enforces max_bytes limit.
/// Returns Err(HttpResponse) on missing file, empty file, or size exceed.
pub async fn extract_file_from_multipart(
    payload: &mut Multipart,
    max_bytes: usize,
) -> Result<Vec<u8>, HttpResponse> {
    let mut file_bytes = Vec::new();
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            ApiResponse::bad_request(&format!("Multipart parse error: {}", e))
        })?;
        if field.name() == Some("file") {
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| {
                    ApiResponse::bad_request(&format!("Stream error: {}", e))
                })?;
                if file_bytes.len() + data.len() > max_bytes {
                    return Err(ApiResponse::payload_too_large(
                        "File exceeds maximum allowed size",
                    ));
                }
                file_bytes.extend_from_slice(&data);
            }
            break;
        }
    }
    if file_bytes.is_empty() {
        return Err(ApiResponse::bad_request("Missing or empty file field"));
    }
    Ok(file_bytes)
}
