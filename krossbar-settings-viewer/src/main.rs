//! Krossbar settings viewer
//!
//! A convenient tool to read Krossbar settings files.
//! It uses [DEFAULT_SETTINGS_DIR] by default and is able to query individual
//! fields, which is simpler, than reading corresponding JSON by hands.
//!
//! # Usage
//!
//! ```sh
//! Krossbar settings viewer
//!
//! USAGE:
//!     krossbar-settings-viewer <TARGET_SERVICE> [KEY]
//!
//! ARGS:
//!     <TARGET_SERVICE>    Service to monitor
//!     <KEY>               
//!
//! OPTIONS:
//!     -h, --help       Print help information
//!     -V, --version    Print version information
//! ```
use std::path::Path;

use clap::{self, Parser};
use colored::*;
use krossbar_settings_common::{settings::Settings, Error, DEFAULT_SETTINGS_DIR};
use serde_json::Value;

/// Krossbar storage viewer
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Service to monitor
    #[clap()]
    pub target_service: String,

    #[clap()]
    pub key: Option<String>,
}

fn print_values(values: Vec<(String, Value)>) {
    for (name, value) in values {
        println!("{}: {}", name.bright_blue(), value)
    }
}

fn main() {
    let args = Args::parse();
    let settings_path =
        Path::new(DEFAULT_SETTINGS_DIR).join(format!("{}.json", &args.target_service));

    let mut settings = Settings::open(&settings_path).unwrap();

    if let Some(key) = args.key {
        match settings.get::<Value>(&key) {
            Ok(value) => println!("{value}"),
            Err(Error::NotFound) => println!("null"),
            e => {
                e.expect("Failed to read settings");
            }
        }
    } else {
        print_values(settings.list_values().unwrap());
    }
}
