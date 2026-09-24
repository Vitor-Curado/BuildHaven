use crate::{
    constants::{icons, paths::VITE_MANIFEST, titles},
    error::{AppError, AppResult},
    navbar::DOCS,
    templates::{BaseTemplateContext, ContactTemplate, DocsTemplate},
    utils::markdown_to_html,
};
use askama::Template;
use serde::Deserialize;
use std::{fs, sync::Arc};

#[derive(Debug, Deserialize)]
struct ViteManifestEntry {
    // The file is actually the JS, don't blame me blame Vite
    file: String,
    #[serde(default)]
    css: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ViteManifest {
    #[serde(rename = "index.html")]
    index: ViteManifestEntry,
}

#[derive(Clone)]
pub struct Assets {
    pub css: String,
    pub js: String,
}

impl Assets {
    pub fn new() -> AppResult<Self> {
        let content = fs::read_to_string(VITE_MANIFEST)?;
        let manifest: ViteManifest = serde_json::from_str(&content)?;

        Ok(Self {
            css: manifest
                .index
                .css
                .first()
                .cloned()
                .ok_or(AppError::MissingAsset("index.css"))?,
            js: manifest.index.file,
        })
    }

    pub fn generate_all_static_pages(&self) -> AppResult<()> {
        self.generate_contact()?;
        self.generate_docs()
    }

    fn generate_contact(&self) -> AppResult<()> {
        let base = BaseTemplateContext::new(
            titles::CONTACT,
            icons::CONTACT,
            Arc::new(self.clone()),
            DOCS,
        );
        let template = ContactTemplate { base };

        self.write_static_page("contact", template)
    }

    fn generate_docs(&self) -> AppResult<()> {
        for doc in DOCS {
            let base =
                BaseTemplateContext::new(doc.title, icons::DOCS, Arc::new(self.clone()), DOCS);

            let html = markdown_to_html(doc.markdown);

            let template = DocsTemplate {
                base,
                title: doc.title,
                content_html: html,
            };

            self.write_static_page(&format!("docs/{}", doc.slug), template)?;
        }

        Ok(())
    }

    fn write_static_page<T: Template>(&self, route: &str, template: T) -> AppResult<()> {
        let directory = format!("static/{route}");
        fs::create_dir_all(&directory)?;
        let html = template.render()?;
        fs::write(format!("{directory}/index.html"), html)?;

        Ok(())
    }
}
