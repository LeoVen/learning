use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

pub struct TracingConfig<'a> {
    pub environment: &'a str,
}

pub fn setup(config: &TracingConfig) {
    if config.environment == "dev" {
        tracing_subscriber::registry()
            .with(fmt::layer().with_filter(LevelFilter::TRACE))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(fmt::layer().json().with_filter(LevelFilter::INFO))
            .init();
    }

    tracing::info!("Tracing setup finished");
}
