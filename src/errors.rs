use std::io::Error as IoError;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown error: `{0}`")]
    UnknownError(String),
    #[error("IO error: `{0}`")]
    IoError(#[from] IoError),
}
