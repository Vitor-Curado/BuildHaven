// Note: we are explicitly not minifying.
// This is because the CSS/JS content is already
// very small and minification would not save much space,
// but it would add unnecessary complexity.
// Also: cleaning dist is a design choice.

use crate::{
    constants::paths,
    error::{AppError, AppResult},
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs, path::Path, str};
use usvg::{Options, Tree, WriteOptions};

#[derive(Serialize, Clone)]
pub struct Assets {
    pub css: String,
    pub js: String,
    pub icons: HashMap<String, String>,
}

impl Assets {
    pub fn new() -> Result<Self, AppError> {
        let manifest = load_manifest()?;

        let mut icons = HashMap::new();
        for (key, value) in &manifest {
            if key.starts_with("icons/") {
                let name = key.trim_start_matches("icons/").to_string();
                icons.insert(name.to_string(), value.clone());
            }
        }

        Ok(Self {
            css: manifest.get("index.css").ok_or(AppError::Internal)?.clone(),

            js: manifest.get("app.js").ok_or(AppError::Internal)?.clone(),

            icons,
        })
    }

    /// # Errors
    /// Returns an error if file writing fails.
    pub fn build() -> Result<Self, AppError> {
        // clean dist
        if Path::new(paths::DIST).exists() {
            fs::remove_dir_all(paths::DIST)?;
        }
        fs::create_dir_all(paths::DIST)?;

        // CSS
        let css = bundle_css()?;
        let css_hash = hash_content(&css);
        let css_filename = format!("index-{}.css", &css_hash[..12]);
        let css_path = Path::new(paths::DIST).join(&css_filename);
        fs::write(css_path, css)?;

        // JS
        let js = bundle_js()?;
        let js_hash = hash_content(&js);
        let js_filename = format!("app-{}.js", &js_hash[..12]);
        let js_path = Path::new(paths::DIST).join(&js_filename);
        fs::write(js_path, js)?;

        // Manifest
        let mut manifest = HashMap::new();
        manifest.insert("index.css".to_string(), css_filename);
        manifest.insert("app.js".to_string(), js_filename);

        // Images
        process_images(&mut manifest)?;

        write_manifest(&manifest)?;
        Self::new()
    }

    pub fn icon(&self, name: &str) -> String {
        self.icons
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_string())
    }
}

fn bundle_css() -> std::io::Result<String> {
    let css_bundle = collect_files(paths::CSS)?;
    let mut result = String::new();

    for file in css_bundle {
        result.push_str(&fs::read_to_string(file)?);
        result.push('\n');
    }

    Ok(result)
}

fn bundle_js() -> std::io::Result<String> {
    let js_files = vec![paths::JS_ENTRY];

    let mut result = String::new();

    for file in js_files {
        result.push_str(&fs::read_to_string(file)?);
        result.push('\n');
    }

    Ok(result)
}

fn process_images(manifest: &mut HashMap<String, String>) -> AppResult<()> {
    let src_dir = Path::new(paths::ICONS_SRC);
    let dest_dir = Path::new(paths::ICONS_DIST);

    fs::create_dir_all(dest_dir)?;

    let mut entries: Vec<_> = fs::read_dir(src_dir)?.filter_map(|e| e.ok()).collect();

    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();

        if path.is_file() {
            let content = fs::read(&path)?;

            let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");

            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            match extension {
                "svg" | "png" | "jpg" | "jpeg" | "ico" => {}
                _ => continue,
            }

            let optimized_bytes = match extension {
                "svg" => match optimize_svg(&content) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        tracing::warn!(error = %e,
                        path = ?path,
                        "SVG optimization failed, using original"
                        );
                        content
                    }
                },
                _ => content,
            };

            let hash = hash_bytes(&optimized_bytes);

            let hashed_filename = format!("{}-{}.{}", filename, &hash[..12], extension);

            let dest_path = dest_dir.join(&hashed_filename);

            fs::write(dest_path, &optimized_bytes)?;

            manifest.insert(format!("icons/{}.{}", filename, extension), hashed_filename);
        }
    }

    Ok(())
}

fn optimize_svg(content: &[u8]) -> AppResult<Vec<u8>> {
    // Convert bytes → UTF-8 string
    let svg_str = std::str::from_utf8(content).map_err(|e| AppError::Other {
        message: "Invalid UTF-8 in SVG",
        source: Box::new(e),
    })?;

    // Default options
    let options = Options::default();

    // Parse SVG
    let tree = Tree::from_str(svg_str, &options).map_err(|e| AppError::Other {
        message: "Failed to parse SVG",
        source: Box::new(e),
    })?;

    // Serialize optimized SVG
    let write_options = WriteOptions::default();
    let optimized_string = tree.to_string(&write_options);

    Ok(optimized_string.into_bytes())
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);

    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_bytes(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

fn write_manifest(manifest: &HashMap<String, String>) -> AppResult<()> {
    let json = serde_json::to_string_pretty(manifest)?;

    let manifest_path = Path::new(paths::DIST).join("manifest.json");

    fs::write(manifest_path, json)?;

    Ok(())
}

pub fn load_manifest() -> Result<HashMap<String, String>, AppError> {
    let content = std::fs::read_to_string(format!("{}/manifest.json", paths::DIST))
        .map_err(|_| AppError::Internal)?;

    let manifest: HashMap<String, String> = serde_json::from_str(&content).map_err(|e| {
        tracing::error!(error = %e, "Failed to read manifest");
        AppError::Internal
    })?;

    Ok(manifest)
}

fn visit(path: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit(&path, files)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("css") {
            files.push(path.to_string_lossy().to_string());
        }
    }

    Ok(())
}

fn collect_files(dir: &str) -> std::io::Result<Vec<String>> {
    let mut files = vec![];

    visit(Path::new(dir), &mut files)?;
    files.sort();
    Ok(files)
}
