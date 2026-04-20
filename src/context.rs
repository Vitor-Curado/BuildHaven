use crate::{config::Config, content::Content, services::Services};

#[derive(Clone)]
pub struct AppContext {
    pub config: Config,
    pub content: Content,
    pub services: Services,
}