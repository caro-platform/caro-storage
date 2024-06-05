use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::Deref,
    sync::{Arc, Mutex},
};

use krossbar_settings_common::{settings, Result};
use serde::{de::DeserializeOwned, Serialize};

/// Settings entry handle
pub struct Entry<T> {
    /// Name
    name: String,
    /// Value
    value: T,
    /// Settings handle
    settings: Arc<Mutex<settings::Settings>>,
}

impl<T: Serialize + DeserializeOwned> Entry<T> {
    pub(crate) fn new(name: &str, value: T, settings: Arc<Mutex<settings::Settings>>) -> Self {
        Self {
            name: name.into(),
            value,
            settings,
        }
    }

    /// Get the value
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Update the entry to **value**.
    /// Writes new value into the settings file
    pub fn update(&mut self, value: T) -> Result<()> {
        self.value = value;
        self.settings.lock().unwrap().set(&self.name, &self.value)
    }

    /// Write entry value into the settings file
    pub fn save(&mut self) -> Result<()> {
        self.settings.lock().unwrap().set(&self.name, &self.value)
    }

    /// Delete entry from the settings file
    pub fn clear(&self) -> Result<()> {
        self.settings.lock().unwrap().clear(&self.name)
    }
}

impl<T: Serialize + DeserializeOwned> Deref for Entry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: Serialize + DeserializeOwned + Display> Display for Entry<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value)
    }
}

impl<T: Serialize + DeserializeOwned + Debug> Debug for Entry<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Value {{ name: {}, value: {:?} }}",
            self.name, self.value
        )
    }
}
