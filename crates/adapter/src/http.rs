// Adapter for HTTP (Axum handlers)
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
}
