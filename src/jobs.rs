use crate::{repository::delete_expired_sessions, services::list_posts, state::AppState};
use async_trait::async_trait;
use futures_util::FutureExt;
use std::{panic::AssertUnwindSafe, sync::Arc, time::Duration};
use tokio::task::JoinHandle;

pub struct JobRunner {
    jobs: Vec<Arc<dyn Job>>,
    handles: Vec<JoinHandle<()>>,
}

#[async_trait]
pub trait Job: Send + Sync + 'static {
    fn name(&self) -> &'static str;

    fn interval(&self) -> Duration;

    async fn run(&self);
}

impl Default for JobRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl JobRunner {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            handles: Vec::new(),
        }
    }

    pub fn register<J: Job>(&mut self, job: J) {
        self.jobs.push(Arc::new(job));
    }

    pub fn start(mut self) {
        for job in self.jobs.drain(..) {
            let handle = tokio::spawn(Self::run_job(job));
            self.handles.push(handle);
        }
    }

    async fn run_job(job: Arc<dyn Job>) {
        let name = job.name();

        loop {
            tokio::time::sleep(job.interval()).await;

            tracing::debug!("Running job: {}", name);

            let result = AssertUnwindSafe(job.run()).catch_unwind().await;

            if let Err(e) = result {
                tracing::error!("Job '{}' panicked: {:?}", name, e);
            }
        }
    }
}

pub struct SessionCleanupJob {
    state: AppState,
}

impl SessionCleanupJob {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Job for SessionCleanupJob {
    fn name(&self) -> &'static str {
        "session_cleanup"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(300)
    }

    async fn run(&self) {
        match delete_expired_sessions(&self.state.ctx.services.db).await {
            Ok(count) => tracing::info!("Deleted {} expired sessions", count),
            Err(e) => tracing::error!("Session cleanup failed: {:?}", e),
        }
    }
}

pub struct CacheWarmupJob {
    state: AppState,
}

impl CacheWarmupJob {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Job for CacheWarmupJob {
    fn name(&self) -> &'static str {
        "cache_warmup"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(60)
    }

    async fn run(&self) {
        match list_posts(&self.state.ctx.services.db).await {
            Ok(_posts) => {
                tracing::debug!("Cache refreshed");
            }
            Err(e) => tracing::error!("Cache refresh failed: {:?}", e),
        }
    }
}

pub struct SessionMetricsJob {
    state: AppState,
}

impl SessionMetricsJob {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Job for SessionMetricsJob {
    fn name(&self) -> &'static str {
        "session_metrics"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(60)
    }

    async fn run(&self) {
        use sqlx::query_scalar;

        let db = &self.state.ctx.services.db;

        let active: i64 = query_scalar("SELECT COUNT(*) FROM sessions WHERE expires_at > NOW()")
            .fetch_one(db)
            .await
            .unwrap_or(0);

        let expired: i64 = query_scalar("SELECT COUNT(*) FROM sessions WHERE expires_at <= NOW()")
            .fetch_one(db)
            .await
            .unwrap_or(0);

        tracing::info!(
            active_sessions = active,
            expired_sessions = expired,
            "session metrics snapshot"
        );
    }
}
