use crate::{
    assets::{Assets, load_manifest},
    auth::AuthService,
    config::Config,
    content::Content,
    context::AppContext,
    error::AppError,
    repository::mock_food_data,
    services::Services,
};

use pulldown_cmark::{Parser, html};
use sqlx::PgPool;

use std::{fs, sync::Arc};

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

            js: manifest
                .get("app.js")
                .ok_or(AppError::Internal)?
                .clone(),
        };

        // Load README
        let readme_md =
            fs::read_to_string("./readme.md")
                .unwrap_or_else(|_| "# README not found".to_string());

        let parser = Parser::new(&readme_md);

        let mut readme_html = String::new();

        html::push_html(&mut readme_html, parser);

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

        Ok(Self {
            ctx: Arc::new(ctx),
        })
    }
}