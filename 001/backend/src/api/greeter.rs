use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Router;

pub fn router() -> Router {
    Router::<()>::new().route("/greetings/:name", routing::get(greet))
}

#[tracing::instrument(skip_all)]
async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("Hello, {}!", &name))
}
