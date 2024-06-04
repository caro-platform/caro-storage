use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::Settings;

/// Persistend value handle
pub struct Value<T> {
    /// Name
    name: String,
    /// Value
    value: T,
    /// SQL queries runner
    settings: Arc<Mutex<Settings>>,
}

impl<T: Serialize + DeserializeOwned> Value<T> {
    pub(crate) fn new(name: &str, value: T, settings: Arc<Mutex<Settings>>) -> Self {
        Self {
            name: name.into(),
            value,
            settings,
        }
    }

    /// Get value from te storage
    /// **Returns** error if value doens't exist in the storage
    pub fn get(&self) -> crate::Result<T> {
        let blob: Vec<u8> = self.settings.get_value(&self.name)?;

        let bson = bson::from_slice(blob.as_ref())?;

        let document = bson::from_bson::<Document<T>>(bson)?;
        Ok(document.value())
    }

    /// Get value from te storage. If the value doesn't exists, sets it to the **default** value
    /// **Returns** current value data, or default value if doesn't exists
    pub fn get_default(&self, default: T) -> T {
        match self.get() {
            Err(_) => {
                self.set(default);
                self.get().unwrap()
            }
            Ok(value) => value,
        }
    }

    /// Set value to **value**
    pub fn set(&self, value: T) {
        let bytes = Document::into_bytes(value);
        self.settings.set_value(&self.name, &bytes);
    }

    /// Delete value from the storage
    pub fn clear(&self) {
        self.settings.clear_value(&self.name);
    }
}
