use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use anyhow::Context;
use axum::Router;

use crate::config::AppConfig;
use crate::Dependencies;

pub mod calc;

pub async fn setup(config: &AppConfig, deps: Dependencies) -> anyhow::Result<()> {
    let calc_api = calc::router(config, deps);
    let app = Router::<()>::new().merge(calc_api);

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
