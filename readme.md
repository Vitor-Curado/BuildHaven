# 📘 Victor's Personal Website

**Version:** 0.4.
**Last updated:** September 08, 2026  

A containerized Rust web application serving a personal website with server-side rendering, persistent storage, and a custom asset pipeline.

Built using modern Rust tooling and deployed on a Fedora Linux server behind Nginx.

---

## 🧠 Core Stack

- **Language:** Rust
- **Database:** PostgreSQL (SQLx)
- **Deployment:** Podman 
- **CI/CD:** WIP
- **Reverse Proxy:** Nginx
- **Benchmarking:** wrk
- **Observability:** VictoriaMetrics + Prometheus + Grafana
- **Object storage:** AWS S3
- **Content Delivery Network:** AWS CloudFront (planned)
- **Authentication:** OAuth2 (planned)
- **Tracing:** OpenTelemetry (planned)

---

## 🚀 Highlights

- Layered Rust web architecture
- Server-side rendered HTML
- Database-backed blog system
- Compile-time verified SQL queries
- Custom Rust asset pipeline
- Containerized production deployment
- Structured logging and tracing
- Security-focused HTTP configuration
- Production rate limiting via tower-governor
- Deterministic build and deployment workflow

## 📁 Key Capabilities

- HTTP routing via modular Axum layers
- Runtime asset resolution through manifest loading
- Database-backed content rendering
- In-memory shared application state
