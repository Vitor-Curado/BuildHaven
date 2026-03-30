# 🗄️ Back-End Overview

## 🔁 Request Flow

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


## APIs

- Health: `/health` → returns service status in JSON
- Food: `/food` → lists foods, `/food/:slug` → food details
- Resume, Blog, Contact, Assets: respective pages