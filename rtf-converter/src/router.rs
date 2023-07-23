//! Main [axum::Router] interface for webserver.

use std::sync::Arc;
use crate::{
    middleware::logging::{log_request_response, DebugOnlyLogger, Logger},
    routes::{fallback::notfound_404, health, convert, template},
    repositories::create_repositories,
};
use axum::{routing::get, routing::post, Router, Extension};

/// Setup main router for application.
pub async fn setup_app_router() -> Router {
    let mut router = Router::new()
        .route("/convert", post(convert::convert))
        .route("/template", post(template::upload)).layer(Extension(Arc::new(create_repositories().await.clone())))
        .fallback(notfound_404);

    router = router.layer(axum::middleware::from_fn(log_request_response::<Logger>));

    let mut healthcheck_router = Router::new().route("/healthcheck", get(health::healthcheck));

    healthcheck_router = healthcheck_router.layer(axum::middleware::from_fn(
        log_request_response::<DebugOnlyLogger>,
    ));

    Router::merge(router, healthcheck_router)
}
