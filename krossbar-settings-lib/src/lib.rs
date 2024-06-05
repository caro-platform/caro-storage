//! Krossbar settings lib
//!
//! The library provides a convenient wrapper for reading and writing
//! settings into JSON file.
//!
//! Use [Settings] handle to open settings file. Using the handle you can
//! read, insert or delete settings entries from the inner JSON.
//!
//! To create an [Entry] use [Settings::read] or [Settings::read_or_insert] method, which tries to read
//! the value from the settings file, or adds new JSON entry if doesn't exist.
//!
//! Using [Entry] handle you can modify corresponding JSON settings file.
//!
//! # Examples
//! ```rust
//! use std::path::Path;
//!
//! use krossbar_settings_common::DEFAULT_SETTINGS_DIR;
//! use krossbar_settings_lib::Settings;
//!
//! const SERVICE_NAME: &str = "krossbar.storage.example";
//!
//! fn print_json(path: &Path) {
//!     let data = std::fs::read_to_string(path).unwrap();
//!     println!("Settings: '{data}'");
//! }
//!
//! fn main() {
//!     let settings = Settings::init(SERVICE_NAME).unwrap();
//!
//!     let settings_path = Path::new(DEFAULT_SETTINGS_DIR).join(format!("{SERVICE_NAME}.json"));
//!     print_json(&settings_path); // {}
//!
//!     let mut value = settings.read_or_insert("test_value", 42i32).unwrap();
//!     print_json(&settings_path);
//!     // { "test_value": ${some number} } if existed or
//!     // { "test_value": 42 } if a newly created entry
//!
//!     value.update(11).unwrap();
//!     print_json(&settings_path); // { "test_value": 11 }
//!
//!     value.clear().unwrap(); // {}
//! }
//! ```
pub mod entry;
pub mod settings;

pub use entry::Entry;
pub use settings::Settings;
