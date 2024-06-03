pub mod error;
pub mod settings;

pub use error::{Error, Result};

pub const DEFAULT_STORAGE_DIR: &str = "/etc/krossbar/storage/";
