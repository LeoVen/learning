pub mod calc;

use mongodb::Client;

use crate::config::AppConfig;

#[tracing::instrument(skip_all)]
pub async fn setup(config: &AppConfig) -> anyhow::Result<Client> {
    let result = Client::with_uri_str(&config.database_uri).await?;
    tracing::info!("Database setup");
    Ok(result)
}
