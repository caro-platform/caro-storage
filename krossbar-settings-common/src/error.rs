use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, Clone, Error)]
pub enum Error {
    /// User is not allowed to register with a provided service name, or
    /// not allowed to connect to a requested client
    #[error("Filesystem IO error")]
    IoError(String),
    #[error("Requested settings type error")]
    Type(String),
    #[error("Value not found")]
    NotFound,
    #[error("Settings file corrupted")]
    Corrupted(String),
}
