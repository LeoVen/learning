use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use serde_json::json;

use crate::metrics::Metrics;

#[derive(Clone)]
pub struct GreetState {
    pub copy: String,
    pub metrics: Metrics,
}

pub fn router(copy: impl ToString, metrics: Metrics) -> Router {
    Router::new()
        .route("/greet/:name", routing::get(greet))
        .with_state(Arc::new(GreetState {
            copy: copy.to_string(),
            metrics,
        }))
}

async fn greet(
    State(state): State<Arc<GreetState>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    state.metrics.api_count.inc();

    (
        StatusCode::OK,
        Json(json!({"message": format!("Hello, {}!", name), "copy": state.copy})),
    )
}
