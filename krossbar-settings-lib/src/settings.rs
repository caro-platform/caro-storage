use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::value::Value;
use serde::{de::DeserializeOwned, Serialize};

use krossbar_settings_common::{settings, Error, Result, DEFAULT_STORAGE_DIR};

/// Settings connections handle
pub struct Settings {
    /// Common settings handle
    inner: Arc<Mutex<settings::Settings>>,
}

impl Settings {
    /// Open storage for a **service_name** service
    pub fn init(service_name: &str) -> Result<Self> {
        let settings_path = Path::new(DEFAULT_STORAGE_DIR).join(format!("{service_name}.json"));
        let settings = settings::Settings::open(&settings_path)?;

        Ok(Self {
            inner: Arc::new(Mutex::new(settings)),
        })
    }

    /// Open storage for a **service_name** service
    pub fn init_at(service_name: &str, settings_dir: &Path) -> Result<Self> {
        let settings_path = settings_dir.join(format!("{service_name}.json"));
        let settings = settings::Settings::open(&settings_path)?;

        Ok(Self {
            inner: Arc::new(Mutex::new(settings)),
        })
    }

    /// Create value **name** handle
    pub fn load<T: Serialize + DeserializeOwned>(&self, name: &str) -> Result<Value<T>> {
        let value = self.inner.lock().unwrap().get(name)?;

        Ok(Value::new(name, value, self.inner.clone()))
    }

    /// Create value **name** handle. Sets to **deafult** if doesn't exist
    pub fn load_or_default<T: Serialize + DeserializeOwned>(
        &self,
        name: &str,
        default: T,
    ) -> Result<Value<T>> {
        let value = match self.inner.lock().unwrap().get(name) {
            Ok(value) => value,
            Err(Error::NotFound) => default,
            Err(e) => return Err(e),
        };

        Ok(Value::new(name, value, self.inner.clone()))
    }
}
