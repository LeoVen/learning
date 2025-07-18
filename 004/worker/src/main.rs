use common::tracing::TracingConfig;
use queue::queue_recv;

pub mod processor;
pub mod queue;
pub mod serv_config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = serv_config::load()?;

    common::tracing::setup(&TracingConfig {
        environment: &config.environment,
        log_level: &config.log_level,
    });

    let storage = common::storage::Storage::new(&config.storage).await;

    queue_recv(&config.queue, storage).await?;

    Ok(())
}
