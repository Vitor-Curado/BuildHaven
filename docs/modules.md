# 🧩 Project Modules

## Core Application

**main.rs**  
Initializes the server, middleware, and runtime.

**lib.rs**  
Declares crate modules and shared interfaces.

---

## Runtime Infrastructure

**config.rs**  
Loads environment-driven configuration.

**state.rs**  
Defines shared `AppState`.

**pool.rs**  
Initializes database connection pool.

**logging.rs**  
Configures structured logging and tracing.

---

## HTTP Layer

**router.rs**  
Builds middleware stack.

**routes.rs**  
Defines route mappings.

**handlers.rs**  
Processes HTTP requests.

**cors.rs**  
Applies cross-origin policies.

**rate_limit.rs**  
Applies request throttling.

**security.rs**  
Applies HTTP security headers.

---

## Domain Layer

**models.rs**  
Domain data structures.

**services.rs**  
Business logic operations.

**repository.rs**  
Database queries and persistence logic.

---

## Asset System

**assets.rs**  
Handles runtime asset resolution.

**bin/assets.rs**  
Build-time asset pipeline.

---

## Templates

**templates.rs**  
Template bindings and rendering helpers.

---

## Utilities

**utils.rs**  
General-purpose helpers.

**api.rs**  
JSON response structures.

**error.rs**  
Application error types.

---

## Development Tools

**indexer.rs**  
Build-time searchable file index generator.

---

## Testing

**tests.rs**  
Integration tests.

---

## Authentication & Sessions

**auth.rs**  
Authentication logic and credential validation.

**session.rs**  
Session lifecycle management and cookie handling.

---

## Application Lifecycle

**bootstrap.rs**  
Startup initialization routines.

**shutdown.rs**  
Graceful shutdown handling.

---

## Background Processing

**jobs.rs**  
Background task execution and scheduling.

---

## Observability

**metrics.rs**  
Prometheus metrics definitions.

**telemetry.rs**  
Tracing and OpenTelemetry integration.

---

## Middleware Composition

**middleware.rs**  
Centralized middleware configuration.

---

## Core Context

**context.rs**  
Request-scoped context utilities.

**constants.rs**  
Shared application constants.

**content.rs**  
Content loading and parsing logic.
