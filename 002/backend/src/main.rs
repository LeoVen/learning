pub mod api;
pub mod health;
pub mod metrics;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use anyhow::Context;
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

#[derive(Deserialize)]
struct ServiceConfig {
    pub service_name: String,
    pub copy: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(LevelFilter::DEBUG))
        .init();

    tracing::info!("Starting backend service");

    let cfg = envy::from_env::<ServiceConfig>()?;

    let trace_layer = TraceLayer::new_for_http();
    let metrics = metrics::setup()?;

    let app = Router::new()
        .layer(trace_layer)
        .merge(health::router(&cfg.service_name))
        .merge(api::router(&cfg.copy, metrics))
        .merge(metrics::router());

    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080)).await?;

    axum::serve(listener, app)
        .await
        .context("Axum serve failed")?;

    Ok(())
}
