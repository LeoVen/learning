pub mod files;
pub mod upload;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::time::Duration;
use std::time::SystemTime;

use anyhow::Context;
use axum::Router;
use axum::body::Body;
use axum::extract::MatchedPath;
use axum::extract::Request;
use axum::response::Response;
use tower_http::trace::TraceLayer;

use crate::Dependencies;
use crate::serv_config::AppConfig;

pub async fn setup(config: &AppConfig, deps: Dependencies) -> anyhow::Result<()> {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            tracing::info_span!(
                "http_context",
                method = ?request.method(),
                path,
            )
        })
        .on_request(move |_request: &Request<Body>, span: &tracing::Span| {
            let _ = span.enter();
            let time = SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap() // Unwrap is fine because UNIX_EPOCH is always earlier than SystemTime::now()
                .as_secs();

            tracing::info!(time, "http_on_request");
        })
        .on_response(
            move |response: &Response<Body>, duration: Duration, span: &tracing::Span| {
                let _ = span.enter();

                let status = response.status().to_string();
                let duration = duration.as_millis();

                tracing::info!(status, duration, "http_on_response");
            },
        );

    let upload = upload::router(&deps);
    let files = files::router();
    let app = Router::<()>::new()
        .merge(upload)
        .merge(files)
        .layer(trace_layer);

    tracing::info!("connecting service to port {}", config.api_port);

    let listener = tokio::net::TcpListener::bind(SocketAddrV4::new(
        Ipv4Addr::new(0, 0, 0, 0),
        config.api_port,
    ))
    .await?;

    axum::serve(listener, app)
        .await
        .context("Axum serve failed")?;

    Ok(())
}
