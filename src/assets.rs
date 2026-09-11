use crate::{
    constants::paths::VITE_MANIFEST,
    error::{AppError, AppResult},
};
use serde::Deserialize;
use std::fs;

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
}
