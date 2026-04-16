use crate::{
    assets::Assets,
    models::{Food},
};

use std::sync::Arc;

pub struct Content {
    //pub posts: Vec<Post>,
    pub assets: Arc<Assets>,
    pub readme_html: Arc<String>,
    pub food_data: Arc<Vec<Food>>,
}
