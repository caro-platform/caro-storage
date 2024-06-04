use serde::{Deserialize, Serialize};

use krossbar_settings_lib::Settings;

#[derive(Serialize, Deserialize)]
struct ComplexStructure {
    integer: i32,
    string: String,
    vec: Vec<bool>,
}

fn main() {
    let storage = Settings::init("krossbar.viewer.example").unwrap();

    storage.load::<i32>("int").unwrap().update(11).unwrap();

    storage
        .load::<String>("string")
        .unwrap()
        .update("Hello, world!".into())
        .unwrap();

    storage
        .load::<ComplexStructure>("struct")
        .unwrap()
        .update(ComplexStructure {
            integer: 42,
            string: "Test".into(),
            vec: vec![true, false],
        })
        .unwrap();
}
