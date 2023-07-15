//! OpenAPI doc generation.

use crate::{
    error::AppError,
    routes::{
        convert::{
            self,
            RequestData
        },
        health,
    }
};
use utoipa::OpenApi;

/// API documentation generator.
#[derive(OpenApi)]
#[openapi(
        paths(health::healthcheck, convert::convert),
        components(schemas(AppError), schemas(RequestData)),
        tags(
            (name = "", description = "rtf-converter service/middleware")
        )
    )]

/// Tied to OpenAPI documentation.
#[derive(Debug)]
pub struct ApiDoc;
