# 📁 Project Structure

```
├── docs
│   ├── deployment.md
│   ├── features.md
│   ├── modules.md
│   ├── roadmap.md
│   └── structure.md
│
├── scripts
│   ├── check.sh
│   ├── deploy.sh
│   └── inspect.sh
│
├── src
│   ├── bin
│   │   └── assets.rs
│   │
│   ├── api.rs
│   ├── assets.rs
│   ├── config.rs
│   ├── handlers.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── models.rs
│   ├── repository.rs
│   ├── router.rs
│   ├── routes.rs
│   ├── security.rs
│   ├── state.rs
│   ├── templates.rs
│   └── utils.rs
│
├── static
│   ├── css
│   │   ├── components
│   │   │   ├── buttons.css
│   │   │   ├── cards.css
│   │   │   ├── containers.css
│   │   │   ├── dropdown.css
│   │   │   ├── grids.css
│   │   │   └── navbar.css
│   │   │
│   │   ├── pages
│   │   │   ├── contact.css
│   │   │   ├── food-detail.css
│   │   │   └── food.css
│   │   │
│   │   ├── base.css
│   │   ├── layout.css
│   │   ├── navbar.css
│   │   ├── themes.css
│   │   ├── typography.css
│   │   └── variables.css
│   │
│   ├── dist
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
│   │   └── navbar.html
│   │
│   └── base.html
│
├── tests
│   └── tests.rs
│
├── .dockerignore
├── .gitignore
├── .woodpecker.yml
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml
├── Dockerfile
└── readme.md
```