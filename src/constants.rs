pub mod endpoints {
    pub const HOME: &str = "/";
    pub const BLOG_POST: &str = "/{slug}";
    pub const CONTACT: &str = "/contact";
    pub const DOCS: &str = "/docs";
    pub const LOGIN: &str = "/login";

    pub const ADMIN: &str = "/admin";
    pub const ADMIN_POSTS: &str = "/admin/posts";
    pub const ADMIN_NEW_POST: &str = "/admin/posts/new";
    pub const ADMIN_POST: &str = "/admin/posts/{id}";
    pub const ADMIN_DELETE_POST: &str = "/admin/posts/{id}/delete";
    pub const ADMIN_PUBLISH_POST: &str = "/admin/posts/{id}/publish";
    pub const ADMIN_UNPUBLISH_POST: &str = "/admin/posts/{id}/unpublish";
}

pub mod icons {
    pub const BLOG: &str = "blog.png";
    pub const CONTACT: &str = "contact.svg";
    pub const DOCS: &str = "blog.png";
    pub const EMAIL: &str = "email.png";
    pub const HOME: &str = "home.svg";
    pub const LANGUAGE: &str = "language.svg";
    pub const LOGIN: &str = "login.svg";
    pub const RESUME: &str = "resume.svg";
    pub const SOFTWARE: &str = "software.svg";
    pub const THEME: &str = "theme.svg";
}

pub mod titles {
    pub const BLOG: &str = "Blog";
    pub const CONTACT: &str = "Contact";
    pub const DOCS: &str = "Documentation";
    pub const HOME: &str = "BuildHaven";
    pub const LOGIN: &str = "Login";
}

pub mod paths {
    pub const DIST: &str = "dist";
    pub const CSS: &str = "static/css";
    pub const VITE_MANIFEST: &str = "dist/.vite/manifest.json";
}

pub mod errors {
    pub const SESSION_ERROR: &str = "Session error";
    pub const MISSING_ASSET: &str = "Missing asset";
    pub const CONFIG_ERROR: &str = "Configuration error";
    pub const BAD_REQUEST: &str = "Bad request";
    pub const NOT_FOUND: &str = "Not found";
    pub const UNAUTHORIZED: &str = "Unauthorized";
    pub const INTERNAL: &str = "Internal server error";
}

pub mod cookies {
    pub const SESSION_TOKEN: &str = "session_token";
    pub const SESSION_ID: &str = "session_id";
}

pub mod service {
    pub const NAME: &str = "buildhaven";
}

pub mod project {
    pub const NAME: &str = "BuildHaven";
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const REPOSITORY: &str = "https://github.com/Vitor-Curado/BuildHaven/";
}

pub mod seo_descriptions {
    pub const HOME: &str = "Welcome to BuildHaven, a personal website programmed from scratch using Rust, TS, Postgres and AWS. Here you can find various blog posts and hopefully I can make you smile";
    pub const BLOG: &str = "Blog posts ranging from heartfelt stories to funny, to thoughtful, and a variety of other topics.";
    pub const LOGIN: &str = "Sign in to BuildHaven";
    pub const CONTACT: &str = "Contact information to get in touch with me";
    pub const DOCS: &str = "Heartfelt story-telling about my journey into software development, unfinished and full of humanity. I hope you enjoy it";
}

pub mod seo_canonical_paths {
    use crate::constants::endpoints;
    pub const HOME: &str = endpoints::HOME;
    pub const BLOG: &str = endpoints::BLOG_POST;
    pub const CONTACT: &str = endpoints::CONTACT;
    pub const DOCS: &str = endpoints::DOCS;
}

pub mod contact_information {
    pub const EMAIL: &str = "vitorbmcurado@proton.me";
    pub const LINKEDIN: &str = "https://www.linkedin.com/in/vitor-curado/";
    pub const GITHUB: &str = "https://github.com/Vitor-Curado";
    pub const WHATSAPP: &str = "https://wa.me/16477817484";
}
