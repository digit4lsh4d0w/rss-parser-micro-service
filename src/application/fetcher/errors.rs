use thiserror::Error;

// TODO: Создать нормальные ошибки
#[derive(Debug, Error)]
pub enum FetcherError {
    #[error("Unknown error")]
    Unknown,
}
