// src/context.rs
use std::sync::Arc;

use crate::{config::Config, content::Content, services::Services};

#[derive(Clone)]
pub struct AppContext {
    pub config: Arc<Config>,
    pub content: Content,
    pub services: Services,
}

impl AppContext {
    pub fn new(config: Arc<Config>, content: Content, services: Services) -> Self {
        Self {
            config,
            content,
            services,
        }
    }
}
