pub mod admin;
pub mod greeter;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use anyhow::Context;
use axum::middleware;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::auth::KeycloakAuthMiddleware;

#[derive(Serialize, Deserialize, Debug)]
struct AxumApiConfig {
    #[serde(rename(deserialize = "api_axum_port"))]
    pub port: u16,
}

pub async fn setup() -> anyhow::Result<()> {
    let config = envy::from_env::<AxumApiConfig>().context("Failed to get env vars")?;

    let trace_layer = TraceLayer::new_for_http();

    let greeter = greeter::router().layer(trace_layer.clone());

    let admin =
        admin::router()
            .layer(trace_layer.clone())
            .route_layer(middleware::from_fn_with_state(
                KeycloakAuthMiddleware::new()?,
                KeycloakAuthMiddleware::authenticate,
            ));

    let app = Router::new().merge(greeter).merge(admin);

    let listener =
        tokio::net::TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), config.port))
            .await?;

    axum::serve(listener, app)
        .await
        .context("Axum serve failed")?;

    Ok(())
}
