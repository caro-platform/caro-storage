use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Lib error
#[derive(Debug, Serialize, Deserialize, Clone, Error)]
pub enum Error {
    /// IO error during file operation
    #[error("Filesystem IO error")]
    IoError(String),
    /// Invalid type coercion
    #[error("Requested settings type error")]
    Type(String),
    /// Requested value not found in a settings file
    #[error("Value not found")]
    NotFound,
    /// Settings file is corrupted
    #[error("Settings file corrupted")]
    Corrupted(String),
}
