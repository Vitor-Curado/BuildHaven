# 📘 Victor's Personal Website

**Version:** 0.4
**Last updated:** March 19, 2026

A small Rust web application serving my personal website.
The project uses server-side rendering with Askama, runs inside containers with Podman, and is deployed on a Fedora Linux server behind Nginx.

---

# 📐 Architecture

## Application Layer

* **Language:** Rust
* **Web framework:** Axum
* **Async runtime:** Tokio
* **Server-side templating:** Askama
* **Serialization:** Serde

## Front-end

The front-end uses a structured, framework-free CSS system inspired by component-based design:

* Design tokens defined in variables.css (colors, spacing, typography, shadows)
* Theme system via themes.css using [data-theme] attributes
* Component styles (components/) for reusable UI elements
* Page-specific styles (pages/) for isolated layout concerns
* Base + typography layers for global consistency

### 🎨 Theme System

The application supports multiple visual themes:

* Controlled via data-theme attribute on <html>
* Persisted in localStorage
* Updated via lightweight JavaScript (theme.js)
* Fully driven by CSS variables

Themes include:

* Night (default)
* Sunset
* Ocean
* Forest

## Infrastructure

* **Server OS:** Fedora Linux
* **Reverse proxy:** Nginx
* **Container engine:** Podman
* **Container runtime:** Podman
* **Service management:** systemd
* **Image registry:** GitHub Container Registry (GHCR)
* **Container image:** Distroless

## CI / Tooling

* **CI pipeline:** Woodpecker CI
* **Formatting:** `cargo fmt`
* **Linting:** `cargo clippy --pedantic`
* **Tests:** `cargo test`
* **Container builds:** Multi-stage Dockerfile with `cargo chef`

## Supporting Assets

* Static media files
* HTML templates
* CSS stylesheets
* Shell scripts for deployment and maintenance

---

# ✨ Features

**Server Architecture**
* Server-side rendered HTML using Askama templates
* Modular Axum router and handler architecture
* Shared application state container
* Environment-driven configuration

**Performance**
* Gzip compression for HTTP responses
* Static files served via `tower-http::ServeDir`
* Cache-Control headers for client-side caching
* Support for pre-compressed static assets
* Multi-stage container builds with cargo-chef for faster rebuilds
* Very few JavaScript

**Observability**
* Structured request tracing via tower-http
* Configurable logging through tracing and RUST_LOG

**Security**
* Secure HTTP headers:
   * Content-Security-Policy (CSP)
   * Strict-Transport-Security (HSTS)
   * X-Frame-Options
   * X-Content-Type-Options
   * Referrer-Policy
   * Permissions-Policy
   * Cross-Origin-Resource-Policy
* Distroless container runtime for reduced attack surface
* Runs as non-root user for improved security

**Infrastructure**
* Containerized deployment using Podman
* Orchestrated with Podman Compose
* Reverse-proxied through Nginx
* Automated deployment using Woodpecker CI
* Multi-stage builds using `cargo-chef` for dependency caching
* Minimal distroless runtime image

**API**
* JSON health check endpoint
`GET /health`
* Returns service metadata and runtime status.

**Content System**
* Markdown rendering using pulldown-cmark
* README content dynamically rendered on the homepage
* Structured content models (example: food database)

**Testing**
* Unit tests
* Integration tests
* Automated verification via CI pipeline

---

# 🎨 Asset Pipeline

Static assets are processed at build time using a custom Rust pipeline:

* **Bundling** — merges CSS and JS into single files
* **Minification** — removes whitespace and unnecessary characters
* **Fingerprinting** — content-based hashing (e.g. `index-[hash].css`)
* **Manifest generation** — maps logical names to hashed files
* **Precompression** — supports gzip assets for efficient delivery

The generated files are placed in:

static/dist/

At runtime, the application loads `manifest.json` to resolve the correct asset filenames.

---

# ⚙️ Application State

The application initializes a shared `AppState` at startup containing:

* Pre-rendered README HTML (Markdown → HTML)
* Static asset mappings (via manifest.json)
* In-memory mock data (food database)

This avoids repeated computation during request handling.

---

## 🚀 Deployment Pipeline

The application is deployed using a registry-driven container workflow.

### Flow Overview

```
Developer pushes to GitHub (main branch)
        ↓
Woodpecker CI runs checks:
    - cargo fmt
    - cargo clippy
    - cargo test
        ↓
Container image is built using Dockerfile
        ↓
Image is pushed to GitHub Container Registry (GHCR)
        ↓
Production server pulls latest image
        ↓
systemd restarts container
        ↓
Nginx serves updated application
```

---

## 🧰 Production Runtime

The production server uses:

* Podman for container runtime
* systemd for container lifecycle management
* GitHub Container Registry (GHCR) as image source
* Nginx as reverse proxy

The systemd unit automatically pulls the latest image before starting:

```ini
ExecStartPre=/usr/bin/podman pull ghcr.io/vitor-curado/personal-website:latest
```

---

# 🛣️ Roadmap

Planned improvements:

**Content**
* Populate blog and project sections
* Expand content models

**Back-end**
* Add a database layer
* Replace mock repositories with persistent storage
* Introduce authentication and user sessions

**Front-end**
* Build a modular CSS system
* Improve layout and typography
* Introduce reusable UI components

**Internationalization**
* Multi-language support

---

# 🧩 Project Modules

* **src/bin/assets.rs** — build assets
* **src/api.rs** — JSON response structures (API responses)
* **src/assets.rs** — cache busting, compression of static files
* **src/config.rs** — environment-based application configuration 
* **src/handlers.rs** — HTTP request handlers
* **src/lib.rs** — crate module declarations
* **src/main.rs** — server initialization and runtime setup
* **src/models.rs** — domain data structures
* **src/repository.rs** — mock data provider
* **src/router.rs** — Axum router middleware and global configuration
* **src/routes.rs** — route definitions
* **src/state.rs** — shared application state
* **src/templates.rs** — Askama template bindings
* **src/utils.rs** — utility helpers (markdown conversion, file loading)
* **tests/tests.rs** — integration tests

---

# 🔁 Request Flow

```
Browser
   ↓
Nginx (reverse proxy)
   ↓
Axum Router
   ↓
Request Handler
   ↓
Application State (preloaded: README HTML, assets, data)
   ↓
Askama Template
   ↓
HTML Response
   ↓
Browser
```

---

# 📁 Project Structure

```
├── docs
│   ├── deployment.md
│   ├── features.md
│   ├── modules.md
│   ├── roadmap.md
│   └── structure.md
│
├── scripts
│   ├── check.sh
│   ├── deploy.sh
│   └── inspect.sh
│
├── src
│   ├── bin
│   │   └── assets.rs
│   │
│   ├── api.rs
│   ├── compresser.rs
│   ├── config.rs
│   ├── handlers.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── models.rs
│   ├── repository.rs
│   ├── router.rs
│   ├── state.rs
│   ├── templates.rs
│   └── utils.rs
│
├── static
│   ├── css
│   │   ├── components
│   │   │   ├── buttons.css
│   │   │   ├── cards.css
│   │   │   ├── containers.css
│   │   │   ├── dropdown.css
│   │   │   ├── grids.css
│   │   │   └── navbar.css
│   │   │
│   │   ├── pages
│   │   │   ├── contact.css
│   │   │   ├── food-detail.css
│   │   │   └── food.css
│   │   │
│   │   ├── base.css
│   │   ├── layout.css
│   │   ├── navbar.css
│   │   ├── themes.css
│   │   ├── typography.css
│   │   └── variables.css
│   │
│   ├── dist
│   │   ├── app-[sha256].js
│   │   ├── index-[sha256].css
│   │   └── manifest.json
│   │
│   ├── js
│   │   ├── navbar.js
│   │   └── theme.js
│   │
│   └── media
│       ├── food
│       ├── icons
│       └── languages
│   
├── templates
│   ├── pages
│   │   ├── apps.html
│   │   ├── assets.html
│   │   ├── blog.html
│   │   ├── boardgames.html
│   │   ├── contact_me.html
│   │   ├── food_detail.html
│   │   ├── food.html
│   │   ├── index.html
│   │   └── resume.html
│   │
│   ├── partials
│   │   └── navbar.html
│   │
│   └── base.html
│
├── tests
│   └── tests.rs
│
├── .dockerignore
├── .gitignore
├── .woodpecker.yml
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml
├── Dockerfile
└── readme.md
```