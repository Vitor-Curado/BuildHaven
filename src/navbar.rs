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
];

pub const DOCS: &[DocItem] = &[
    DocItem {
        title: "Decisions",
        slug: "state",
        markdown: include_str!("../docs/decisions.md"),
    },
    DocItem {
        title: "Learning journey",
        slug: "structure",
        markdown: include_str!("../docs/learning-journey.md"),
    },
    DocItem {
        title: "Tech stack",
        slug: "testing",
        markdown: include_str!("../docs/tech-stack.md"),
    },
];
