use axum::Router;
use axum::routing::get_service;
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new().route("/", get_service(ServeDir::new("./public/")))
}
