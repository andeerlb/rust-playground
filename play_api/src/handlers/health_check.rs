use axum::Json;
use serde::Serialize;


#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: &'static str,
}

pub async fn is_ok() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse { status: "ok" })
}