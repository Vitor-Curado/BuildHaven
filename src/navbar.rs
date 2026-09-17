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

pub const DOCS: &[DocItem] = &[
    DocItem {
        title: "Decisions",
        slug: "decicions",
        markdown: include_str!("../docs/decisions.md"),
    },
    DocItem {
        title: "Learned concepts",
        slug: "learned-concepts",
        markdown: include_str!("../docs/learned-concepts.md"),
    },
    DocItem {
        title: "Learning journey",
        slug: "learning-journey",
        markdown: include_str!("../docs/learning-journey.md"),
    },
    DocItem {
        title: "Scope",
        slug: "scope",
        markdown: include_str!("../docs/scope.md"),
    },
    DocItem {
        title: "What is software engineering",
        slug: "what-is-software-engineering",
        markdown: include_str!("../docs/what-is-software-engineering.md"),
    },
];
