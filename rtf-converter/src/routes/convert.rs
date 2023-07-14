//! Generic convert route.

use crate::error::AppResult;
use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::post,
    Extension, Json, Router,
};
use serde_json::json;

/// GET handler for internal pings and availability
#[utoipa::path(
post,
path = "/convert",
responses(
(status = 200, description = "Conversion successful"),
(status = 500, description = "Conversion failed", body=AppError)
)
)]

pub async fn convert(mut file: Multipart) -> AppResult<(StatusCode, Json<serde_json::Value>)> {

    Ok((StatusCode::OK, Json(json!({ "msg": "Healthy"}))))
}
