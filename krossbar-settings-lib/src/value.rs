use std::{marker::PhantomData, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};

use krossbar_storage_common::{document::Document, queries::QueryRunner};

/// Persistend value handle
pub struct Value<T> {
    /// Value name
    name: String,
    /// SQL queries runner
    runner: Arc<QueryRunner>,
    _pd: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> Value<T> {
    pub(crate) fn new(name: &str, runner: Arc<QueryRunner>) -> Self {
        Self {
            name: name.into(),
            runner,
            _pd: PhantomData,
        }
    }

    /// Get value from te storage
    /// **Returns** error if value doens't exist in the storage
    pub fn get(&self) -> crate::Result<T> {
        let blob: Vec<u8> = self.runner.get_value(&self.name)?;

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
        self.runner.set_value(&self.name, &bytes);
    }

    /// Delete value from the storage
    pub fn clear(&self) {
        self.runner.clear_value(&self.name);
    }
}
