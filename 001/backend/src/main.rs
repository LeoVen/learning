pub mod api;
pub mod auth;
pub mod error;
pub mod tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing::setup();

    crate::api::setup().await?;

    Ok(())
}
