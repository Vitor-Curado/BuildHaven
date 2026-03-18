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

## Frontend

* HTML
* Raw CSS (no frameworks)
* Fully server-rendered pages

## Infrastructure

* **Server OS:** Fedora Linux
* **Reverse proxy:** Nginx
* **Container engine:** Podman
* **Container orchestration:** Podman Compose
* **Container runtime:** Distroless

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
Application State
   ↓
Askama Template
   ↓
HTML Response
   ↓
Browser
```

---

# 🧩 Project Modules

* **api.rs** — JSON response structures (API responses)
* **compresser.rs** —cache busting, compression of static files
* **config.rs** — environment-based application configuration 
* **handlers.rs** — HTTP request handlers
* **lib.rs** — crate module declarations
* **main.rs** — server initialization and runtime setup
* **models.rs** — domain data structures
* **repository.rs** — mock data provider
* **router.rs** — Axum router configuration
* **state.rs** — shared application state
* **templates.rs** — Askama template bindings
* **utils.rs** — utility helpers (markdown conversion, file loading)
* **tests.rs** — integration tests

---

# 📁 Project Structure

```
├── scripts
│   ├── check.sh
│   ├── deploy.sh
│   └── inspect.sh
│
├── src
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
│   ├── media
│   │   ├── food
│   │   ├── icons
│   │   └── languages
│   │
│   ├── js
│   │   ├── navbar.js
│   │   └── languages
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

---

# ✨ Features

**Server Architecture**
* Server-side rendered HTML using Askama templates
* Modular Axum router and handler architecture
* Shared application state container
* Environment-driven configuration

**Performance**
* Gzip compression for HTTP responses
* Efficient static file serving
* Support for pre-compressed static assets
* Multi-stage container builds with cargo-chef for faster rebuilds
* JavaScript-free

**Observability**
* Structured request tracing via tower-http
* Configurable logging through tracing and RUST_LOG

**Security**
* Secure HTTP headers:
   * X-Frame-Options
   * X-Content-Type-Options
   * Referrer-Policy
* Distroless container runtime for reduced attack surface

**Infrastructure**
* Containerized deployment using Podman
* Orchestrated with Podman Compose
* Reverse-proxied through Nginx
* Automated deployment using Woodpecker CI

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