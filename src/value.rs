pub struct Value<T> {
    value: T,
}

impl<T> Value<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Default> Default for Value<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
        }
    }
}
