use axum::{Router, routing::get};
use crate::handlers::health_check;

pub fn routes() -> Router {
    Router::new()
        .route("/request", get(health_check::is_ok))
}