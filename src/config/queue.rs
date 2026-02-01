use serde::Deserialize;

use crate::config::error::QueueError;

/// Конфигурация очереди
#[derive(Debug, Deserialize)]
pub struct QueueConfig {
    /// Точка подключения к очереди
    ///
    /// Пример: `amqp://localhost:5672`
    pub endpoint: String,

    /// Название обменника
    pub exchange_name: String,

    /// Имя пользователя
    pub username: String,

    /// Пароль пользователя
    pub password: String,
}

impl QueueConfig {
    pub fn validate(&self) -> Result<(), QueueError> {
        if self.endpoint.is_empty() {
            return Err(QueueError::EmptyEndpoint);
        }

        if self.exchange_name.is_empty() {
            return Err(QueueError::EmptyExchangeName);
        }

        if self.username.is_empty() {
            return Err(QueueError::EmptyUsername);
        }

        if self.password.is_empty() {
            return Err(QueueError::EmptyPassword);
        }

        Ok(())
    }
}
