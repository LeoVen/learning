use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Router;
use prometheus::Counter;
use prometheus::Encoder;
use prometheus::Opts;
use prometheus::TextEncoder;

#[derive(Clone)]
pub struct Metrics {
    pub api_count: Counter,
}

#[tracing::instrument]
pub fn setup() -> anyhow::Result<Metrics> {
    tracing::info!("Setting up metrics");

    let r = prometheus::default_registry();

    let api_count = Counter::with_opts(Opts::new("api_count", "Count of API requests"))?;
    r.register(Box::new(api_count.clone()))?;

    tracing::info!("Metrics setup finished");

    Ok(Metrics { api_count })
}

pub fn router() -> Router {
    Router::new().route("/metrics", routing::get(get_metrics))
}

async fn get_metrics() -> impl IntoResponse {
    let registry = prometheus::default_registry();

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();

    match encoder.encode(&metric_families, &mut buffer) {
        Ok(_) => {}
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    };

    let result = match String::from_utf8(buffer) {
        Ok(result) => result,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    };

    (StatusCode::OK, result)
}
