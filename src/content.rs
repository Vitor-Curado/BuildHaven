use crate::{
    assets::Assets,
    models::Food,
    repository::mock_food_data,
    utils::{load_readme, markdown_to_html},
};

use std::sync::Arc;

#[derive(Clone)]
pub struct Content {
    //pub posts: Vec<Post>,
    pub assets: Arc<Assets>,
    pub readme_html: Arc<String>,
    pub food_data: Arc<Vec<Food>>,
}

impl Content {
    pub fn new(assets: Assets) -> Self {
        // Load README
        let readme_md = load_readme();
        let readme_html = markdown_to_html(&readme_md);

        Self {
            assets: Arc::new(assets),
            readme_html: Arc::new(readme_html),
            food_data: Arc::new(mock_food_data()), // Placeholder, replace with actual data loading
        }
    }
}
