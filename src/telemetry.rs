use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .json() // ← structured logs
                .with_target(false),
        )
        .init();
}