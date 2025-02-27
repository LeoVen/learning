use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use serde_json::json;

use crate::services::upload::UploadService;
use crate::Dependencies;

struct UploadState {
    service: UploadService,
}

pub fn router(deps: &Dependencies) -> Router {
    Router::new()
        .route("/pre-signed/{name}", routing::get(get_presigned))
        .with_state(Arc::new(UploadState {
            service: UploadService::new(deps.storage.clone(), "file-upload".to_string()),
        }))
}

#[tracing::instrument(skip_all)]
pub async fn get_presigned(
    State(state): State<Arc<UploadState>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    tracing::trace!(fn = "get_presigned", name, "API Call");

    match state.service.get_presigned(&name).await {
        Ok(result) => (StatusCode::OK, Json(json!(result))),
        Err(error) => {
            let error = error.to_string();

            tracing::error!(error, "Error when getting pre-signed URL");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": error})),
            )
        }
    }
}
