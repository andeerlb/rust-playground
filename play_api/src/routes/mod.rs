use axum::{Router, middleware};
use crate::middleware::auth::auth_middleware;

pub mod private;
pub mod public;

pub fn create_routes() -> Router {
    Router::new()
        .nest("/api", public::routes())
        .nest(
            "/api",
            private::routes()
                .layer(middleware::from_fn(auth_middleware)),
        )
}