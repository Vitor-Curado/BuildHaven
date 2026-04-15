use crate::{
    api::HealthResponse,
    services::list_posts,
    state::AppState,
    templates::{
        AssetsTemplate, BlogTemplate, ContactTemplate, FoodDetailTemplate, FoodTemplate,
        IndexTemplate, ResumeTemplate, RegisterTemplate, LoginTemplate,
    },
    repository::create_user,
    models::{NewUser, RegisterForm},
};

use axum::{
    Json,
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response, Redirect},
};

use askama::Template;

#[allow(clippy::needless_pass_by_value)]
pub fn render_template<T: Template>(t: T) -> Response {
    match t.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Template rendering failed",
            )
                .into_response()
        }
        .into_response(),
    }
}

pub async fn home(State(app_state): State<AppState>) -> impl IntoResponse {
    render_template(IndexTemplate {
        title: "Buildhaven",
        favicon: "home-icon.png",
        readme_html: app_state.readme_html.clone(),
        assets: app_state.assets.clone(),
    })
}

pub async fn register_user(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {

    let hashed =
        state.auth.hash_password(&form.password);

    let new_user = NewUser {
        username: form.username,
        email: form.email,
        password_hash: hashed,
    };

    create_user(&state.db, new_user)
        .await
        .unwrap();

    Redirect::to("/login")
}

pub async fn register_page(State(state): State<AppState>) -> impl IntoResponse {
    render_template(RegisterTemplate {
        title: "Register",
        favicon: "register-icon.png",
        assets: state.assets.clone(),
    })
}

pub async fn login_page(State(state): State<AppState>) -> impl IntoResponse {
    render_template(LoginTemplate {
        title: "Login",
        favicon: "login-icon.png",
        assets: state.assets.clone(),
    })
}

/// Renders the food page.
///
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn food(State(app_state): State<AppState>) -> impl IntoResponse {
    render_template(FoodTemplate {
        title: "Food",
        favicon: "food-icon.png",
        foods: &app_state.food_data,
        assets: app_state.assets.clone(),
    })
}

pub async fn food_detail(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    /*
    let food = state.food_data.iter().find(|f| f.slug == slug)
    .ok_or(AppError::NotFound)?;
    */
    let Some(food) = state.food_data.iter().find(|f| f.slug == slug) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    render_template(FoodDetailTemplate {
        title: food.title.to_string(),
        favicon: "food-detail-icon.png",
        food,
        assets: state.assets.clone(),
    })
}

/// Renders the resume page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn resume(State(app_state): State<AppState>) -> impl IntoResponse {
    render_template(ResumeTemplate {
        title: "Resume",
        favicon: "resume-icon.png",
        assets: app_state.assets.clone(),
    })
}

/// Returns a JSON response indicating the health status of the application.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "personal-website",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// Renders the blog page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn blog(State(app_state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let posts = list_posts(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(render_template(BlogTemplate {
        title: "Blog",
        favicon: "blog-icon.png",
        assets: app_state.assets.clone(),
        posts,
    }))
}

/// Renders the contact page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn contact(State(app_state): State<AppState>) -> impl IntoResponse {
    render_template(ContactTemplate {
        title: "Contact",
        favicon: "contact-icon.png",
        assets: app_state.assets.clone(),
    })
}

/// Renders the assets page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn assets(State(app_state): State<AppState>) -> impl IntoResponse {
    render_template(AssetsTemplate {
        title: "Assets",
        favicon: "assets-icon.png",
        assets: app_state.assets.clone(),
    })
}
