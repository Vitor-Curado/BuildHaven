use crate::{
    api::HealthResponse,
    config::Environment,
    error::{AppError, AppResult},
    models::{LoginForm, NewUser, RegisterForm},
    repository::{create_user, find_user_by_email},
    services::list_posts,
    session::create_session,
    state::AppState,
    templates::{
        AssetsTemplate, BlogTemplate, ContactTemplate, FoodDetailTemplate, FoodTemplate,
        IndexTemplate, LoginTemplate, RegisterTemplate, ResumeTemplate,
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
use std::time::Instant;

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

pub async fn home(State(app_state): State<AppState>) -> Result<Response, AppError> {
    render_template(IndexTemplate {
        title: "BuildHaven",
        favicon: "home-icon.png",
        readme_html: app_state.ctx.content.readme_html.clone(),
        assets: app_state.ctx.content.assets.clone(),
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
        title: "Register",
        favicon: "register-icon.png",
        assets: state.ctx.content.assets.clone(),
    })
}

pub async fn login_user(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    // Find user in DB
    let user = match find_user_by_email(&state.ctx.services.db, &form.email).await {
        Ok(Some(user)) => user,
        _ => return (jar, Redirect::to("/login")),
    };

    // Verify password
    let valid = state
        .ctx
        .services
        .auth
        .verify_password(&form.password, &user.password_hash);

    if !valid {
        return (jar, Redirect::to("/login"));
    }

    // If correct → create session
    let session = match create_session(&state.ctx.services.db, user.id, &state.ctx.config).await {
        Ok(session) => session,
        Err(_) => {
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
        title: "Login",
        favicon: "login-icon.png",
        assets: state.ctx.content.assets.clone(),
    })
}

/// Renders the food page.
///
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn food(State(app_state): State<AppState>) -> AppResult<Response> {
    render_template(FoodTemplate {
        title: "Food",
        favicon: "food-icon.png",
        foods: &app_state.ctx.content.food_data,
        assets: app_state.ctx.content.assets.clone(),
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
        title: food.title.to_string(),
        favicon: "food-detail-icon.png",
        food,
        assets: state.ctx.content.assets.clone(),
    })
}

/// Renders the resume page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn resume(State(app_state): State<AppState>) -> AppResult<Response> {
    render_template(ResumeTemplate {
        title: "Resume",
        favicon: "resume-icon.png",
        assets: app_state.ctx.content.assets.clone(),
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
pub async fn blog(State(app_state): State<AppState>) -> AppResult<Response> {
    let posts = list_posts(&app_state.ctx.services.db).await?;

    render_template(BlogTemplate {
        title: "Blog",
        favicon: "blog-icon.png",
        assets: app_state.ctx.content.assets.clone(),
        posts,
    })
}

/// Renders the contact page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn contact(State(app_state): State<AppState>) -> AppResult<Response> {
    render_template(ContactTemplate {
        title: "Contact",
        favicon: "contact-icon.png",
        assets: app_state.ctx.content.assets.clone(),
    })
}

/// Renders the assets page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn assets(State(app_state): State<AppState>) -> AppResult<Response> {
    render_template(AssetsTemplate {
        title: "Assets",
        favicon: "assets-icon.png",
        assets: app_state.ctx.content.assets.clone(),
    })
}
