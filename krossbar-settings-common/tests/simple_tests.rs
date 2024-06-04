use krossbar_settings_common::settings::Settings;
use serde_json::Value;
use tempdir::TempDir;

#[test]
fn simple_test() {
    let settings_dir =
        TempDir::new("krossbar_hub_socket_dir").expect("Failed to create socket tempdir");

    let settings_path = settings_dir.path().join("krossbar_test.settings");

    let mut settings = Settings::open(&settings_path).unwrap();

    assert!(settings.get::<()>("non-existing-settings").is_err());

    settings.set("test-settings-0", &42i32).unwrap();
    assert_eq!(settings.get::<i32>("test-settings-0").unwrap(), 42i32);

    settings.set("test-settings-1", &42i32).unwrap();
    assert_eq!(settings.get::<i32>("test-settings-1").unwrap(), 42i32);

    let values = settings.list_values().unwrap();
    assert_eq!(values[0].0, "test-settings-0");
    assert!(matches!(values[0].1, Value::Number(_)));

    assert_eq!(values[1].0, "test-settings-1");
    assert!(matches!(values[1].1, Value::Number(_)));

    settings.clear("test-settings-1").unwrap();
    assert!(settings.get::<i32>("test-settings-1").is_err());

    settings.clear("test-settings-0").unwrap();
    assert!(settings.get::<i32>("test-settings-0").is_err());
    assert!(settings.get::<i32>("test-settings-1").is_err());

    settings.set("test-settings-2", &42i32).unwrap();
    assert_eq!(settings.get::<i32>("test-settings-2").unwrap(), 42i32);

    let values = settings.list_values().unwrap();
    assert_eq!(values[0].0, "test-settings-2");
    assert!(matches!(values[0].1, Value::Number(_)));
}
