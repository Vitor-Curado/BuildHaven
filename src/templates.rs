use crate::{
    assets::Assets,
    models::Post,
    navbar::{DOCS, DocItem, THEMES, Theme},
    state::AppState,
};
use askama::Template;
use std::sync::Arc;

#[derive(Clone)]
pub struct BaseTemplateContext {
    pub title: String,
    pub favicon: &'static str,
    pub assets: Arc<Assets>,
    pub themes: &'static [Theme],
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
            themes: THEMES,
            docs,
        }
    }

    pub fn build_base_context(
        state: &AppState,
        title: impl Into<String>,
        favicon: &'static str,
    ) -> BaseTemplateContext {
        BaseTemplateContext::new(title, favicon, state.assets.clone(), DOCS)
    }
}

#[derive(Template)]
#[template(path = "pages/index.html", escape = "none")]
pub struct IndexTemplate {
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
#[template(path = "pages/blog_post.html")]
pub struct BlogPostTemplate {
    pub base: BaseTemplateContext,
    pub post: Post,
    pub content_html: String,
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
#[template(path = "pages/admin.html")]
pub struct AdminTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/admin/posts.html")]
pub struct AdminPostsTemplate {
    pub base: BaseTemplateContext,
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "pages/admin/post_new.html")]
pub struct AdminNewPostTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/admin/post_edit.html")]
pub struct AdminEditPostTemplate {
    pub base: BaseTemplateContext,
    pub post: Post,
}
