use axum::Router;

pub fn apply_rate_limiting(router: Router) -> Router {
    #[cfg(debug_assertions)]
    {
        // Dev mode — no rate limiting
        router
    }

    #[cfg(not(debug_assertions))]
    {
        use tower_governor::{
            GovernorLayer,
            governor::GovernorConfigBuilder,
            key_extractor::{GlobalKeyExtractor, SmartIpKeyExtractor},
        };

        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        match env.as_str() {
            "production" => {
                let config = Box::leak(Box::new(
                    GovernorConfigBuilder::default()
                        .per_second(20)
                        .burst_size(40)
                        .key_extractor(SmartIpKeyExtractor)
                        .use_headers()
                        .finish()
                        .expect("Failed to build rate limiter configuration"),
                ));

                router.layer(GovernorLayer { config })
            }
            "benchmark" => {
                // No rate limiting in benchmark mode
                router
            }
            _ => {
                let config = Box::leak(Box::new(
                    GovernorConfigBuilder::default()
                        .per_second(20)
                        .burst_size(40)
                        .key_extractor(GlobalKeyExtractor)
                        .finish()
                        .expect("Failed to build rate limiter configuration"),
                ));

                router.layer(GovernorLayer { config })
            }
        }
    }
}
