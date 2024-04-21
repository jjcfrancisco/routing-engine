use std::io;

pub type RoutyResult<T> = Result<T, RoutyError>;

#[derive(thiserror::Error, Debug)]
pub enum RoutyError {
    #[error("Unable to bind to {1}: {0}")]
    NetworkCreationError(io::Error, String),
}
