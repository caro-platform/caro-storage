use std::path::Path;

use tempdir::TempDir;

use krossbar_settings_lib::Settings;

fn print_json(path: &Path) {
    let data = std::fs::read_to_string(path).unwrap();

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
            Settings::init_at(settings_tempdir.path(), "krossbar.storage.example").unwrap();
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
        let settings =
            Settings::init_at(settings_tempdir.path(), "krossbar.storage.example").unwrap();
        let value = settings.load::<i32>("test_value").unwrap();
        assert_eq!(*value, 11);

        let mut value2 = settings.load_or_default("test_value2", 41i32).unwrap();
        assert_eq!(*value2, 41);
        value2.update(42).unwrap();

        print_json(&file_path);

        value.clear().unwrap();
        print_json(&file_path);
    }

    let settings = Settings::init_at(settings_tempdir.path(), "krossbar.storage.example").unwrap();

    let value = settings.load::<i32>("test_value");
    assert!(value.is_err());

    let value = settings.load::<i32>("test_value2").unwrap();
    assert_eq!(*value, 42);
}
