# 📐 Architecture Overview

## 🧠 Core Application

- **Language:** Rust
- **Web Framework:** Axum
- **Async Runtime:** Tokio
- **Templating Engine:** Askama
- **Serialization:** Serde
- **Database:** PostgreSQL
- **Database Layer:** SQLx
- **Migration System:** SQLx migrations

The application uses a shared `AppState`, initialized at startup, injected into all request handlers.  
This keeps request handlers lightweight and fast.

---

## ⚙️ Application State

The application initializes a shared `AppState` at startup containing:

- Database connection pool
- Configuration data
- Static asset manifest mappings
- Pre-rendered Markdown content (README → HTML)
- In-memory mock or cached data (food database)

This avoids repeated computation during request handling and keeps handlers lightweight.

---

## 🗃️ Persistence Layer

Database operations are separated into:

- Repository layer — SQL queries
- Service layer — business logic
- Models — domain data structures

This separation improves maintainability and testability.

---

## 🧭 High-Level Flow

```
Browser
↓
Nginx
↓
Axum Router
↓
Middleware Stack
↓
Handlers
↓
AppState
↓
Service Layer
↓
Repository Layer
↓
PostgreSQL
↓
Askama Templates
↓
HTML Response
```

---

## Middleware Stack Order

Middleware execution order:

1. Security Headers
2. Logging
3. CORS
4. Rate Limiting
5. Compression
6. Routing

Order matters.

Each layer builds upon the previous one.

---

---

## Startup Initialization Flow

```
Program Start
↓
Load Configuration
↓
Create Database Pool
↓
Load Asset Manifest
↓
Parse README Markdown
↓
Load Cached Data
↓
Build AppState
↓
Start HTTP Server

```