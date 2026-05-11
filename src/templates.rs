// src/templates.rs
use crate::{
    assets::Assets,
    models::{Food, Post},
    navbar::{DocItem, LANGUAGES, Language, NAV_ITEMS, NavItem, THEMES, Theme, DOCS},
    state::AppState,
};
use askama::Template;
use std::sync::Arc;

#[derive(Clone)]
pub struct BaseTemplateContext {
    pub title: String,
    pub favicon: &'static str,
    pub assets: Arc<Assets>,

    pub nav_items: &'static [NavItem],
    pub themes: &'static [Theme],
    pub languages: &'static [Language],

    pub docs: &'static [DocItem],
}

impl BaseTemplateContext {
    pub fn new(
        title: impl Into<String>,
        favicon: &'static str,
        assets: Arc<Assets>,
        docs: &'static [DocItem],
    ) -> Self {
        Self {
            title: title.into(),
            favicon,
            assets,
            nav_items: NAV_ITEMS,
            themes: THEMES,
            languages: LANGUAGES,
            docs,
        }
    }

    pub fn build_base_context(
        state: &AppState,
        title: impl Into<String>,
        favicon: &'static str,
    ) -> BaseTemplateContext {
        BaseTemplateContext::new(
            title,
            favicon,
            state.ctx.content.assets.clone(),
            DOCS,
        )
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
#[template(path = "pages/docs.html", escape = "none")]
pub struct DocsTemplate {
    pub base: BaseTemplateContext,
    pub title: &'static str,
    pub content_html: String,
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
