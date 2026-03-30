use crate::assets::{Assets, load_manifest};
use crate::error::AppError;
use crate::config::Config;
use crate::models::Food;
use crate::repository::mock_food_data;

use pulldown_cmark::{Parser, html};
use sqlx::PgPool;
use std::fs;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub readme_html: Arc<String>,
    pub food_data: Arc<Vec<Food>>,
    pub assets: Arc<Assets>,
}

impl AppState {
    #[must_use]
    pub fn new(db: PgPool, config: Config) -> Result<Self, AppError> {
        let manifest = load_manifest()?;
        let assets = Assets {
            css: manifest
                .get("index.css")
                .expect("missing css bundle")
                .clone(),
            js: manifest.get("app.js").expect("missing js bundle").clone(),
        };

        let readme_md =
            fs::read_to_string("./readme.md").unwrap_or("# README not found".to_string());

        let parser = Parser::new(&readme_md);
        let mut readme_html = String::new();
        html::push_html(&mut readme_html, parser);

        Ok(Self {
            db,
            config: Arc::new(config),
            readme_html: Arc::new(readme_html),
            food_data: Arc::new(mock_food_data()),
            assets: Arc::new(assets),
        })
    }
}
