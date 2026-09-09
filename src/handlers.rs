use crate::{
    auth::verify_password,
    constants::{cookies, icons, titles},
    error::{AppError, AppResult},
    models::LoginForm,
    navbar::DOCS,
    repository::{find_user_by_email, get_all_posts},
    session::create_session,
    state::AppState,
    templates::{
        BaseTemplateContext, BlogTemplate, ContactTemplate, DocsTemplate, IndexTemplate,
        LoginTemplate, ResumeTemplate,
    },
    utils::markdown_to_html,
};

use axum::{
    Form,
    extract::{Path, State},
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
        base: BaseTemplateContext::build_base_context(&state, titles::HOME, icons::HOME),
    })
}

pub async fn login_user(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    // Landmine, to be investigated later.
    let user = find_user_by_email(&state.db, &form.email)
        .await
        .ok()
        .flatten();

    let hash = user
        .as_ref()
        .map(|u| u.password_hash.as_str())
        .unwrap_or(DUMMY_HASH);

    // Verify password
    let valid = verify_password(&form.password, hash);

    if !valid || user.is_none() {
        return (jar, Redirect::to("/login"));
    }

    let user = user.unwrap();
    let start = Instant::now();

    // If correct → create session
    let created = match create_session(&state.db, user.id, &state.config).await {
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
    let cookie = Cookie::build((cookies::SESSION_TOKEN, created.token))
        .path("/")
        .http_only(true)
        .secure(state.config.app.cookie_secure)
        .same_site(SameSite::Strict)
        .domain(state.config.app.cookie_domain.clone())
        .max_age(time::Duration::hours(state.config.session.duration_hours))
        .build();

    // Attach cookie
    let jar = jar.add(cookie);

    // Redirect
    (jar, Redirect::to("/"))
}

pub async fn login_page(State(state): State<AppState>) -> impl IntoResponse {
    render_template(LoginTemplate {
        base: BaseTemplateContext::build_base_context(&state, titles::LOGIN, icons::LOGIN),
    })
}

/// Renders the resume page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn resume(State(state): State<AppState>) -> AppResult<Response> {
    render_template(ResumeTemplate {
        base: BaseTemplateContext::build_base_context(&state, titles::RESUME, icons::RESUME),
    })
}

/// Renders the blog page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn blog(State(state): State<AppState>) -> AppResult<Response> {
    let posts = get_all_posts(&state.db).await?;

    render_template(BlogTemplate {
        base: BaseTemplateContext::build_base_context(&state, titles::BLOG, icons::BLOG),
        posts,
    })
}

pub async fn docs(Path(slug): Path<String>, State(state): State<AppState>) -> AppResult<Response> {
    let doc = DOCS
        .iter()
        .find(|d| d.slug == slug)
        .ok_or(AppError::NotFound)?;

    let html = markdown_to_html(doc.markdown);

    render_template(DocsTemplate {
        base: BaseTemplateContext::build_base_context(&state, doc.title, icons::DOCS),

        title: doc.title,
        content_html: html,
    })
}

/// Renders the contact page.
/// # Panics
/// This function will panic if the template rendering fails.
pub async fn contact(State(state): State<AppState>) -> AppResult<Response> {
    render_template(ContactTemplate {
        base: BaseTemplateContext::build_base_context(&state, titles::CONTACT, icons::CONTACT),
    })
}
