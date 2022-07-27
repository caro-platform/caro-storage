use std::sync::Arc;

use crate::value::Value;
use rusqlite::Connection;
use serde::{de::DeserializeOwned, Serialize};

const STORAGE_DIR: &str = "/etc/caro.storage.d/";
pub(crate) const VALUES_TABLE_NAME: &str = "storage";

pub struct Storage {
    connection: Arc<Connection>,
}

impl Storage {
    pub fn open(service_name: &str) -> crate::Result<Self> {
        let this = Self {
            connection: Arc::new(Connection::open(&format!(
                "{}{}.db3",
                STORAGE_DIR, service_name
            ))?),
        };

        this.check_table()?;
        Ok(this)
    }

    fn check_table(&self) -> crate::Result<()> {
        self.connection.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                name STRING PRIMARY KEY,
                value BLOB
            )",
                VALUES_TABLE_NAME
            ),
            [],
        )?;

        Ok(())
    }

    pub fn load<T: Serialize + DeserializeOwned>(&self, name: &str) -> Value<T> {
        Value::new(name, self.connection.clone())
    }
}
