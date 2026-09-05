// src/state.rs
use crate::{assets::Assets, config::Config};

use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub assets: Arc<Assets>,
}

impl AppState {
    pub fn new(db: PgPool, config: Arc<Config>, assets: Arc<Assets>) -> Self {
        Self { db, config, assets }
    }
}
