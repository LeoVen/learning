use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use config::Config;
use health::CustomHealthCheck;
use pingora::prelude::*;
use reqwest::Method;

pub mod config;
pub mod health;
pub mod logging;

fn main() -> anyhow::Result<()> {
    let cfg = Config::load("/config.toml")?;

    logging::setup(&cfg);

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let mut upstreams =
        LoadBalancer::try_from_iter(cfg.proxy.upstreams).context("failed to setup LoadBalancer")?;

    let hc = CustomHealthCheck::new(
        Method::GET,
        "/health",
        Some(vec![("name".to_string(), cfg.proxy.name)]),
    );
    upstreams.set_health_check(Box::new(hc));
    upstreams.health_check_frequency = Some(std::time::Duration::from_secs(5));

    let background = background_service("health check", upstreams);
    let upstreams = background.task();
    let mut lb = http_proxy_service(&server.configuration, LB(upstreams));
    lb.add_tcp(format!("0.0.0.0:{}", cfg.proxy.port).as_str());

    server.add_service(background);
    server.add_service(lb);
    server.run_forever();
}

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) {}

    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        let client_addr = session.client_addr().unwrap();
        upstream_request
            .insert_header("X-Forwarded-For", client_addr.to_string())
            .unwrap();
        Ok(())
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"", 256).unwrap();

        let peer = Box::new(HttpPeer::new(upstream, false, "".to_string()));
        Ok(peer)
    }
}
