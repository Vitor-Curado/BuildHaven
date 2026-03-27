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
* **Database:** PostgreSQL
* **Database Layer:** SQLx

---

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

---

## 🗄️ Data Layer

Persistent storage is handled through **PostgreSQL** using **SQLx** for compile-time verified queries.

### Responsibilities

* Database connection pooling
* Query execution via SQLx
* Schema migrations
* Data modeling
* Optional seed data loading

---

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

# 📐 Architecture

## 🧠 Core Application

**Language:** Rust
**Web Framework:** Axum
**Async Runtime:** Tokio
**Templating Engine:** Askama
**Serialization:** Serde
**Database:** PostgreSQL
**Database Layer:** SQLx
**Migration System:** SQLx migrations

The application is structured around a shared `AppState`, initialized at startup and injected into request handlers.

---

## 🧱 Application State

A shared `AppState` is initialized during server startup.

It includes:

* Database connection pool
* Static asset manifest mappings
* Pre-rendered Markdown content (README → HTML)
* In-memory mock or cached data (when applicable)

This approach minimizes repeated computation during request handling and keeps handlers lightweight.

---

# 🎨 Front-End Architecture

The front-end follows a **framework-free, component-oriented CSS structure** built around reusable primitives.

### Design Philosophy

* No front-end framework dependency
* Token-driven styling
* Modular components
* Page isolation
* Predictable cascade layers

---

## 🎨 Theme System

The UI supports multiple visual themes powered entirely by CSS variables.

### Behavior

* Controlled via `data-theme` on `<html>`
* Persisted using `localStorage`
* Updated via lightweight JavaScript (`javascript.js`)
* Fully CSS-variable driven

### Available Themes

* Night *(default)*
* Sunset
* Ocean
* Forest

---

# 🏗️ Asset Pipeline

Static assets are processed at build time using a **custom Rust-based pipeline**.

## Processing Steps

1. **Bundling**
   Combines CSS and JavaScript into unified output files.

2. **Minification**
   Removes unnecessary whitespace and characters.

3. **Fingerprinting**
   Generates content-hashed filenames:

```
index-[hash].css
app-[hash].js
```

4. **Manifest Generation**
   Produces:

```
manifest.json
```

Mapping logical names to fingerprinted files.

5. **Precompression**
   Generates compressed assets:

* gzip (`.gz`)
* optional Brotli (`.br` — recommended)

---

## Output Directory

```
static/dist/
```

At runtime, the application loads:

```
manifest.json
```

to resolve correct asset filenames.

---

# 🖥️ Infrastructure

The application is deployed using containerized Linux infrastructure.

## Host Environment

* **Operating System:** Fedora Linux
* **Service Management:** systemd
* **Reverse Proxy:** Nginx

---

## Containerization

* **Container Engine:** Podman
* **Container Runtime:** Podman
* **Base Image:** Distroless
* **Image Registry:** GitHub Container Registry (GHCR)

Container images are built using multi-stage builds for minimal runtime footprint.

---

# 🔄 CI / Tooling

Continuous integration and development tooling ensure consistent builds and code quality.

## CI Pipeline

* **CI System:** Woodpecker CI

---

## Rust Tooling

Formatting:

```
cargo fmt
```

Linting:

```
cargo clippy --pedantic
```

Testing:

```
cargo test
```

---

## Container Builds

Uses a multi-stage Dockerfile with:

```
cargo chef
```

to optimize build caching and dependency reuse.

---

# 📦 Supporting Assets

The project includes non-code resources required at runtime or deployment.

## Includes

* Static media files
* HTML templates
* CSS stylesheets
* JavaScript utilities
* SQL migrations
* Shell scripts for deployment and maintenance

---