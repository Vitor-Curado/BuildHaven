# 📘 Victor's Personal Website

**Version:** 0.2
**Last updated:** March 8, 2026

A small Rust web application serving my personal website.
The project uses server-side rendering with Askama, runs inside containers with Podman, and is deployed on a Fedora Linux server behind Nginx.

---

# 📐 Architecture

## Application Layer

* **Language:** Rust
* **Web framework:** Axum
* **Async runtime:** Tokio
* **Server-side templating:** Askama

## Frontend

* HTML
* Raw CSS (no frameworks)

## Infrastructure

* **Server OS:** Fedora Linux
* **Reverse proxy:** Nginx
* **Container engine:** Podman
* **Container orchestration:** Docker Compose

## CI / Tooling

* **CI pipeline:** Woodpecker CI
* **Formatting:** `cargo fmt`
* **Linting:** `cargo clippy --pedantic`
* **Tests:** `cargo test`

## Supporting Assets

* Static media files
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
Data Layer
   ↓
Askama Template
   ↓
HTML Response
   ↓
Browser
```

---

# 🧩 Project Modules

* **main.rs** — server startup and routing
* **handlers.rs** — request handlers
* **models.rs** — domain data structures
* **templates.rs** — Askama template bindings
* **data.rs** — mock data provider
* **api.rs** — JSON response types
* **tests.rs** — unit and integration tests

---

# 📁 Project Structure

```
.
├── src
│   ├── api.rs
│   ├── handlers.rs
│   ├── main.rs
│   ├── models.rs
│   ├── repository.rs
│   ├── templates.rs
│   └── tests.rs
│
├── templates
│   ├── base.html
│   ├── index.html
│   ├── blog.html
│   ├── boardgames.html
│   ├── food.html
│   ├── food_detail.html
│   ├── assets.html
│   ├── apps.html
│   ├── contact_me.html
│   └── resume.html
│
├── static
│   ├── css
│   │   ├── base-style.css
│   │   ├── contact-style.css
│   │   ├── food-style.css
│   │   ├── food-detail-style.css
│   │   └── resume-style.css
│   │
│   └── media
│       ├── food
│       ├── icons
│       └── languages
│
├── scripts
│   └── deploy.sh
│
├── docs
│   ├── architecture.md
│   └── features.md
│
├── Dockerfile
├── docker-compose.yml
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── .dockerignore
└── README.md
```

---

# ✨ Features

* Containerized deployment with Podman
* Server-side rendered pages with Askama
* Static assets (CSS, images)
* Unit testing
* Integration testing

---

# 🛣️ Roadmap

Planned improvements:

* Introduce shared **AppState**
* Add a **database layer**
* Implement a `/health` endpoint
