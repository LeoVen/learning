use common::database::DatabaseConfig;
use common::tracing::TracingConfig;
use worker::Dependencies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = worker::config::load()?;

    common::tracing::setup(&TracingConfig {
        environment: &config.environment,
    });

    let client = common::database::setup(DatabaseConfig {
        database_uri: &config.database_uri,
    })
    .await?;

    worker::api::setup(&config, Dependencies { mongo: client }).await?;

    Ok(())
}
