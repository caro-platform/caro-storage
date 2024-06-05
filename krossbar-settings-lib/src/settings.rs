use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::entry::Entry;
use serde::{de::DeserializeOwned, Serialize};

use krossbar_settings_common::{settings, Error, Result, DEFAULT_SETTINGS_DIR};

/// Settings connections handle
pub struct Settings {
    /// Common settings handle
    inner: Arc<Mutex<settings::Settings>>,
}

impl Settings {
    /// Open settings for the **service_name** service
    pub fn init(service_name: &str) -> Result<Self> {
        let settings_path = Path::new(DEFAULT_SETTINGS_DIR).join(format!("{service_name}.json"));
        let settings = settings::Settings::open(&settings_path)?;

        Ok(Self {
            inner: Arc::new(Mutex::new(settings)),
        })
    }

    /// Open settings for the **service_name** service in the
    /// **settings_dir** directory
    pub fn init_at(settings_dir: &Path, service_name: &str) -> Result<Self> {
        let settings_path = settings_dir.join(format!("{service_name}.json"));
        let settings = settings::Settings::open(&settings_path)?;

        Ok(Self {
            inner: Arc::new(Mutex::new(settings)),
        })
    }

    /// Load value from settings. Returns [Error::NotFound] if no entry found
    pub fn read<T: Serialize + DeserializeOwned>(&self, name: &str) -> Result<Entry<T>> {
        let value = self.inner.lock().unwrap().get(name)?;

        Ok(Entry::new(name, value, self.inner.clone()))
    }

    /// Load value from settings. Set value to **default** if not found
    pub fn read_or_insert<T: Serialize + DeserializeOwned>(
        &self,
        name: &str,
        default: T,
    ) -> Result<Entry<T>> {
        let (value, update) = match self.inner.lock().unwrap().get(name) {
            Ok(value) => (value, false),
            Err(Error::NotFound) => (default, true),
            Err(e) => return Err(e),
        };

        let mut result = Entry::new(name, value, self.inner.clone());

        if update {
            result.save()?;
        }

        Ok(result)
    }
}
