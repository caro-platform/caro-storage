pub mod storage;
pub mod value;

pub use storage::Storage;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
