use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error occurred: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Request error occurred: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Riot client error: {0}")]
    RiotClientError(String),
}
pub type AppResult<T> = Result<T, AppError>;