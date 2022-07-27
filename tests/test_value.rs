use std::env;

use tempdir::TempDir;

use caro_storage::{storage::STORAGE_DIR_PATH_ENV, Storage};

#[test]
fn test_values() {
    let storage_dir = TempDir::new("caro_hub_socket_dir").expect("Failed to create socket tempdir");
    let storage_dir_path = storage_dir.path().as_os_str();

    env::set_var(STORAGE_DIR_PATH_ENV, storage_dir_path);

    let storage = Storage::open("caro.storage.example").unwrap();

    let value = storage.load::<i32>("test_value");
    assert!(value.get().is_err());

    assert_eq!(value.get_default(42), 42);
    assert_eq!(value.get().unwrap(), 42);

    value.set(11);
    assert_eq!(value.get().unwrap(), 11);

    value.clear();
    assert!(value.get().is_err());
}
