use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use common::database::calc::CalcDatabase;
use serde_json::json;

use crate::config::AppConfig;
use crate::services::calc_crud::CalcCrudService;
use crate::Dependencies;

#[derive(Clone)]
pub struct CalcCrudApiState {
    pub service: CalcCrudService,
}

pub fn router(config: &AppConfig, deps: Dependencies) -> Router {
    Router::<Arc<CalcCrudApiState>>::new()
        .route("/calculations", routing::get(list_results))
        .route("/calculations", routing::delete(delete_results))
        .with_state(Arc::new(CalcCrudApiState {
            service: CalcCrudService::new(CalcDatabase::new(
                deps.mongo,
                config.database_name.to_string(),
            )),
        }))
}

#[tracing::instrument(skip_all)]
async fn list_results(
    State(state): State<Arc<CalcCrudApiState>>,
    Query(query): Query<HashMap<String, i64>>,
) -> impl IntoResponse {
    tracing::info!("Retrieving results");

    let min = query.get("min").copied();
    let max = query.get("max").copied();

    match state.service.list_results(min, max).await {
        Ok(result) => (StatusCode::OK, Json(json! { result })),
        Err(error) => {
            let error = error.to_string();
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": error})),
            )
        }
    }
}

#[tracing::instrument(skip_all)]
async fn delete_results(
    State(state): State<Arc<CalcCrudApiState>>,
    Query(query): Query<HashMap<String, i64>>,
) -> impl IntoResponse {
    tracing::info!("Deleting results");

    let min = query.get("min").copied();
    let max = query.get("max").copied();

    match state.service.delete_results(min, max).await {
        Ok(result) => (StatusCode::OK, Json(json!({ "deleted": result }))),
        Err(error) => {
            let error = error.to_string();
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": error})),
            )
        }
    }
}
