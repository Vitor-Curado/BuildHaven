use crate::{
    auth::verify_password, constants::{icons, titles}, error::{AppError, AppResult}, models::{LoginForm, NewPost, UpdatePost}, navbar::DOCS, repository::{
        create_post, delete_post, find_user_by_email, get_all_posts, get_post_by_id, get_posts_by_slug, get_published_posts, publish_post, unpublish_post, update_post,
    }, state::AppState, templates::{
        AdminEditPostTemplate, AdminNewPostTemplate, AdminPostsTemplate, AdminTemplate,
        BaseTemplateContext, BlogPostTemplate, ContactTemplate, DocsTemplate,
        IndexTemplate, LoginTemplate, ResumeTemplate,
    }, utils::markdown_to_html,
};

use axum::{
    Form,
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
};

use askama::Template;
use std::time::Instant;
use tower_sessions::Session;
use uuid::Uuid;

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

// pub fn generate_static_pages()

pub async fn home(State(state): State<AppState>) -> Result<Response, AppError> {
    let posts = get_published_posts(&state.db).await?;

    render_template(IndexTemplate {
        base: BaseTemplateContext::build_base_context(&state, titles::BLOG, icons::BLOG),
        posts,
    })
}

pub async fn blog_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Response> {
    let post = get_posts_by_slug(&state.db, slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let content_html = markdown_to_html(&post.content);

    render_template(BlogPostTemplate {
        base: BaseTemplateContext::build_base_context(&state, &post.title, icons::BLOG),
        content_html,
        post,
    })
}

pub async fn login_user(
    State(state): State<AppState>,
    session: Session,
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
        return Redirect::to("/login");
    }

    let user = user.unwrap();

    if let Err(error) = session.insert("user_id", user.id).await {
        tracing::error!(?error, "Failed to create user sessions");
        return Redirect::to("/login");
    }

    Redirect::to("/admin")
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

// ***** Admin stuff *****

pub async fn admin(State(state): State<AppState>) -> AppResult<Response> {
    render_template(AdminTemplate {
        // Temporary values
        base: BaseTemplateContext::build_base_context(&state, titles::BLOG, icons::DOCS),
    })
}

pub async fn admin_posts(State(state): State<AppState>) -> AppResult<Response> {
    let posts = get_all_posts(&state.db).await?;

    render_template(AdminPostsTemplate {
        base: BaseTemplateContext::build_base_context(&state, "Manage Posts", icons::BLOG),
        posts,
    })
}

pub async fn admin_new_post(State(state): State<AppState>) -> AppResult<Response> {
    render_template(AdminNewPostTemplate {
        base: BaseTemplateContext::build_base_context(&state, "New Post", icons::BLOG),
    })
}

pub async fn admin_create_post(
    State(state): State<AppState>,
    Form(form): Form<NewPost>,
) -> AppResult<Response> {
    create_post(&state.db, &form).await?;

    Ok(Redirect::to("/admin/posts").into_response())
}

pub async fn admin_edit_post(
    Path(post_id): Path<Uuid>,
    State(state): State<AppState>,
) -> AppResult<Response> {
    let post = get_post_by_id(&state.db, post_id)
        .await?
        .ok_or(AppError::NotFound)?;

    render_template(AdminEditPostTemplate {
        base: BaseTemplateContext::build_base_context(&state, "Edit post", icons::BLOG),
        post,
    })
}

pub async fn admin_update_post(
    Path(post_id): Path<Uuid>,
    State(state): State<AppState>,
    Form(form): Form<UpdatePost>,
) -> AppResult<Response> {
    let updated = update_post(&state.db, post_id, &form).await?;

    if !updated {
        return Err(AppError::NotFound);
    }

    Ok(Redirect::to("/admin/posts").into_response())
}

pub async fn admin_delete_post(
    Path(post_id): Path<Uuid>,
    State(state): State<AppState>,
) -> AppResult<Response> {
    let deleted = delete_post(&state.db, post_id).await?;

    if !deleted {
        return Err(AppError::NotFound);
    }

    Ok(Redirect::to("/admin/posts").into_response())
}

pub async fn admin_publish_post(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>
) -> AppResult<Response> {
    let updated = publish_post(&state.db, post_id).await?;

    if !updated {
        return Err(AppError::NotFound);
    }

    Ok(Redirect::to(&format!("/admin/posts")).into_response())
}

pub async fn admin_unpublish_post(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>
) -> AppResult<Response> {
    let updated = unpublish_post(&state.db, post_id).await?;

    if !updated {
        return Err(AppError::NotFound);
    }

    Ok(Redirect::to(&format!("/admin/posts")).into_response())
}
