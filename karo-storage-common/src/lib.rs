use std::env;

pub mod document;
pub mod queries;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const STORAGE_DIR_PATH_ENV: &str = "CARO_STORAGE_DIR_PATH";
const DEFAULT_STORAGE_DIR: &str = "/etc/karo.storage.d/";

pub fn get_storage_dir() -> String {
    if let Ok(path) = env::var(STORAGE_DIR_PATH_ENV) {
        path
    } else {
        DEFAULT_STORAGE_DIR.into()
    }
}
