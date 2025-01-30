pub mod calc;

pub struct DatabaseConfig<U> {
    pub database_uri: U,
}

#[tracing::instrument(skip_all)]
pub async fn setup<U>(config: DatabaseConfig<U>) -> anyhow::Result<mongodb::Client>
where
    U: AsRef<str>,
{
    let result = mongodb::Client::with_uri_str(config.database_uri).await?;

    tracing::info!("Database setup");

    Ok(result)
}
