use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotificationServiceConfig {
    pub endpoint: String,
    pub timeout_secs: u32,
}
