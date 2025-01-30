use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Router;
use common::database::calc::CalcDatabase;

use crate::config::AppConfig;
use crate::services::calc::CalcService;
use crate::Dependencies;

#[derive(Clone)]
pub struct CalcApiState {
    pub service: CalcService,
}

pub fn router(config: &AppConfig, deps: Dependencies) -> Router {
    Router::<Arc<CalcApiState>>::new()
        .route("/calc/{p}", routing::get(calculate_prime))
        .with_state(Arc::new(CalcApiState {
            service: CalcService::new(CalcDatabase::new(
                deps.mongo,
                config.database_name.to_string(),
            )),
        }))
}

#[tracing::instrument(skip_all)]
async fn calculate_prime(
    State(state): State<Arc<CalcApiState>>,
    Path(p): Path<i64>,
) -> impl IntoResponse {
    state.service.calculate_prime(p);

    (StatusCode::ACCEPTED, format!("calculating for {}", p))
}
