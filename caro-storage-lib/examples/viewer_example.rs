use caro_storage::Storage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ComplexStructure {
    integer: i32,
    string: String,
    vec: Vec<bool>,
}

fn main() {
    let storage = Storage::open("caro.viewer.example").unwrap();

    storage.load::<i32>("int").set(11);
    storage.load::<String>("string").set("Hello, world!".into());
    storage
        .load::<ComplexStructure>("struct")
        .set(ComplexStructure {
            integer: 42,
            string: "Test".into(),
            vec: vec![true, false],
        });
}
