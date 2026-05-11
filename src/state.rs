// src/state.rs
use crate::{
    assets::Assets, config::Config, content::Content, context::AppContext, error::AppError,
    services::Services,
};

use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub ctx: Arc<AppContext>,
}

impl AppState {
    pub fn new(db: PgPool, config: Arc<Config>, assets: Assets) -> Result<Self, AppError> {
        let content = Content::new(assets);
        let services = Services::new(db);
        let ctx = AppContext::new(config, content, services);

        Ok(Self { ctx: Arc::new(ctx) })
    }
}
