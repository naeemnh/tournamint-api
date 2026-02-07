//! Cloudinary media upload integration.
//! Uses CLOUDINARY_URL (cloudinary://api_key:api_secret@cloud_name) for credentials.

mod client;

pub use client::{CloudinaryClient, CloudinaryConfig, CloudinaryUploadResult};
