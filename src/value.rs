use std::{marker::PhantomData, sync::Arc};

use crate::storage::VALUES_TABLE_NAME;
use rusqlite::{params, Connection};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Document<T> {
    pub inner: T,
}

pub struct Value<T> {
    name: String,
    connection: Arc<Connection>,
    _pd: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> Value<T> {
    pub fn new(name: &str, connection: Arc<Connection>) -> Self {
        Self {
            name: name.into(),
            connection,
            _pd: PhantomData,
        }
    }

    pub fn get(&self) -> crate::Result<T> {
        let blob: Vec<u8> = self.connection.query_row(
            &format!("SELECT value FROM {} WHERE name=?", VALUES_TABLE_NAME),
            [&self.name],
            |row| row.get(0),
        )?;

        let bson = bson::from_slice(blob.as_ref())?;

        let result = bson::from_bson::<Document<T>>(bson)?;
        Ok(result.inner)
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
        let bson = bson::to_vec(&Document { inner: value }).unwrap();

        let _ = self.connection.execute(
            &format!(
                "INSERT OR REPLACE INTO {} (name, value) VALUES (?1, ?2)",
                VALUES_TABLE_NAME
            ),
            params![&self.name, &bson],
        );
    }

    pub fn clear(&self) {
        let _ = self.connection.execute(
            &format!("DELETE FROM {} WHERE name=?", VALUES_TABLE_NAME),
            [&self.name],
        );
    }
}
