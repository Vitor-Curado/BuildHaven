use serde::Serialize;

#[derive(Serialize)]
pub enum ServiceStatus {
    Ok,
    Degraded,
    Down,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: ServiceStatus,
    pub service: &'static str,
    pub version: &'static str,
    pub uptime_seconds: u64,
}

impl HealthResponse {
    pub fn ok(service: &'static str, version: &'static str, uptime_seconds: u64) -> Self {
        Self {
            status: ServiceStatus::Ok,
            service,
            version,
            uptime_seconds,
        }
    }

    pub fn degraded(service: &'static str, version: &'static str, uptime_seconds: u64) -> Self {
        Self {
            status: ServiceStatus::Degraded,
            service,
            version,
            uptime_seconds,
        }
    }
}
