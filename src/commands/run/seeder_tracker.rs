use crate::commands::run::drivers::SchemeDriver;
use grow_core::SqlValue;
use std::error::Error;
use std::str::FromStr;

pub struct SeederTracker {
    scheme: SchemeDriver,
    database_url: String,
}

impl SeederTracker {
    pub fn new(database_url: String) -> Result<Self, Box<dyn Error>> {
        let scheme = SchemeDriver::from_str(&database_url)?;
        Ok(Self {
            scheme,
            database_url,
        })
    }

    /// Detects the specific database type from the URL
    fn get_database_type(&self) -> &str {
        if let Some((scheme, _)) = self.database_url.split_once("://") {
            scheme
        } else {
            "unknown"
        }
    }

    /// Ensures the seeds table exists in the database
    pub async fn ensure_seeds_table(&self) -> Result<(), Box<dyn Error>> {
        let create_table_sql = match self.scheme {
            SchemeDriver::Mock => {
                "CREATE TABLE IF NOT EXISTS seeds (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                name TEXT NOT NULL UNIQUE
            )"
            }
            #[cfg(feature = "libsql")]
            SchemeDriver::Libsql => {
                "CREATE TABLE IF NOT EXISTS seeds (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                name TEXT NOT NULL UNIQUE
            )"
            }
            #[cfg(feature = "sqlx")]
            SchemeDriver::Sqlx => {
                // Generate database-specific SQL
                match self.get_database_type() {
                    "postgres" => {
                        "CREATE TABLE IF NOT EXISTS seeds (
                            id SERIAL PRIMARY KEY,
                            timestamp BIGINT NOT NULL,
                            name TEXT NOT NULL UNIQUE
                        )"
                    }
                    "mysql" => {
                        "CREATE TABLE IF NOT EXISTS seeds (
                            id INT AUTO_INCREMENT PRIMARY KEY,
                            timestamp BIGINT NOT NULL,
                            name VARCHAR(255) NOT NULL UNIQUE
                        )"
                    }
                    "sqlite" => {
                        "CREATE TABLE IF NOT EXISTS seeds (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            timestamp INTEGER NOT NULL,
                            name TEXT NOT NULL UNIQUE
                        )"
                    }
                    _ => {
                        // Fallback to PostgreSQL syntax for unknown databases
                        "CREATE TABLE IF NOT EXISTS seeds (
                            id SERIAL PRIMARY KEY,
                            timestamp BIGINT NOT NULL,
                            name TEXT NOT NULL UNIQUE
                        )"
                    }
                }
            }
            #[cfg(feature = "surrealdb")]
            SchemeDriver::Surrealdb => {
                // SurrealDB doesn't need explicit table creation for basic operations
                return Ok(());
            }
        };

        match self.scheme {
            SchemeDriver::Mock => {
                println!("{}", create_table_sql);
            }
            #[cfg(feature = "libsql")]
            SchemeDriver::Libsql => {
                self.execute_libsql_query(create_table_sql, vec![]).await?;
            }
            #[cfg(feature = "sqlx")]
            SchemeDriver::Sqlx => {
                self.execute_sqlx_query(create_table_sql, vec![]).await?;
            }
            #[cfg(feature = "surrealdb")]
            SchemeDriver::Surrealdb => {
                // SurrealDB doesn't need explicit table creation for basic operations
                // Tables are created automatically when inserting data
            }
        }

        Ok(())
    }

    /// Checks if a seeder has already been executed
    pub async fn is_seeder_executed(
        &self,
        seeder_name: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let query = "SELECT COUNT(*) FROM seeds WHERE name = ?";

        match self.scheme {
            SchemeDriver::Mock => {
                println!("{} (name: {})", query, seeder_name);
                Ok(false) // Mock always returns false to allow execution
            }
            #[cfg(feature = "libsql")]
            SchemeDriver::Libsql => self.check_seeder_libsql(seeder_name).await,
            #[cfg(feature = "sqlx")]
            SchemeDriver::Sqlx => self.check_seeder_sqlx(seeder_name).await,
            #[cfg(feature = "surrealdb")]
            SchemeDriver::Surrealdb => {
                self.check_seeder_surrealdb(seeder_name).await
            }
        }
    }

    /// Marks a seeder as executed by inserting a record in the seeds table
    pub async fn mark_seeder_executed(
        &self,
        seeder_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let query = "INSERT INTO seeds (timestamp, name) VALUES (?, ?)";

        match self.scheme {
            SchemeDriver::Mock => {
                println!(
                    "{} (timestamp: {}, name: {})",
                    query, timestamp, seeder_name
                );
            }
            #[cfg(feature = "libsql")]
            SchemeDriver::Libsql => {
                self.execute_libsql_query(
                    query,
                    vec![
                        SqlValue::Integer(timestamp),
                        SqlValue::Text(seeder_name.to_string()),
                    ],
                )
                .await?;
            }
            #[cfg(feature = "sqlx")]
            SchemeDriver::Sqlx => {
                // Update query syntax for SQLx (use $1, $2 instead of ?)
                let sqlx_query =
                    "INSERT INTO seeds (timestamp, name) VALUES ($1, $2)";
                grow_sqlx::execute_query_with_params(
                    self.database_url.clone(),
                    sqlx_query,
                    timestamp,
                    seeder_name,
                )
                .await?;
            }
            #[cfg(feature = "surrealdb")]
            SchemeDriver::Surrealdb => {
                self.mark_seeder_surrealdb(seeder_name, timestamp).await?;
            }
        }

        Ok(())
    }

    #[cfg(feature = "libsql")]
    async fn execute_libsql_query(
        &self,
        query: &str,
        params: Vec<SqlValue>,
    ) -> Result<(), Box<dyn Error>> {
        grow_libsql::execute_query(self.database_url.clone(), query, params)
            .await
            .map_err(|e| e.into())
    }

    #[cfg(feature = "libsql")]
    async fn check_seeder_libsql(
        &self,
        seeder_name: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let query = "SELECT COUNT(*) FROM seeds WHERE name = ?";
        let count = grow_libsql::query_single_int(
            self.database_url.clone(),
            query,
            vec![SqlValue::Text(seeder_name.to_string())],
        )
        .await
        .map_err(|e| -> Box<dyn Error> { e.into() })?;

        Ok(count > 0)
    }

    #[cfg(feature = "sqlx")]
    async fn execute_sqlx_query(
        &self,
        query: &str,
        _params: Vec<SqlValue>,
    ) -> Result<(), Box<dyn Error>> {
        grow_sqlx::execute_query(self.database_url.clone(), query)
            .await
            .map_err(|e| e.into())
    }

    #[cfg(feature = "sqlx")]
    async fn check_seeder_sqlx(
        &self,
        seeder_name: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let query = "SELECT COUNT(*) FROM seeds WHERE name = $1";
        let count = grow_sqlx::query_single_int(
            self.database_url.clone(),
            query,
            seeder_name,
        )
        .await
        .map_err(|e| -> Box<dyn Error> { e.into() })?;

        Ok(count > 0)
    }

    #[cfg(feature = "surrealdb")]
    async fn check_seeder_surrealdb(
        &self,
        _seeder_name: &str,
    ) -> Result<bool, Box<dyn Error>> {
        // For SurrealDB, we'll use the grow_surrealdb crate's functionality
        // This is a simplified implementation - the actual implementation should use grow_surrealdb
        Ok(false) // Placeholder - implement actual SurrealDB check
    }

    #[cfg(feature = "surrealdb")]
    async fn mark_seeder_surrealdb(
        &self,
        _seeder_name: &str,
        timestamp: i64,
    ) -> Result<(), Box<dyn Error>> {
        // For SurrealDB, we'll use the grow_surrealdb crate's functionality
        // This is a simplified implementation - the actual implementation should use grow_surrealdb
        println!(
            "INSERT INTO seeds {{ timestamp: {}, name: '{}' }}",
            timestamp, _seeder_name
        );
        Ok(())
    }
}
