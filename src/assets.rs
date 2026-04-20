// Note: we are explicitly not minifying.
// This is because the CSS/JS content is already
// very small and minification would not save much space,
// but it would add unnecessary complexity.
// Also: cleaning dist is a design choice.

use crate::error::AppError;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs, path::Path};

#[derive(Serialize, Clone)]
pub struct Assets {
    pub css: String,
    pub js: String,
}

impl Assets {
    pub fn new() -> Result<Self, AppError> {
        // Load assets manifest
        let manifest = load_manifest()?;

        Ok(Self {
            css: manifest
                .get("index.css")
                .ok_or(AppError::Internal)?
                .clone(),

            js: manifest.get("app.js").ok_or(AppError::Internal)?.clone(),
        })
    }
}

/// # Errors
/// Returns an error if file writing fails.
pub fn build_assets() -> std::io::Result<()> {
    // clean dist
    if Path::new("static/dist").exists() {
        fs::remove_dir_all("static/dist")?;
    }
    fs::create_dir_all("static/dist")?;

    // CSS
    let css = bundle_css()?;
    let css_hash = hash_content(&css);
    let css_filename = format!("index-{}.css", &css_hash[..12]);
    fs::write(format!("static/dist/{css_filename}"), css)?;

    // JS
    let js = bundle_js()?;
    let js_hash = hash_content(&js);
    let js_filename = format!("app-{}.js", &js_hash[..12]);
    fs::write(format!("static/dist/{}", js_filename), js)?;

    // Manifest
    let mut manifest = HashMap::new();
    manifest.insert("index.css".to_string(), css_filename);
    manifest.insert("app.js".to_string(), js_filename);

    // Images
    process_images(&mut manifest)?;

    write_manifest(&manifest)?;
    Ok(())
}

fn bundle_css() -> std::io::Result<String> {
    let css_bundle = collect_files("static/css")?;
    let mut result = String::new();

    for file in css_bundle {
        result.push_str(&fs::read_to_string(file)?);
        result.push('\n');
    }

    Ok(result)
}

fn bundle_js() -> std::io::Result<String> {
    let js_files = vec!["static/js/javascript.js"];

    let mut result = String::new();

    for file in js_files {
        result.push_str(&fs::read_to_string(file)?);
        result.push('\n');
    }

    Ok(result)
}

fn process_images(manifest: &mut HashMap<String, String>) -> std::io::Result<()> {
    let src_dir = Path::new("static/media/icons");
    let dest_dir = Path::new("static/dist/icons");

    fs::create_dir_all(dest_dir)?;

    for entry in fs::read_dir(src_dir)? {
        let path = entry?.path();

        if path.is_file() {
            let content = fs::read(&path)?;

            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("file.png");

            let dest_path = dest_dir.join(filename);

            fs::write(dest_path, content)?;

            // Optional manifest entry (useful later)
            manifest.insert(format!("icons/{}", filename), format!("icons/{}", filename));
        }
    }

    Ok(())
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

fn write_manifest(manifest: &HashMap<String, String>) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(manifest)?;
    fs::write("static/dist/manifest.json", json)
}

pub fn load_manifest() -> Result<HashMap<String, String>, AppError> {
    let content =
        std::fs::read_to_string("static/dist/manifest.json").map_err(|_| AppError::Internal)?;

    serde_json::from_str(&content).map_err(|_| AppError::Internal)
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
