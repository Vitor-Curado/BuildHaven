# Full request lifecycle

Browser
↓
Nginx
↓
Axum Router
↓
Middleware Layers
    - Logging
    - Security Headers
    - CORS
    - Rate Limiting
↓
Handler
↓
Service
↓
Repository
↓
PostgreSQL
↓
Template
↓
HTML Response

# Startup initialization

Program Start
↓
Load Config
↓
Create DB Pool
↓
Load Asset Manifest
↓
Parse README Markdown
↓
Load Mock Data
↓
Build AppState
↓
Start HTTP Server