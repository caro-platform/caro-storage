use std::io::Error;

use krossbar_storage::Storage;

fn main() -> Result<(), Error> {
    let storage = Storage::open("krossbar.storage.example").unwrap();

    let value = storage.load::<i32>("test_value");
    assert!(value.get().is_err());

    assert_eq!(value.get_default(42), 42);
    assert_eq!(value.get().unwrap(), 42);

    value.set(11);
    assert_eq!(value.get().unwrap(), 11);

    value.clear();
    assert!(value.get().is_err());

    Ok(())
}
