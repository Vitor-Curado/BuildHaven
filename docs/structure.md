# 📁 Project Structure

```
├── docs
│   ├── architecture.md
│   ├── assets.md
│   ├── backend.md
│   ├── ci_cd.md
│   ├── dependencies.md
│   ├── deployment.md
│   ├── diagrams.md
│   ├── features.md
│   ├── frontend.md
│   ├── glossary.md
│   ├── infrastructure.md
│   ├── modules.md
│   ├── principles.md
│   ├── roadmap.md
│   ├── security.md
│   ├── structure.md
│   └── testing.md
│
├── scripts
│   ├── benchmark.sh
│   ├── check.sh
│   ├── deploy.sh
│   ├── grafana.sh
│   ├── inspect.sh
│   ├── prometheus.sh
│   ├── start-observability.sh
│   └── stop-observability.sh
│
├── src
│   ├── api.rs
│   ├── assets.rs
│   ├── auth.rs
│   ├── bootstrap.rs
│   ├── config.rs
│   ├── constants.rs
│   ├── content.rs
│   ├── context.rs
│   ├── cors.rs
│   ├── error.rs
│   ├── handlers.rs
│   ├── jobs.rs
│   ├── lib.rs
│   ├── logging.rs
│   ├── main.rs
│   ├── metrics.rs
│   ├── middleware.rs
│   ├── models.rs
│   ├── pool.rs
│   ├── rate_limit.rs
│   ├── repository.rs
│   ├── router.rs
│   ├── routes.rs
│   ├── security.rs
│   ├── services.rs
│   ├── session.rs
│   ├── shutdown.rs
│   ├── state.rs
│   ├── telemetry.rs
│   ├── templates.rs
│   └── utils.rs
│
├── static
│   ├── css
│   │   ├── aspect.css
│   │   ├── base.css
│   │   ├── buttons.css
│   │   ├── cards.css
│   │   ├── containers.css
│   │   ├── disclosure.css
│   │   ├── dropdown.css
│   │   ├── footer.css
│   │   ├── grids.css
│   │   ├── layout.css
│   │   ├── media.css
│   │   ├── navbar.css
│   │   ├── page-header.css
│   │   ├── pages.css
│   │   ├── posts.css
│   │   ├── stats.css
│   │   ├── themes.css
│   │   ├── typography.css
│   │   └── variables.css
│   │
│   ├── dist
│   │   ├── icons
│   │   ├── app-[sha256].js
│   │   ├── index-[sha256].css
│   │   └── manifest.json
│   │
│   ├── js
│   │   └── javascript.js
│   │
│   └── media
│       ├── food
│       ├── icons
│       └── languages
│   
├── templates
│   ├── pages
│   │   ├── apps.html
│   │   ├── assets.html
│   │   ├── blog.html
│   │   ├── boardgames.html
│   │   ├── contact_me.html
│   │   ├── food_detail.html
│   │   ├── food.html
│   │   ├── index.html
│   │   └── resume.html
│   │
│   ├── partials
│   │   ├── footer.html
│   │   └── navbar.html
│   │
│   └── base.html
│
├── tests
│   └── tests.rs
│
├── .dockerignore
├── .env
├── .gitignore
├── .index.txt
├── .woodpecker.yml
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── prometheus.yml
└── readme.md
```