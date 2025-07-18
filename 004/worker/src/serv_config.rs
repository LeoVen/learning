use common::storage::StorageConfig;
use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueueConsumerConfig {
    pub queue_name: String,
    pub tag: String,
}

#[derive(Deserialize, Debug)]
pub struct QueueConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
    pub consumer: QueueConsumerConfig,
    // pub producer: QueueProducerConfig,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub api_port: u16,
    pub environment: String,
    pub log_level: String,
    pub queue: QueueConfig,
    pub storage: StorageConfig,
}

#[tracing::instrument]
pub fn load() -> anyhow::Result<AppConfig> {
    let config = Config::builder()
        .add_source(config::File::with_name("./config/worker.conf.toml"))
        .build()?;

    tracing::info!("config file successfully loaded");

    let result = config.try_deserialize()?;

    Ok(result)
}
