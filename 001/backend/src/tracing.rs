use serde::Deserialize;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

#[derive(Deserialize, Debug)]
pub struct TracingConfig {
    #[serde(rename(deserialize = "environment"))]
    pub env: String,
}

pub fn setup() -> String {
    let env_vars = envy::from_env::<TracingConfig>();
    let config = env_vars.unwrap_or(TracingConfig {
        env: "prod".to_string(),
    });

    if config.env == "dev" {
        tracing_subscriber::registry()
            .with(fmt::layer().with_filter(LevelFilter::DEBUG))
            .init();
    } else {
        tracing_subscriber::fmt().json().init();
    }

    tracing::info!("Tracing setup finished");

    config.env
}
