use std::path::Path;

use clap::{self, Parser};
use colored::*;
use krossbar_settings_common::{settings::Settings, DEFAULT_SETTINGS_DIR};
use serde_json::Value;

/// Krossbar storage viewer
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Service to monitor
    #[clap(value_parser)]
    pub target_service: String,
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

    print_values(settings.list_values().unwrap());
}
