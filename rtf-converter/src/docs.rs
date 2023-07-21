//! OpenAPI doc generation.

use crate::{
    error::AppError,
    routes::template::{self, TemplateUploadRequest, TemplateUploadResponse, ReqUuid, ReqDateTimeUtc},
    routes::health,
    routes::convert::{self, RequestData}
};
use utoipa::OpenApi;

/// API documentation generator.
#[derive(OpenApi)]
#[openapi(
        paths(health::healthcheck, convert::convert, template::upload),
        components(schemas(AppError), schemas(TemplateUploadRequest), schemas(TemplateUploadResponse), schemas(RequestData), schemas(ReqUuid), schemas(ReqDateTimeUtc)),
        tags(
            (name = "", description = "rtf-converter service/middleware")
        )
    )]

/// Tied to OpenAPI documentation.
#[derive(Debug)]
pub struct ApiDoc;
