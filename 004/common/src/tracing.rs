use std::str::FromStr;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct TracingConfig<'a> {
    pub environment: &'a str,
    pub log_level: &'a str,
}

pub fn setup(config: &TracingConfig) {
    let filter = LevelFilter::from_str(&config.log_level).unwrap_or(LevelFilter::INFO);

    if config.environment == "dev" {
        tracing_subscriber::registry()
            .with(fmt::layer().with_filter(filter))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(fmt::layer().json().with_filter(filter))
            .init();
    }

    tracing::info!("Tracing setup finished");
}
