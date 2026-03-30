# 🏗️ Asset Pipeline

Static assets are processed at build time using a **custom Rust-based pipeline**.

## Processing Steps

1. **Bundling** —  Combines CSS and JavaScript into unified output files.
2. **Minification** — removes whitespace and unnecessary characters
3. **Fingerprinting** — content-based hashing (e.g. `index-[hash].css`, `app-[hash].css`)
4.  **Manifest generation** — produces manifest.json, maps logical names to hashed files
5. **Precompression** — generates compressed assets (`.gz`, `.br`) for efficient delivery

The generated files are placed in:

static/dist/

At runtime, the application loads `manifest.json` to resolve the correct asset filenames.

---