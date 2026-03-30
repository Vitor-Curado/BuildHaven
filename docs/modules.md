# 🧩 Project Modules

* **bin/assets.rs** — build assets
* **indexer.rs** — builds search or content indexes for fast lookup
* **api.rs** — JSON response structures (API responses)
* **assets.rs** — cache busting, compression of static files
* **config.rs** — environment-based application configuration
* **cors.rs** — CORS middleware configuration
* **error.rs** — defines application error types (thiserror)
* **handlers.rs** — HTTP request handlers
* **lib.rs** — crate module declarations
* **main.rs** — server initialization and runtime setup
* **models.rs** — domain data structures
* **pool.rs** — database connection pooling (SQLx)
* **rate_limit.rs** — request throttling using tower-governor
* **repository.rs** — mock data provider
* **router.rs** — Axum router middleware and global configuration
* **routes.rs** — route definitions
* **security.rs** — HTTP header & auth middleware
* **services.rs** — business logic layer / domain services
* **state.rs** — shared application state
* **templates.rs** — Askama template bindings
* **utils.rs** — utility helpers (markdown conversion, file loading)
* **tests.rs** — integration tests