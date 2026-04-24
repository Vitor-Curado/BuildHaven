use crate::{
    api::{HealthResponse, ServiceStatus},
    config::Environment,
    constants::icons,
    error::{AppError, AppResult},
    models::{LoginForm, NewUser, RegisterForm},
    repository::{create_user, find_user_by_email},
    services::list_posts,
    session::create_session,
    state::AppState,
    templates::{
        AssetsTemplate, BaseTemplateContext, BlogTemplate, ContactTemplate, FoodDetailTemplate,
        FoodTemplate, IndexTemplate, LoginTemplate, RegisterTemplate, ResumeTemplate,
    },
};

use axum::{
    Form, Json,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};

use askama::Template;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use std::time::{Duration, Instant};

const DUMMY_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$C5Z8YlH9l5k6n6u5W1zvQ8FJ5m0e3M3G7pT9oXk2c9Q";

#[allow(clippy::needless_pass_by_value)]
pub fn render_template<T: Template>(t: T) -> AppResult<Response> {
    // Timing instrumentation to measure render cost
    let start = Instant::now();
    let html = t.render().map_err(|e| {
        tracing::error!(
            error = ?e,
            template = std::any::type_name::<T>(),
            "Template render failed");
        e
    })?;
    tracing::debug!("Template render took {:?}", start.elapsed());
    Ok(Html(html).into_response())
}

pub async fn home(State(state): State<AppState>) -> Result<Response, AppError> {
    render_template(IndexTemplate {
        base: BaseTemplateContext::new(
            "BuildHaven",
            "home-icon.png",
            state.ctx.content.assets.clone(),
        ),
        readme_html: state.ctx.content.readme_html.clone(),
    })
}

pub async fn register_user(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let hashed = state.ctx.services.auth.hash_password(&form.password);

    let new_user = NewUser {
        username: form.username,
        email: form.email,
        password_hash: hashed,
    };

    match create_user(&state.ctx.services.db, new_user).await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(e) => {
            tracing::error!("User creation failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn register_page(State(state): State<AppState>) -> AppResult<Response> {
    render_template(RegisterTemplate {
        base: BaseTemplateContext::new(
            "Register",
            "register-icon.png",
            state.ctx.content.assets.clone(),
        ),
    })
}

pub async fn login_user(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    // Find user in DB
    let user = find_user_by_email(&state.ctx.services.db, &form.email)
        .await
        .ok()
        .flatten();

    let hash = user
        .as_ref()
        .map(|u| u.password_hash.as_str())
        .unwrap_or(DUMMY_HASH);

    // Verify password
    let valid = state
        .ctx
        .services
        .auth
        .verify_password(&form.password, hash);

    if !valid || user.is_none() {
        return (jar, Redirect::to("/login"));
    }

    let user = user.unwrap();
    let start = Instant::now();

    // If correct → create session
    let session = match create_session(&state.ctx.services.db, user.id, &state.ctx.config).await {
        Ok(session) => session,
        Err(_) => {
            let elapsed = start.elapsed();

            if elapsed < Duration::from_millis(150) {
                tokio::time::sleep(Duration::from_millis(150) - elapsed).await;
            }
            return (jar, Redirect::to("/login"));
        }
    };

    // Create cookie
    let cookie = Cookie::build(("session_id", session.id.to_string()))
        .path("/")
        .http_only(true)
        .secure(matches!(
            state.ctx.config.app.environment,
            Environment::Production
        ))
        .same_site(SameSite::Strict)
        .domain(state.ctx.config.app.cookie_domain.clone())
        .max_age(time::Duration::hours(24))
        .build();

    // Attach cookie
    let jar = jar.add(cookie);

    // Redirect
    (jar, Redirect::to("/"))
}

pub async fn login_page(State(state): State<AppState>) -> impl IntoResponse {
    render_template(LoginTemplate {
        base: BaseTemplateContext::new("Login", "login-icon.png", state.ctx.content.assets.clone()),
    })
}

/// Renders the food page.
///
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn food(State(state): State<AppState>) -> AppResult<Response> {
    render_template(FoodTemplate {
        base: BaseTemplateContext::new("Food", "food-icon.png", state.ctx.content.assets.clone()),
        foods: &state.ctx.content.food_data,
    })
}

pub async fn food_detail(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Response> {
    let food = state
        .ctx
        .content
        .food_data
        .iter()
        .find(|f| f.slug == slug)
        .ok_or(AppError::NotFound)?;

    render_template(FoodDetailTemplate {
        base: BaseTemplateContext::new(
            food.title,
            "food-icon.png",
            state.ctx.content.assets.clone(),
        ),
        food,
    })
}

/// Renders the resume page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn resume(State(state): State<AppState>) -> AppResult<Response> {
    render_template(ResumeTemplate {
        base: BaseTemplateContext::new("Resume", icons::RESUME, state.ctx.content.assets.clone()),
    })
}

/// Returns a JSON response indicating the health status of the application.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: ServiceStatus::Ok,
        service: "buildhaven",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: 0,
    })
}

/// Renders the blog page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn blog(State(state): State<AppState>) -> AppResult<Response> {
    let posts = list_posts(&state.ctx.services.db).await?;

    render_template(BlogTemplate {
        base: BaseTemplateContext::new("Blog", icons::BLOG, state.ctx.content.assets.clone()),
        posts,
    })
}

/// Renders the contact page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn contact(State(state): State<AppState>) -> AppResult<Response> {
    render_template(ContactTemplate {
        base: BaseTemplateContext::new("Contact", icons::CONTACT, state.ctx.content.assets.clone()),
    })
}

/// Renders the assets page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn assets(State(state): State<AppState>) -> AppResult<Response> {
    render_template(AssetsTemplate {
        base: BaseTemplateContext::new(
            "Assets",
            "assets-icon.png",
            state.ctx.content.assets.clone(),
        ),
    })
}
