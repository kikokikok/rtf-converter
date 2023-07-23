use std::ops::Add;
use humantime::Duration;
use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use axum::{body::Bytes as BodyBytes, Extension, http::StatusCode, Json};
use axum::extract::multipart::Field;
use axum_macros::{debug_handler, FromRequestParts};
use serde_json::json;
use serde;
use axum_typed_multipart::{FieldData, TryFromField, TryFromMultipart, TypedMultipart, TypedMultipartError};
use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use utoipa::{ToSchema, IntoParams};
use uuid::Uuid;

use crate::models::file::{FileIdentifier, NewFile};
use crate::repositories::{RepoExt, RepoImpls};
use crate::repositories::file::FileRepo;

#[derive(ToSchema, From, Into)]
pub struct ReqUuid(Uuid);

#[derive(ToSchema, From, Into)]
pub struct ReqHumanDuration(Duration);

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
impl TryFromField for ReqHumanDuration {
    async fn try_from_field(field: Field<'_>) -> Result<Self, TypedMultipartError> {
        let field_name = field.name().unwrap().to_string();
        let text = field.text().await?;

        let result = text.parse::<humantime::Duration>().map_err(move |_| TypedMultipartError::WrongFieldType {
            field_name,
            wanted_type: "humantime::Duration".to_string()});

        //
        // let result = DateTime::parse_from_rfc2822(&text).map_err(move |_| TypedMultipartError::WrongFieldType {
        //     field_name,
        //     wanted_type: "chrono::DateTime<FixedOffset>".to_string()});

        match result {
            Ok(date) => Ok(ReqHumanDuration(date)),
            Err(err) => Err(err)
        }
    }
}

#[derive(TryFromMultipart, ToSchema)]
pub struct TemplateUploadRequest {
    #[schema(value_type = String, format = Binary)]
    file: FieldData<BodyBytes>,
    tenant_id: Option<ReqUuid>,
    owner_id: Option<ReqUuid>,
    max_age: Option<ReqHumanDuration>,
    templating_engine: Option<String>,
    templating_engine_version: Option<i32>,
}

#[derive(ToSchema)]
pub struct TemplateUploadResponse {
    id: FileIdentifier
}

//#[debug_handler]
#[utoipa::path(
post,
path = "/template",
request_body(content = TemplateUploadRequest, description = "RTF file content", content_type = "multipart/form-data"),
responses(
(status = 200, description = "Conversion successful"),
(status = 500, description = "Conversion failed", body=AppError)
)
)]
pub async fn upload(Extension(repo): RepoExt, TypedMultipart(
                        TemplateUploadRequest {file, tenant_id, templating_engine, templating_engine_version, owner_id, max_age }): TypedMultipart<TemplateUploadRequest>)
    -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    let repo = repo.clone();

    let age = if let Some(age) = max_age {
        let date = chrono::Duration::from_std(age.0.into()).expect("Overflow datetime");
        Some(Utc::now().add(date))
    } else {
        None
    };

    let file_contents = file.contents;
    let file_size = file_contents.len() as i64;

    let result = repo.file.add(&NewFile {
        file_name: file.metadata.file_name.clone().unwrap(),
        content_type: file.metadata.content_type.unwrap(),
        file_binary_content: Vec::from(file_contents),
        file_size,
        owner_id: if let Some(owner) = owner_id { Some(owner.0) } else { None },
        max_age: age,
        tenant_id: if let Some(tenant) = tenant_id {Some(tenant.0)} else { None },
        insertion_date: Some(chrono::offset::Utc::now()),
        templating_engine,
        templating_engine_version,
    }).await;

    match result {
        Ok(result) => Ok((StatusCode::OK, Json(json!(FileIdentifier{id: result.id, version: result.version })))),
        Err(error) => Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, Some(error)))
    }

}