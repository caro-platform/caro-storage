use krossbar_settings_lib::Settings;

fn main() {
    {
        let settings = Settings::init("krossbar.storage.example").unwrap();

        let value = settings.load::<i32>("test_value");
        assert!(value.is_err());

        let mut value = settings.load_or_default("test_value", 42i32).unwrap();
        assert_eq!(*value, 42);

        value.update(11).unwrap();
        assert_eq!(*value, 11);
    }

    {
        let settings = Settings::init("krossbar.storage.example").unwrap();
        let value = settings.load::<i32>("test_value").unwrap();
        assert_eq!(*value, 11);

        value.clear();
    }

    let settings = Settings::init("krossbar.storage.example").unwrap();

    let value = settings.load::<i32>("test_value");
    assert!(value.is_err());
}
