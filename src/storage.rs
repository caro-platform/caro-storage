use std::error::Error;

use diesel::{sqlite::SqliteConnection, Connection};

use crate::value::Value;

const STORAGE_DIR: &str = "/etc/caro.storage.d/";

pub struct Storage {
    connection: SqliteConnection,
}

impl Storage {
    pub fn connect(service_name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection: SqliteConnection::establish(&format!(
                "{}{}.caro.storage",
                STORAGE_DIR, service_name
            ))?,
        })
    }

    // pub fn load<T>(&self) -> Value<T> {
    //     self.connection.
    // }
}
