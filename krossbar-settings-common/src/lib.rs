//! # [Krossbar](https://crates.io/crates/krossbar) settings common lib

pub mod error;
pub mod settings;

pub use error::{Error, Result};

/// Default settings directory
pub const DEFAULT_SETTINGS_DIR: &str = "/etc/krossbar/settings/";
