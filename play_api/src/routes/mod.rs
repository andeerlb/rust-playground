use axum::{Router, routing::get};
use crate::handlers::hello;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello::hello_world))
}