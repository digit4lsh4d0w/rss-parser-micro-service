use serde::Deserialize;

use crate::config::{
    error::{ValidationError, ValidationResult},
    validation::validate_url,
};

pub const MIN_NOTIFICATION_TIMEOUT: usize = 1;

/// Сервис уведомлений.
#[derive(Debug)]
pub struct NotificationServiceConfig {
    pub url: String,
    pub timeout: usize,
}

impl TryFrom<RawNotificationServiceConfig> for NotificationServiceConfig {
    type Error = ValidationError;

    fn try_from(raw_config: RawNotificationServiceConfig) -> Result<Self, Self::Error> {
        raw_config.validate()?;

        Ok(Self {
            url: raw_config.url,
            timeout: raw_config.timeout,
        })
    }
}

#[derive(Deserialize)]
pub struct RawNotificationServiceConfig {
    pub url: String,
    pub timeout: usize,
}

impl RawNotificationServiceConfig {
    pub fn validate(&self) -> ValidationResult<()> {
        validate_url(&self.url)?;
        self.validate_timeout()?;

        Ok(())
    }

    pub fn validate_timeout(&self) -> ValidationResult<()> {
        if self.timeout < MIN_NOTIFICATION_TIMEOUT {
            return Err(ValidationError::NotificationServiceTimeoutTooSmall(
                self.timeout,
            ));
        }

        Ok(())
    }
}
