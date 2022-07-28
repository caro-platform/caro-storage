use std::{marker::PhantomData, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};

use caro_storage_common::{document::Document, queries::QueryRunner};

pub struct Value<T> {
    name: String,
    runner: Arc<QueryRunner>,
    _pd: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> Value<T> {
    pub fn new(name: &str, runner: Arc<QueryRunner>) -> Self {
        Self {
            name: name.into(),
            runner,
            _pd: PhantomData,
        }
    }

    pub fn get(&self) -> crate::Result<T> {
        let blob: Vec<u8> = self.runner.get_value(&self.name)?;

        let bson = bson::from_slice(blob.as_ref())?;

        let result = bson::from_bson::<Document<T>>(bson)?;
        Ok(result.value())
    }

    pub fn get_default(&self, default: T) -> T {
        match self.get() {
            Err(_) => {
                self.set(default);
                self.get().unwrap()
            }
            Ok(value) => value,
        }
    }

    pub fn set(&self, value: T) {
        let bytes = Document::into_bytes(value);
        self.runner.set_value(&self.name, &bytes);
    }

    pub fn clear(&self) {
        self.runner.clear_value(&self.name);
    }
}
