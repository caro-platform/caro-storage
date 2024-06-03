use krossbar_settings_common::settings::Settings;
use tempdir::TempDir;

#[tokio::test()]
async fn simple_test() {
    let settings_dir =
        TempDir::new("krossbar_hub_socket_dir").expect("Failed to create socket tempdir");

    let settings_path = settings_dir.path().join("krossbar_test.settings");

    let mut settings = Settings::open(&settings_path).await.unwrap();

    assert!(settings.get::<()>("non-existing-settings").await.is_err());

    settings.set("test-settings", &42i32).await.unwrap();
    assert_eq!(settings.get::<i32>("test-settings").await.unwrap(), 42i32);
}
