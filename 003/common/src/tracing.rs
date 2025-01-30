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
            .with(fmt::layer().with_filter(LevelFilter::DEBUG))
            .init();
    } else {
        tracing_subscriber::fmt().json().init();
    }

    tracing::info!("Tracing setup finished");
}
