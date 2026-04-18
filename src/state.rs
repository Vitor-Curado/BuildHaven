use crate::{
    assets::{Assets, load_manifest},
    auth::AuthService,
    config::Config,
    content::Content,
    context::AppContext,
    error::AppError,
    repository::mock_food_data,
    services::Services,
    utils::{load_readme, markdown_to_html},
};

use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub ctx: Arc<AppContext>,
}

impl AppState {
    pub fn new(db: PgPool, config: Config) -> Result<Self, AppError> {
        // Load assets manifest
        let manifest = load_manifest()?;

        let assets = Assets {
            css: manifest
                .get("index.css")
                .expect("missing css bundle")
                .clone(),

            js: manifest.get("app.js").ok_or(AppError::Internal)?.clone(),
        };

        // Load README
        let readme_md = load_readme();
        let readme_html = markdown_to_html(&readme_md);

        // Services
        let services = Services {
            db,
            auth: AuthService::new(),
        };

        // Content
        let content = Content {
            readme_html: Arc::new(readme_html),
            food_data: Arc::new(mock_food_data()),
            assets: Arc::new(assets),
        };

        // Context
        let ctx = AppContext {
            services,
            content,
            config,
        };

        Ok(Self { ctx: Arc::new(ctx) })
    }
}
