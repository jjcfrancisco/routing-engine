pub type RoutyResult<T> = Result<T, RoutyError>;

#[derive(thiserror::Error, Debug)]
pub enum RoutyError {
    #[error("Error Saving Network: '{0}'")]
    SaveNetworkError(String),
}

