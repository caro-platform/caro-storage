use std::sync::Arc;

use crate::value::Value;
use serde::{de::DeserializeOwned, Serialize};

use caro_storage_common::{get_storage_dir, queries::QueryRunner};

pub struct Storage {
    runner: Arc<QueryRunner>,
}

impl Storage {
    pub fn open(service_name: &str) -> crate::Result<Self> {
        let this = Self {
            runner: Arc::new(QueryRunner::open(&format!(
                "{}{}.db3",
                get_storage_dir(),
                service_name
            ))?),
        };

        this.runner.create_table()?;
        Ok(this)
    }

    pub fn load<T: Serialize + DeserializeOwned>(&self, name: &str) -> Value<T> {
        Value::new(name, self.runner.clone())
    }
}
