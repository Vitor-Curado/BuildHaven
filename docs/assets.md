# 🏗️ Asset Pipeline

Static assets are processed at build time using a **custom Rust-based pipeline**.

## Processing Steps

1. **Bundling** —  Combines CSS and JavaScript into unified output files.
2. **Minification** — due to complexity and resource (mainly time) constraints, this project does not minify its contents at all.
3. **Fingerprinting** — content-based hashing (e.g. `index-[hash].css`, `app-[hash].css`)
4. **Manifest generation** — produces manifest.json, maps logical names to hashed files
5. **Precompression Support** — supports serving precompressed assets (`.gz`)

The generated files are placed in:

static/dist/

At runtime, the application loads `manifest.json` to resolve the correct asset filenames.

---
