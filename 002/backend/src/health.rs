use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Router;

#[derive(Clone)]
pub struct HealthCheckState {
    pub service_name: String,
}

pub fn router(service_name: impl ToString) -> Router {
    Router::new()
        .route("/health", routing::get(health_check))
        .with_state(Arc::new(HealthCheckState {
            service_name: service_name.to_string(),
        }))
}

async fn health_check(
    State(state): State<Arc<HealthCheckState>>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if let Some(entry) = query.get("name") {
        if entry != &state.service_name {
            tracing::error!("bad service name");

            return StatusCode::BAD_REQUEST;
        }
    }

    StatusCode::OK
}
