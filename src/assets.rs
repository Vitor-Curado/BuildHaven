/* This file's functions are:
* bundling: merge JS/CSS into fewer files
* minification: remove whitespace, comments, etc
* fingerprinting (hashing): index-[sha512].css
* compression: gzip/brotli versions
*/

use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Assets {
    pub css: String,
    pub js: String
}

pub fn build_assets() -> std::io::Result<()> {
    // clean dist
    if Path::new("static/dist").exists() {
        fs::remove_dir_all("static/dist")?;
    }
    fs::create_dir_all("static/dist")?;

    // CSS
    let css = bundle_css()?;
    let css_minified = minify_css(&css);
    let css_hash = hash_content(&css_minified);
    let css_filename = format!("index-{}.css", &css_hash[..12]);
    fs::write(format!("static/dist/{}", css_filename), css_minified)?;

    // JS
    let js = bundle_js()?;
    let js_minified = minify_css(&js);
    let js_hash = hash_content(&js_minified);
    let js_filename = format!("app-{}.js", &js_hash[..12]);
    fs::write(format!("static/dist/{}", js_filename), js_minified)?;

    // Manifest
    let mut manifest = HashMap::new();
    manifest.insert("index.css".to_string(), css_filename);
    manifest.insert("app.js".to_string(), js_filename);

    write_manifest(&manifest)?;

    Ok(())
}

fn bundle_css() -> std::io::Result<String> {
    let css_bundle = vec![
        "static/css/variables.css",
        "static/css/themes.css",
        "static/css/base.css",
        "static/css/typography.css",
        "static/css/components/navbar.css",
        "static/css/components/dropdown.css",
        "static/css/components/containers.css",
        "static/css/components/grids.css",
        "static/css/components/cards.css",
        "static/css/components/buttons.css",
    ];

    let mut result = String::new();

    for file in css_bundle {
        result.push_str(&fs::read_to_string(file)?);
    }

    Ok(result)
}

fn bundle_js() -> std::io::Result<String> {
    let js_files = vec![
        "static/js/theme.js",
        "static/js/navbar.js",
    ];

    let mut result = String::new();

    for file in js_files {
        result.push_str(&fs::read_to_string(file)?);
    }

    Ok(result)
}

fn minify_css(input: &str) -> String {
    input
        .lines()
        .map(str::trim)
        .collect::<String>()
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

pub fn load_manifest() -> HashMap<String, String> {
    let content = std::fs::read_to_string("static/dist/manifest.json")
        .expect("manifest missing");

    serde_json::from_str(&content)
        .expect("invalid manifest")
}