[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/krossbar-settings-lib.svg
[crates-url]: https://crates.io/crates/krossbar-settings-lib
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/krossbar-platform/krossbar-bus/blob/main/LICENSE
[actions-badge]: https://github.com/krossbar-platform/krossbar-settings/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/krossbar-platform/krossbar-settings/actions/workflows/ci.yml

# krossbar-settings-lib

Krossbar settings lib

The library provides a convenient wrapper for reading and writing
settings into JSON file.

Use [Settings](https://docs.rs/krossbar-settings-lib/latest/krossbar_settings_lib/settings/struct.Settings.html) handle to open settings file. Using the handle you can
read, insert or delete settings entries from the inner JSON.

To create an [Entry](https://docs.rs/krossbar-settings-lib/latest/krossbar_settings_lib/entry/struct.Entry.html) use [Settings::read] or [Settings::read_or_insert] method, which tries to read
the value from the settings file, or adds new JSON entry if doesn't exist.

Using [Entry](https://docs.rs/krossbar-settings-lib/latest/krossbar_settings_lib/entry/struct.Entry.html) handle you can modify corresponding JSON settings file.

## Examples
```rust
use std::path::Path;

use krossbar_settings_common::DEFAULT_SETTINGS_DIR;
use krossbar_settings_lib::Settings;

const SERVICE_NAME: &str = "krossbar.storage.example";

fn print_json(path: &Path) {
    let data = std::fs::read_to_string(path).unwrap();
    println!("Settings: '{data}'");
}

fn manage_settings() {
    let settings = Settings::init(SERVICE_NAME).unwrap();

    let settings_path = Path::new(DEFAULT_SETTINGS_DIR).join(format!("{SERVICE_NAME}.json"));
    print_json(&settings_path); // {}

    let mut value = settings.read_or_insert("test_value", 42i32).unwrap();
    print_json(&settings_path);
    // { "test_value": ${some number} } if existed or
    // { "test_value": 42 } if a newly created entry

    value.update(11).unwrap();
    print_json(&settings_path); // { "test_value": 11 }

    value.clear().unwrap(); // {}
}
```
