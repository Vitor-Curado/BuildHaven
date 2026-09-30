# 📘 Victor's Personal Website

**Version:** 0.5.1
**Last updated:** September 30, 2026  

A personal blog, made with the help of ChatGPT, no I didn't use agents nor have I ever touched one. No, I don't know what I'm doing and I'm learning as I go.

---

## 🧠 Core Stack

- **Languages:** Rust, TypeScript
- **Database:** PostgreSQL
- **Containerisation:** Podman
- **Orchestration (dev + prod):** Podman-compose + Podman Quadlets
- **Reverse Proxy:** Nginx
- **Benchmarking:** wrk
- **Observability:** VictoriaMetrics + Prometheus + Grafana
- **Content Delivery Network:** Cloudfare (I'm pretty sure it's working)
- **Authentication:** Session-based
- **Operating Systems (dev + prod):** Fedora Linux + RHEL 10
- **Cloud:** AWS EC2 t4g.small, ARM64

## 😎 Capabilities

- Has static pages, moving compute from runtime to compile time and overall reducing costs, but yet to be moved away from the web framework entirely and have the responsibility passed to the reverse proxy which could remove even more responsibility from the application
- Has a functioning login and CMS going on
- Has Vite cache-busting, minifying, merging the styling, which is shared with the SSG engine and with the SSR engine, a nice little touch
- Has a functioning observability subdomain 