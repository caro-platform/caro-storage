use std::{env, sync::Arc};

use crate::value::Value;
use rusqlite::Connection;
use serde::{de::DeserializeOwned, Serialize};

pub const STORAGE_DIR_PATH_ENV: &str = "CARO_STORAGE_DIR_PATH";
const DEFAULT_STORAGE_DIR: &str = "/etc/caro.storage.d/";

pub(crate) const VALUES_TABLE_NAME: &str = "storage";

fn get_storage_dir() -> String {
    if let Ok(path) = env::var(STORAGE_DIR_PATH_ENV) {
        path
    } else {
        DEFAULT_STORAGE_DIR.into()
    }
}

pub struct Storage {
    connection: Arc<Connection>,
}

impl Storage {
    pub fn open(service_name: &str) -> crate::Result<Self> {
        let this = Self {
            connection: Arc::new(Connection::open(&format!(
                "{}{}.db3",
                get_storage_dir(),
                service_name
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
