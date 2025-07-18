use common::storage::Storage;
use common::tracing::TracingConfig;

pub mod api;
pub mod models;
pub mod serv_config;
pub mod services;

#[derive(Clone)]
pub struct Dependencies {
    pub storage: Storage,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = serv_config::load()?;

    common::tracing::setup(&TracingConfig {
        environment: &config.environment,
        log_level: &config.log_level,
    });

    let deps = Dependencies {
        storage: Storage::new(&config.storage).await,
    };

    api::setup(&config, deps).await?;

    Ok(())
}
