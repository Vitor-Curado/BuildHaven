use serde::Serialize;

use crate::constants::icons::{BLOG, CONTACT, HOME, RESUME};

#[derive(Clone, Copy)]
pub struct NavItem {
    pub title: &'static str,
    pub href: &'static str,
    pub icon: &'static str,
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
}

#[derive(Clone, Copy)]
pub struct Language {
    pub code: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
}

#[derive(Clone, Copy, Serialize)]
pub struct DocItem {
    pub title: &'static str,
    pub slug: &'static str,
    pub markdown: &'static str,
}

pub const NAV_ITEMS: &[NavItem] = &[
    NavItem {
        title: "Home",
        href: "/",
        icon: HOME,
    },
    NavItem {
        title: "Blog",
        href: "/blog",
        icon: BLOG,
    },
    NavItem {
        title: "Resume",
        href: "/resume",
        icon: RESUME,
    },
    NavItem {
        title: "Contact",
        href: "/contact",
        icon: CONTACT,
    },
];

pub const THEMES: &[Theme] = &[
    Theme {
        id: "sunset",
        name: "Sunset",
        icon: "sun-theme.png",
    },
    Theme {
        id: "night",
        name: "Night",
        icon: "night-theme.png",
    },
    Theme {
        id: "forest",
        name: "Forest",
        icon: "forest-theme.png",
    },
    Theme {
        id: "ocean",
        name: "Ocean",
        icon: "ocean-theme.png",
    },
];

pub const LANGUAGES: &[Language] = &[
    Language {
        code: "en",
        label: "English",
        icon: "english.png",
    },
    Language {
        code: "fr",
        label: "French",
        icon: "french.png",
    },
    Language {
        code: "pt",
        label: "Português",
        icon: "portuguese.png",
    },
    Language {
        code: "zh",
        label: "中文",
        icon: "mandarin.png",
    },
    Language {
        code: "ru",
        label: "Русский",
        icon: "russian.png",
    },
    Language {
        code: "kr",
        label: "한국인",
        icon: "korean.png",
    },
    Language {
        code: "jp",
        label: "日本語",
        icon: "japanese.png",
    },
    Language {
        code: "de",
        label: "Deutsch",
        icon: "german.png",
    },
];

pub const DOCS: &[DocItem] = &[
    DocItem {
        title: "Architecture",
        slug: "architecture",
        markdown: include_str!("../docs/architecture.md"),
    },
    DocItem {
        title: "Assets",
        slug: "assets",
        markdown: include_str!("../docs/assets.md"),
    },
    DocItem {
        title: "Authentication",
        slug: "authentication",
        markdown: include_str!("../docs/authentication.md"),
    },
    DocItem {
        title: "Backend",
        slug: "backend",
        markdown: include_str!("../docs/backend.md"),
    },
    DocItem {
        title: "Bootstrap",
        slug: "bootstrap",
        markdown: include_str!("../docs/bootstrap.md"),
    },
    DocItem {
        title: "CI/CD",
        slug: "ci-cd",
        markdown: include_str!("../docs/ci_cd.md"),
    },
    DocItem {
        title: "Dependencies",
        slug: "dependencies",
        markdown: include_str!("../docs/dependencies.md"),
    },
    DocItem {
        title: "Deployment",
        slug: "deployment",
        markdown: include_str!("../docs/deployment.md"),
    },
    DocItem {
        title: "Diagrams",
        slug: "diagrams",
        markdown: include_str!("../docs/diagrams.md"),
    },
    DocItem {
        title: "Features",
        slug: "features",
        markdown: include_str!("../docs/features.md"),
    },
    DocItem {
        title: "Frontend",
        slug: "frontend",
        markdown: include_str!("../docs/frontend.md"),
    },
    DocItem {
        title: "Glossary",
        slug: "glossary",
        markdown: include_str!("../docs/glossary.md"),
    },
    DocItem {
        title: "Infrastructure",
        slug: "infrastructure",
        markdown: include_str!("../docs/infrastructure.md"),
    },
    DocItem {
        title: "Jobs",
        slug: "jobs",
        markdown: include_str!("../docs/jobs.md"),
    },
    DocItem {
        title: "Modules",
        slug: "modules",
        markdown: include_str!("../docs/modules.md"),
    },
    DocItem {
        title: "Observability",
        slug: "observability",
        markdown: include_str!("../docs/observability.md"),
    },
    DocItem {
        title: "Principles",
        slug: "principles",
        markdown: include_str!("../docs/principles.md"),
    },
    DocItem {
        title: "Roadmap",
        slug: "roadmap",
        markdown: include_str!("../docs/roadmap.md"),
    },
    DocItem {
        title: "Security",
        slug: "security",
        markdown: include_str!("../docs/security.md"),
    },
    DocItem {
        title: "Session management",
        slug: "session-management",
        markdown: include_str!("../docs/session-management.md"),
    },
    DocItem {
        title: "State",
        slug: "state",
        markdown: include_str!("../docs/state.md"),
    },
    DocItem {
        title: "Structure",
        slug: "structure",
        markdown: include_str!("../docs/structure.md"),
    },
    DocItem {
        title: "Testing",
        slug: "testing",
        markdown: include_str!("../docs/testing.md"),
    }
];
