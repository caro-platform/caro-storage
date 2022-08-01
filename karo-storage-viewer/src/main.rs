use bson::Bson;
use clap::{self, Parser};
use colored::*;

use karo_storage_common::{document::Document, get_storage_dir, queries::QueryRunner};

/// Karo storage viewer
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Service to monitor
    #[clap(value_parser)]
    pub target_service: String,
}

fn print_values(values: Vec<(String, Vec<u8>)>) {
    for (name, value) in values.into_iter() {
        let bson = bson::from_slice(&value).unwrap();
        let document = bson::from_bson::<Document<Bson>>(bson).unwrap();

        println!("{}: {}", name.bright_blue(), document.value())
    }
}

fn main() {
    let args = Args::parse();

    let runner = QueryRunner::open(&format!(
        "{}{}.db3",
        get_storage_dir(),
        &args.target_service
    ))
    .unwrap();

    print_values(runner.list_values().unwrap());
}
