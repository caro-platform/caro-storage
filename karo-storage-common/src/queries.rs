use rusqlite::{params, Connection};

pub(crate) const VALUES_TABLE_NAME: &str = "storage";

/// Utility struct to run SQL queries
pub struct QueryRunner {
    /// SQLite connections
    connection: Connection,
}

impl QueryRunner {
    pub fn open(db_path: &str) -> crate::Result<Self> {
        Ok(Self {
            connection: Connection::open(db_path)?,
        })
    }

    pub fn create_table(&self) -> crate::Result<()> {
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

    pub fn get_value(&self, name: &str) -> crate::Result<Vec<u8>> {
        let result: Vec<u8> = self.connection.query_row(
            &format!("SELECT value FROM {} WHERE name=?", VALUES_TABLE_NAME),
            [&name],
            |row| row.get(0),
        )?;

        Ok(result)
    }

    pub fn set_value(&self, name: &str, value: &Vec<u8>) {
        let _ = self.connection.execute(
            &format!(
                "INSERT OR REPLACE INTO {} (name, value) VALUES (?1, ?2)",
                VALUES_TABLE_NAME
            ),
            params![name, value],
        );
    }

    pub fn clear_value(&self, name: &str) {
        let _ = self.connection.execute(
            &format!("DELETE FROM {} WHERE name=?", VALUES_TABLE_NAME),
            [name],
        );
    }

    pub fn list_values(&self) -> crate::Result<Vec<(String, Vec<u8>)>> {
        let mut statement = self
            .connection
            .prepare(&format!("SELECT name, value FROM {}", VALUES_TABLE_NAME))?;

        let values_iter = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

        Ok(values_iter.map(|res| res.unwrap()).collect())
    }
}
