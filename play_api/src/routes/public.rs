use axum::{Router, routing::get};
use crate::handlers::health_check;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check::is_ok))
}