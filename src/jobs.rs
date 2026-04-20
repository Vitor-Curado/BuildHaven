use crate::state::AppState;

use std::time::Duration;
use tokio::time::sleep;

pub struct JobRunner {
    state: AppState,
}

impl JobRunner {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn start(self) {
        tokio::spawn(self.session_cleanup_loop());
    }

    pub async fn session_cleanup_loop(self) {
        loop {
            sleep(Duration::from_secs(60)).await;

            let _db = &self.state.ctx.services.db;

            // delete expired sessions
            tracing::info!("Cleaning up sessions...");
        }
    }
}
