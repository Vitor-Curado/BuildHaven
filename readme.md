# 📘 Victor's Personal Website

**Version:** 0.4.1
**Last updated:** March 30, 2026

A small Rust web application serving my personal website.
The project uses server-side rendering with Askama, runs inside containers with Podman, and is deployed on a Fedora Linux server behind Nginx.

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

## Supporting Assets

* Static media files
* HTML templates
* CSS stylesheets
* Shell scripts for deployment and maintenance

## 📦 Supporting Assets

The project includes non-code resources required at runtime or deployment.

### Includes

* Static media files
* HTML templates
* CSS stylesheets
* JavaScript utilities
* SQL migrations
* Shell scripts for deployment and maintenance

---