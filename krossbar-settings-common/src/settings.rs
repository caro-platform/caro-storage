use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

/// Utility struct to run SQL queries
pub struct Settings {
    /// SQLite connections
    settings_file: File,
}

impl Settings {
    pub async fn open(db_path: &Path) -> crate::Result<Self> {
        // No settings fiel. Let's create and init one
        let settings_file = if !Path::new(db_path).exists() {
            let mut file = File::create_new(db_path)
                .await
                .map_err(|e| crate::Error::IoError(e.to_string()))?;

            file.write_all("{}".as_bytes())
                .await
                .map_err(|e| crate::Error::IoError(e.to_string()))?;

            file
        // Existing settings file
        } else {
            File::open(db_path)
                .await
                .map_err(|e| crate::Error::IoError(e.to_string()))?
        };

        Ok(Self { settings_file })
    }

    pub async fn get<T: DeserializeOwned>(&mut self, name: &str) -> crate::Result<T> {
        self.modify_settings(false, |map| {
            if let Some(settings_value) = map.remove(name) {
                serde_json::from_value(settings_value)
                    .map_err(|e| crate::Error::Type(e.to_string()))
            } else {
                Err(crate::Error::NotFound)
            }
        })
        .await
    }

    pub async fn set<T: Serialize>(&mut self, name: &str, value: &T) -> crate::Result<()> {
        self.modify_settings(true, |map| {
            let json_value =
                serde_json::to_value(value).map_err(|e| crate::Error::Type(e.to_string()))?;

            map.insert(name.to_owned(), json_value);
            Ok(())
        })
        .await
    }

    pub async fn clear(&mut self, name: &str) -> crate::Result<()> {
        self.modify_settings(true, |map| {
            map.remove(name);
            Ok(())
        })
        .await
    }

    pub async fn list_values(&mut self) -> crate::Result<Vec<(String, Value)>> {
        self.modify_settings(false, |map| {
            let keys: Vec<String> = map.keys().cloned().collect();

            Ok(keys
                .into_iter()
                .map(|key| {
                    let value = map.remove(&key).unwrap();
                    (key, value)
                })
                .collect())
        })
        .await
    }

    async fn modify_settings<T>(
        &mut self,
        write_back: bool,
        func: impl Fn(&mut serde_json::Map<String, Value>) -> crate::Result<T>,
    ) -> crate::Result<T> {
        let mut data = Vec::new();
        self.settings_file
            .read_to_end(&mut data)
            .await
            .map_err(|e| crate::Error::IoError(e.to_string()))?;

        let json: Value =
            serde_json::from_slice(&data).map_err(|e| crate::Error::Corrupted(e.to_string()))?;

        if let Value::Object(mut map) = json {
            let result = func(&mut map);

            if write_back && result.is_ok() {
                let data_to_write = serde_json::to_vec_pretty(&Value::Object(map))
                    .map_err(|e| crate::Error::Type(e.to_string()))?;

                self.settings_file
                    .write_all(&data_to_write)
                    .await
                    .map_err(|e| crate::Error::IoError(e.to_string()))?;
            }

            result
        } else {
            Err(crate::Error::Corrupted(
                "Root settings elemetn is not an Object".into(),
            ))
        }
    }
}
