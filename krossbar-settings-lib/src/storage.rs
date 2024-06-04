use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::value::Value;
use serde::{de::DeserializeOwned, Serialize};

use krossbar_settings_common::{settings, DEFAULT_STORAGE_DIR};

/// Settings connections handle
pub struct Settings {
    /// Common settings handle
    inner: Arc<Mutex<settings::Settings>>,
}

impl Settings {
    /// Open storage for a **service_name** service
    pub fn init(service_name: &str) -> crate::Result<Self> {
        let settings_path = Path::new(DEFAULT_STORAGE_DIR).join(service_name);
        let settings = settings::Settings::open(&settings_path)?;

        Ok(Self {
            inner: Arc::new(Mutex::new(settings)),
        })
    }

    /// Create value **name** handle
    pub fn load<T: Serialize + DeserializeOwned>(&self, name: &str) -> crate::Result<Value<T>> {
        
        Value::new(name, self.inner.clone())
    }
}
