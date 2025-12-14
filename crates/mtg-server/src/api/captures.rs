//! Capture API endpoints

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ListCapturesQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub since: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCaptureRequest {
    /// Base64 encoded image
    pub image: String,
    /// Capture metadata
    pub metadata: CaptureMetadata,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CaptureMetadata {
    pub captured_at: String,
    pub client_recognition: Option<ClientRecognition>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ClientRecognition {
    pub card_name: Option<String>,
    pub confidence: Option<f64>,
    pub ocr_raw: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCaptureRequest {
    pub final_card_id: Option<String>,
    pub user_verified: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CaptureListResponse {
    pub success: bool,
    pub data: Vec<CaptureData>,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize)]
pub struct CaptureResponse {
    pub success: bool,
    pub data: CaptureData,
}

#[derive(Debug, Serialize)]
pub struct CreateCaptureResponse {
    pub success: bool,
    pub data: CreateCaptureData,
}

#[derive(Debug, Serialize)]
pub struct CreateCaptureData {
    pub capture_id: String,
    pub server_recognition: Option<ServerRecognition>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ServerRecognition {
    pub card_id: String,
    pub card_name: String,
    pub confidence: f64,
}

#[derive(Debug, Serialize)]
pub struct CaptureData {
    pub id: String,
    pub captured_at: String,
    pub card_name: Option<String>,
    pub card_id: Option<String>,
    pub confidence: Option<f64>,
    pub user_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct ResponseMeta {
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

/// List captures for the authenticated client
pub async fn list_captures(Query(params): Query<ListCapturesQuery>) -> Json<CaptureListResponse> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);

    // TODO: Implement actual capture listing from database
    Json(CaptureListResponse {
        success: true,
        data: vec![],
        meta: ResponseMeta {
            total: 0,
            limit,
            offset,
        },
    })
}

/// Create a new capture (upload image)
pub async fn create_capture(
    Json(payload): Json<CreateCaptureRequest>,
) -> (StatusCode, Json<CreateCaptureResponse>) {
    let capture_id = Uuid::new_v4().to_string();

    let image_size = payload.image.len();
    let captured_at = &payload.metadata.captured_at;
    
    info!(
        capture_id = %capture_id,
        image_size_bytes = image_size,
        captured_at = %captured_at,
        "Received new capture submission"
    );

    // TODO: Implement actual capture processing
    // 1. Decode base64 image
    // 2. Store image to disk or object storage
    // 3. Run card recognition via ML model
    // 4. Store capture record in database

    (
        StatusCode::CREATED,
        Json(CreateCaptureResponse {
            success: true,
            data: CreateCaptureData {
                capture_id,
                server_recognition: None, // TODO: Run recognition
                status: "queued".to_string(),
            },
        }),
    )
}

/// Get a specific capture
pub async fn get_capture(Path(id): Path<String>) -> Json<CaptureResponse> {
    // TODO: Implement actual capture lookup
    Json(CaptureResponse {
        success: true,
        data: CaptureData {
            id,
            captured_at: chrono::Utc::now().to_rfc3339(),
            card_name: None,
            card_id: None,
            confidence: None,
            user_verified: false,
        },
    })
}

/// Update a capture (e.g., correct card identification)
pub async fn update_capture(
    Path(id): Path<String>,
    Json(payload): Json<UpdateCaptureRequest>,
) -> Json<CaptureResponse> {
    // TODO: Implement actual capture update
    let _final_card_id = payload.final_card_id;
    let user_verified = payload.user_verified.unwrap_or(false);

    Json(CaptureResponse {
        success: true,
        data: CaptureData {
            id,
            captured_at: chrono::Utc::now().to_rfc3339(),
            card_name: None,
            card_id: None,
            confidence: None,
            user_verified,
        },
    })
}
