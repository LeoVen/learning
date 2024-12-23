use worker::Dependencies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = worker::config::load()?;

    worker::tracing::setup(&config);

    let client = worker::database::setup(&config).await?;

    worker::api::setup(&config, Dependencies { mongo: client }).await?;

    Ok(())
}
