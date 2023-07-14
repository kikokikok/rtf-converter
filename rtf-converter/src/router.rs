//! Main [axum::Router] interface for webserver.

use crate::{
    middleware::logging::{log_request_response, DebugOnlyLogger, Logger},
    routes::{fallback::notfound_404, health, convert},
};
use axum::{routing::get, routing::post, Router};

/// Setup main router for application.
pub fn setup_app_router() -> Router {
    let mut router = Router::new()
        .route("/convert", post(convert::convert))
        .fallback(notfound_404);

    router = router.layer(axum::middleware::from_fn(log_request_response::<Logger>));

    let mut healthcheck_router = Router::new().route("/healthcheck", get(health::healthcheck));

    healthcheck_router = healthcheck_router.layer(axum::middleware::from_fn(
        log_request_response::<DebugOnlyLogger>,
    ));

    Router::merge(router, healthcheck_router)
}
