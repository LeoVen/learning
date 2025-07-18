use common::storage::StorageConfig;
use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub api_port: u16,
    pub environment: String,
    pub storage: StorageConfig,
    pub log_level: String,
    // Database
    // #[serde(rename(deserialize = "database_uri"))]
    // pub database_uri: String,
    // #[serde(rename(deserialize = "database_name"))]
    // pub database_name: String,
}

#[tracing::instrument]
pub fn load() -> anyhow::Result<AppConfig> {
    let config = Config::builder()
        .add_source(config::File::with_name("./config/backend.conf.toml"))
        .build()?;

    tracing::info!("config file successfully loaded");

    let result = config.try_deserialize()?;

    Ok(result)
}
