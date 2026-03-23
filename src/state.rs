use crate::assets::{Assets, load_manifest};
use pulldown_cmark::{Parser, html};
use sqlx::PgPool;
use std::fs;

use crate::models::Food;
use crate::repository::mock_food_data;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub readme_html: String,
    pub food_data: Vec<Food>,
    pub assets: Assets,
}

impl AppState {
    #[must_use]
    pub fn new(db: PgPool) -> Self {
        let manifest = load_manifest();
        let assets = Assets {
            css: manifest
                .get("index.css")
                .expect("missing css bundle")
                .clone(),
            js: manifest.get("app.js").expect("missing js bungle").clone(),
        };

        let readme_md =
            fs::read_to_string("./readme.md").unwrap_or_else(|_| "# README not found".to_string());

        let parser = Parser::new(&readme_md);
        let mut readme_html = String::new();
        html::push_html(&mut readme_html, parser);

        Self {
            db,
            readme_html,
            food_data: mock_food_data(),
            assets,
        }
    }
}
