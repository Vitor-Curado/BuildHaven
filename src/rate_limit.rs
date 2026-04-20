use crate::config::{Config, Environment};
use axum::Router;

pub fn apply_rate_limiting(router: Router, config: &Config) -> Router {
    // Box::leak() is required to satisfy 'static lifetime
    match config.app.environment {
        Environment::Production => {
            use tower_governor::{
                GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
            };

            let governor_config = Box::leak(Box::new(
                GovernorConfigBuilder::default()
                    .per_second(config.rate_limit.per_second)
                    .burst_size(config.rate_limit.burst_size)
                    .key_extractor(SmartIpKeyExtractor)
                    .use_headers()
                    .finish()
                    .expect("Failed to build rate limiter configuration"),
            ));

            router.layer(GovernorLayer {
                config: governor_config,
            })
        }
        Environment::Development => {
            use tower_governor::{
                GovernorLayer, governor::GovernorConfigBuilder, key_extractor::GlobalKeyExtractor,
            };

            let governor_config = Box::leak(Box::new(
                GovernorConfigBuilder::default()
                    .per_second(config.rate_limit.per_second)
                    .burst_size(config.rate_limit.burst_size)
                    .key_extractor(GlobalKeyExtractor)
                    .finish()
                    .expect("Failed to build rate limiter configuration"),
            ));

            router.layer(GovernorLayer {
                config: governor_config,
            })
        }
        Environment::Benchmark => {
            // No rate limiting in benchmark mode
            router
        }
    }
}
