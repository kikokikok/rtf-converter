use async_trait::async_trait;
use crate::error::AppResult;
use axum::{
    body::Bytes as BodyBytes,
    http::StatusCode, Json,
};
use axum::extract::multipart::Field;
use serde_json::json;
use serde;
use axum_typed_multipart::{FieldData, TryFromField, TryFromMultipart, TypedMultipart, TypedMultipartError};
use chrono::{DateTime, FixedOffset};
use utoipa::{ToSchema, IntoParams};
use uuid::Uuid;
use crate::models::file::FileIdentifier;

#[derive(ToSchema)]
pub struct ReqUuid(Uuid);

#[derive(ToSchema)]
pub struct ReqDateTimeUtc(DateTime<FixedOffset>);

#[async_trait]
impl TryFromField for ReqUuid {
    async fn try_from_field(field: Field<'_>) -> Result<Self, TypedMultipartError> {
        let field_name = field.name().unwrap().to_string();
        let text = field.text().await?;
        let result = Uuid::parse_str(&text).map_err(move |_| TypedMultipartError::WrongFieldType {
            field_name,
            wanted_type: "uuid::Uuid".to_string()});

        match result {
            Ok(uuid) => Ok(ReqUuid(uuid)),
            Err(err) => Err(err)
        }
    }
}

#[async_trait]
impl TryFromField for ReqDateTimeUtc {
    async fn try_from_field(field: Field<'_>) -> Result<Self, TypedMultipartError> {
        let field_name = field.name().unwrap().to_string();
        let text = field.text().await?;
        let result = DateTime::parse_from_rfc2822(&text).map_err(move |_| TypedMultipartError::WrongFieldType {
            field_name,
            wanted_type: "chrono::DateTime<FixedOffset>".to_string()});

        match result {
            Ok(date) => Ok(ReqDateTimeUtc(date)),
            Err(err) => Err(err)
        }
    }
}

#[derive(TryFromMultipart, IntoParams, ToSchema)]
pub struct TemplateUploadRequest {
    #[schema(value_type = String, format = Binary)]
    file: FieldData<BodyBytes>,
    //file_name: Option<String>,
    tenant_id: Option<ReqUuid>,
    owner_id: Option<ReqUuid>,
    max_age: Option<ReqDateTimeUtc>,
    templating_engine: Option<String>,
    templating_engine_version: Option<i32>,
}

#[derive(IntoParams, ToSchema)]
pub struct TemplateUploadResponse {
    id: FileIdentifier
}

#[utoipa::path(
post,
path = "/template",
request_body(content = TemplateUploadRequest, description = "RTF file content", content_type = "multipart/form-data"),
responses(
(status = 200, description = "Conversion successful"),
(status = 500, description = "Conversion failed", body=AppError)
)
)]
pub async fn upload(TypedMultipart(TemplateUploadRequest {file, tenant_id, templating_engine, templating_engine_version, owner_id, max_age }): TypedMultipart<TemplateUploadRequest>,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
            let resp = format!(
            "file name = '{}', content type = '{}', size = '{}'",
            file.metadata.file_name.unwrap_or(String::new()),
            file.metadata.content_type.unwrap_or(String::from("text/plain")),
            file.contents.len());
            Ok((StatusCode::OK, Json(json!({ "msg": resp}))))
}