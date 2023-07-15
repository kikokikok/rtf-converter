//! Generic convert route.
use crate::error::AppResult;
use axum::{
    body::Bytes,
    extract::Multipart,
    http::StatusCode, Json,
};
use serde_json::json;
use axum_typed_multipart::{FieldData, TempFile, TryFromMultipart, TypedMultipart};
use utoipa::{ToSchema, IntoParams};
use utoipa::openapi::schema::KnownFormat;

#[derive(TryFromMultipart, IntoParams, ToSchema)]
pub struct RequestData {
    #[schema(value_type = String, format = Binary)]
    rtf_file: FieldData<Bytes>,
}

// #[utoipa::path(
// post,
// path = "/convert",
// request_body(content = RequestData, description = "RTF file content", content_type = "multipart/form-data"),
// responses(
// (status = 200, description = "Conversion successful"),
// (status = 500, description = "Conversion failed", body=AppError)
// )
// )]
#[utoipa::path(
post,
path = "/convert",
request_body(content = RequestData, description = "RTF file content", content_type = "multipart/form-data"),
responses(
(status = 200, description = "Conversion successful"),
(status = 500, description = "Conversion failed", body=AppError)
)
)]
pub async fn convert(TypedMultipart(RequestData { rtf_file }): TypedMultipart<RequestData>,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    let resp = format!(
        "file name = '{}', content type = '{}', size = '{}'",
        rtf_file.metadata.file_name.unwrap_or(String::new()),
        rtf_file.metadata.content_type.unwrap_or(String::from("text/plain")),
        rtf_file.contents.len()
    );
    Ok((StatusCode::OK, Json(json!({ "msg": resp}))))
}


