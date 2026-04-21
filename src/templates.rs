use crate::{
    models::{Food, Post},
};
use askama::Template;
use std::sync::Arc;

#[derive(Clone)]
pub struct BaseTemplateContext {
    pub title: String,
    pub favicon: &'static str,
}

impl BaseTemplateContext {
    pub fn new(title: impl Into<String>, favicon: &'static str) -> Self {
        Self {
            title: title.into(),
            favicon,
        }
    }
}

#[derive(Template)]
#[template(path = "pages/index.html", escape = "none")]
pub struct IndexTemplate {
    pub base: BaseTemplateContext,
    pub readme_html: Arc<String>,
}

#[derive(Template)]
#[template(path = "pages/register.html")]
pub struct RegisterTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/resume.html")]
pub struct ResumeTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/blog.html")]
pub struct BlogTemplate {
    pub base: BaseTemplateContext,
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "pages/contact_me.html")]
pub struct ContactTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/food.html")]
pub struct FoodTemplate<'a> {
    pub base: BaseTemplateContext,
    pub foods: &'a [Food],
}

#[derive(Template)]
#[template(path = "pages/food_detail.html")]
pub struct FoodDetailTemplate<'a> {
    pub base: BaseTemplateContext,
    pub food: &'a Food,
}

#[derive(Template)]
#[template(path = "pages/assets.html")]
pub struct AssetsTemplate {
    pub base: BaseTemplateContext,
}
