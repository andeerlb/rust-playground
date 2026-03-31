use axum::{
    http::Request,
    middleware::Next,
    response::{Response, IntoResponse},
    http::StatusCode,
};

pub async fn auth_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let auth_header = req.headers().get("authorization");

    if auth_header.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(req).await
}