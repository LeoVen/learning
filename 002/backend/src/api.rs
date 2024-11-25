use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use serde_json::json;

#[derive(Clone)]
pub struct GreetState {
    pub copy: String,
}

pub fn router(copy: impl ToString) -> Router {
    Router::new()
        .route("/greet/:name", routing::get(greet))
        .with_state(Arc::new(GreetState {
            copy: copy.to_string(),
        }))
}

async fn greet(
    State(state): State<Arc<GreetState>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"message": format!("Hello, {}!", name), "copy": state.copy})),
    )
}
