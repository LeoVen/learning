use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    // Service
    #[serde(rename(deserialize = "environment"))]
    pub environment: String,
    // API
    #[serde(rename(deserialize = "api_port"))]
    pub api_port: u16,
    // Database
    #[serde(rename(deserialize = "database_uri"))]
    pub database_uri: String,
    #[serde(rename(deserialize = "database_name"))]
    pub database_name: String,
}

pub fn load() -> anyhow::Result<AppConfig> {
    let config = envy::from_env::<AppConfig>()?;

    Ok(config)
}
