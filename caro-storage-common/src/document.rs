use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Document<T> {
    value: T,
}

impl<T: Serialize + DeserializeOwned> Document<T> {
    pub fn into_bytes(value: T) -> Vec<u8> {
        bson::to_vec(&Self { value }).unwrap()
    }

    pub fn value(self) -> T {
        self.value
    }
}
