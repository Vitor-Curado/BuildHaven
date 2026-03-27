use axum::Router;

#[cfg(not(debug_assertions))]
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

pub fn apply_rate_limiting(router: Router) -> Router {
    #[cfg(debug_assertions)]
    {
        // Dev mode — no rate limiting
        router
    }

    #[cfg(not(debug_assertions))]
    {
        let config = Box::leak(Box::new(
            GovernorConfigBuilder::default()
                .per_second(5)
                .burst_size(10)
                .finish()
                .unwrap(),
        ));

        router.layer(GovernorLayer { config })
    }
}
