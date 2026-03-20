use crate::assets::Assets;
use crate::models::Food;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/index.html", escape = "none")]
pub struct IndexTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub readme_html: String,
    pub assets: Assets,
}

#[derive(Template)]
#[template(path = "pages/food.html")]
pub struct FoodTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub foods: Vec<Food>,
    pub assets: Assets,
}

#[derive(Template)]
#[template(path = "pages/food_detail.html")]
pub struct FoodDetailTemplate<'a> {
    pub title: String,
    pub favicon: &'static str,
    pub food: &'a Food,
    pub assets: Assets,
}

#[derive(Template)]
#[template(path = "pages/resume.html")]
pub struct ResumeTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub assets: Assets,
}

#[derive(Template)]
#[template(path = "pages/blog.html")]
pub struct BlogTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub assets: Assets,
}

#[derive(Template)]
#[template(path = "pages/contact_me.html")]
pub struct ContactTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub assets: Assets,
}

#[derive(Template)]
#[template(path = "pages/assets.html")]
pub struct AssetsTemplate {
    pub title: &'static str,
    pub favicon: &'static str,
    pub assets: Assets,
}
