pub mod client;

pub mod ui;
pub mod errors;

pub const AHRI_ID: i32 = 103;


#[derive(Debug, Clone)]
pub enum AppError {
    IoError(String),
    ReqwestError(String),
    RiotClientError(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ReqwestError(err.to_string())
    }
}


pub type AppResult<T> = Result<T, AppError>;


pub async fn wait_n_millis(n: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(n)).await;
}