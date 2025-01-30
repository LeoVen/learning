use backend::Dependencies;
use common::database::DatabaseConfig;
use common::tracing::TracingConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = backend::config::load()?;

    common::tracing::setup(&TracingConfig {
        environment: &config.environment,
    });

    let client = common::database::setup(DatabaseConfig {
        database_uri: &config.database_uri,
    })
    .await?;

    backend::api::setup(&config, Dependencies { mongo: client }).await?;

    Ok(())
}
