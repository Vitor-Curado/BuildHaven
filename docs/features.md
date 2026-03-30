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

| Feature                  | Library / Tool              |
|--------------------------|-----------------------------|
| Server-side rendering    | Askama, Askama-Axum         |
| Async runtime            | Tokio                       |
| Web framework            | Axum                        |
| JSON serialization       | Serde, serde_json           |
| Database layer           | SQLx, PostgreSQL            |
| Rate limiting            | tower-governor              |
| Environment config       | dotenvy                     |
| Logging & tracing        | tracing, tracing-subscriber |
| Compression & static     | tower-http                  |
| Asset pipeline           | walkdir, sha2, once_cell    |
| Markdown rendering       | pulldown-cmark              |
| Unique IDs / timestamps  | uuid, chrono                |

---