use std::{fs::File, io::Read, path::Path};

use tempdir::TempDir;

use krossbar_settings_lib::Settings;

fn print_json(path: &Path) {
    let mut file = File::open(path).unwrap();

    let mut result = String::new();
    let data = file.read_to_string(&mut result).unwrap();
    println!("Settings file content: '{data}'");
}

#[test]
fn test_values() {
    let settings_tempdir =
        TempDir::new("krossbar_hub_socket_dir").expect("Failed to create socket tempdir");

    let file_path = settings_tempdir
        .path()
        .join(format!("krossbar.storage.example.json"));

    {
        let settings =
            Settings::init_at("krossbar.storage.example", settings_tempdir.path()).unwrap();
        print_json(&file_path);

        let value = settings.load::<i32>("test_value");
        assert!(value.is_err());

        let mut value = settings.load_or_default("test_value", 42i32).unwrap();
        assert_eq!(*value, 42);

        value.update(11).unwrap();
        assert_eq!(*value, 11);
        print_json(&file_path);
    }

    {
        let storage =
            Settings::init_at("krossbar.storage.example", settings_tempdir.path()).unwrap();
        let value = storage.load::<i32>("test_value").unwrap();
        assert_eq!(*value, 11);

        print_json(&file_path);

        value.clear();
        print_json(&file_path);
    }

    let storage = Settings::init_at("krossbar.storage.example", settings_tempdir.path()).unwrap();

    let value = storage.load::<i32>("test_value");
    println!("{:?}", value);
    assert!(value.is_err());
}
