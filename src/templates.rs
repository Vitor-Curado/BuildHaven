use crate::assets::Assets;
use crate::models::{Food, Post};
use askama::Template;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "pages/index.html", escape = "none")]
pub struct IndexTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub readme_html: Arc<String>,
    pub assets: Arc<Assets>,
}

#[derive(Template)]
#[template(path = "pages/food.html")]
pub struct FoodTemplate<'a> {
    pub title: &'static str,
    pub favicon: &'static str,
    pub foods: &'a [Food],
    pub assets: Arc<Assets>,
}

#[derive(Template)]
#[template(path = "pages/food_detail.html")]
pub struct FoodDetailTemplate<'a> {
    pub title: String,
    pub favicon: &'static str,
    pub food: &'a Food,
    pub assets: Arc<Assets>,
}

#[derive(Template)]
#[template(path = "pages/resume.html")]
pub struct ResumeTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub assets: Arc<Assets>,
}

#[derive(Template)]
#[template(path = "pages/blog.html")]
pub struct BlogTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub assets: Arc<Assets>,

    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "pages/contact_me.html")]
pub struct ContactTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub assets: Arc<Assets>,
}

#[derive(Template)]
#[template(path = "pages/assets.html")]
pub struct AssetsTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub assets: Arc<Assets>,
}
