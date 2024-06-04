pub mod storage;
pub mod value;

pub use storage::Settings;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
