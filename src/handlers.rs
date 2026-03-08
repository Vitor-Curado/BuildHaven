use crate::repository::mock_food_data;
use crate::templates::{
    AssetsTemplate, BlogTemplate, ContactTemplate, FoodDetailTemplate, FoodTemplate, IndexTemplate,
    ResumeTemplate,
};
use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use pulldown_cmark::{Parser, html};
use std::fs;

use crate::api::HealthResponse;
use axum::Json;

fn load_readme() -> String {
    fs::read_to_string("./readme.md") // relative to working dir in container
        .unwrap_or_else(|_| "# README not found".to_string())
}

fn markdown_to_html(md: &str) -> String {
    let parser = Parser::new(md);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

pub async fn home() -> impl IntoResponse {
    let readme_md = load_readme();
    let readme_html = markdown_to_html(&readme_md);

    Html(
        IndexTemplate {
            title: "Home",
            favicon: "home-icon.png",
            readme_html,
        }
        .render()
        .unwrap(),
    )
}

pub async fn food() -> impl IntoResponse {
    let template = FoodTemplate {
        title: "Food",
        favicon: "food-icon.png",
        foods: mock_food_data(),
    };

    Html(template.render().unwrap())
}

pub async fn food_detail(Path(slug): Path<String>) -> impl IntoResponse {
    let foods = mock_food_data();

    let food = foods.iter().find(|f| f.slug == slug).unwrap();

    let template = FoodDetailTemplate {
        title: food.title.to_string(),
        favicon: "food",
        food,
    };

    Html(template.render().unwrap())
}

pub async fn resume() -> impl IntoResponse {
    Html(
        ResumeTemplate {
            title: "Resume",
            favicon: "resume-icon.png",
        }
        .render()
        .unwrap(),
    )
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn blog() -> impl IntoResponse {
    Html(
        BlogTemplate {
            title: "Blog",
            favicon: "blog-icon.png",
        }
        .render()
        .unwrap(),
    )
}

pub async fn contact() -> impl IntoResponse {
    Html(
        ContactTemplate {
            title: "Contact Me",
            favicon: "contact-icon.png",
        }
        .render()
        .unwrap(),
    )
}

pub async fn assets() -> impl IntoResponse {
    Html(
        AssetsTemplate {
            title: "Assets",
            favicon: "assets-icon.png",
        }
        .render()
        .unwrap(),
    )
}

// Unit tests
#[test]
fn converts_multiple_markdown_features() {
    let input = "# Title\n\n**Bold** text";
    let html = markdown_to_html(input);

    assert!(html.contains("<h1>Title</h1>"));
    assert!(html.contains("<strong>Bold</strong>"));
}
