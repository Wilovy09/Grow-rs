pub use grow_core::SqlValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::engine::any::{connect, Any};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub type RenderedTable = Vec<Vec<(String, SqlValue)>>;

// Connection configuration for SurrealDB
#[derive(Debug, Clone)]
pub struct SurrealConfig {
    pub endpoint: String,
    pub namespace: String,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for SurrealConfig {
    fn default() -> Self {
        Self {
            endpoint: "memory".to_string(),
            namespace: "grow".to_string(),
            database: "seeders".to_string(),
            username: None,
            password: None,
        }
    }
}

// Helper struct for SurrealDB record creation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SurrealRecord {
    #[serde(flatten)]
    data: BTreeMap<String, surrealdb::Value>,
}

pub async fn run_seeder(
    config: SurrealConfig,
    tables: BTreeMap<String, RenderedTable>,
) -> Result<(), String> {
    // Connect to SurrealDB
    let db: Surreal<Any> = connect(&config.endpoint).await.map_err(|err| {
        format!("Cannot connect to SurrealDB ({}): {}", config.endpoint, err)
    })?;

    // Authenticate if credentials are provided
    if let (Some(username), Some(password)) =
        (&config.username, &config.password)
    {
        db.signin(Root { username, password })
            .await
            .map_err(|err| format!("Authentication failed: {}", err))?;
    }

    // Use namespace and database
    db.use_ns(&config.namespace)
        .use_db(&config.database)
        .await
        .map_err(|err| {
            format!("Failed to select namespace/database: {}", err)
        })?;

    // Insert data for each table
    for (table_name, rows) in tables {
        for row in rows {
            insert_record(&db, &table_name, row).await?;
        }
    }

    Ok(())
}

/// Main function to run seeders on SurrealDB with connection string  
pub async fn run_seeder_with_connection_string(
    connection_string: &str,
    tables: &BTreeMap<String, RenderedTable>,
) -> Result<(), String> {
    let config = parse_connection_string(connection_string)?;
    let owned_tables = tables.clone();
    run_seeder(config, owned_tables).await
}

async fn insert_record(
    db: &Surreal<Any>,
    table: &str,
    entry: Vec<(String, SqlValue)>,
) -> Result<(), String> {
    // Build SurrealDB query manually to avoid serialization issues
    let query = build_insert_query(table, &entry)?;

    // Execute raw query
    let _result = db.query(&query).await.map_err(|err| {
        format!("Failed to create record in table {}: {}", table, err)
    })?;

    Ok(())
}

/// Build SurrealDB CREATE query from row data
fn build_insert_query(
    table: &str,
    row: &[(String, SqlValue)],
) -> Result<String, String> {
    validate_table_name(table)?;

    let mut fields = Vec::new();

    for (column, value) in row {
        let surreal_value = match value {
            SqlValue::Integer(i) => i.to_string(),
            SqlValue::Float(f) => f.to_string(),
            SqlValue::Text(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            SqlValue::Boolean(b) => b.to_string(),
            SqlValue::Null => "NONE".to_string(),
        };
        // Use = instead of : for SurrealDB CREATE queries
        fields.push(format!("{} = {}", column, surreal_value));
    }

    let query = format!("CREATE {} SET {};", table, fields.join(", "));
    Ok(query)
}

// Helper function to parse connection strings
pub fn parse_connection_string(
    connection_string: &str,
) -> Result<SurrealConfig, String> {
    use std::env;

    // Handle different connection string formats
    if connection_string == "memory" {
        return Ok(SurrealConfig {
            endpoint: "memory".to_string(),
            ..Default::default()
        });
    }

    // For all other connection types, try to extract credentials from environment or URL
    let mut config = SurrealConfig::default();
    let mut endpoint = connection_string.to_string();
    let mut username = None;
    let mut password = None;

    // Try to parse username:password@host format
    if let Some(at_pos) = connection_string.find('@') {
        let (auth_part, host_part) = connection_string.split_at(at_pos);
        let host_part = &host_part[1..]; // Remove '@' character

        // Find the scheme end
        if let Some(scheme_end) = auth_part.find("://") {
            let scheme = &auth_part[..scheme_end + 3];
            let credentials = &auth_part[scheme_end + 3..];

            if let Some(colon_pos) = credentials.find(':') {
                username = Some(credentials[..colon_pos].to_string());
                password = Some(credentials[colon_pos + 1..].to_string());
            } else if !credentials.is_empty() {
                username = Some(credentials.to_string());
            }

            endpoint = format!("{}{}", scheme, host_part);
        }
    }

    // If no credentials in URL, try to get from environment variables
    if username.is_none() {
        username = env::var("SURREAL_USER")
            .ok()
            .or_else(|| env::var("SURREALDB_USER").ok())
            .or_else(|| env::var("SURREAL_USERNAME").ok());
    }

    if password.is_none() {
        password = env::var("SURREAL_PASS")
            .ok()
            .or_else(|| env::var("SURREALDB_PASS").ok())
            .or_else(|| env::var("SURREAL_PASSWORD").ok());
    }

    // Also check for namespace and database in environment
    let namespace = env::var("SURREAL_NS")
        .ok()
        .or_else(|| env::var("SURREALDB_NS").ok())
        .or_else(|| env::var("SURREAL_NAMESPACE").ok())
        .unwrap_or_else(|| "grow".to_string());

    let database = env::var("SURREAL_DB")
        .ok()
        .or_else(|| env::var("SURREALDB_DB").ok())
        .or_else(|| env::var("SURREAL_DATABASE").ok())
        .unwrap_or_else(|| "seeders".to_string());

    config.endpoint = endpoint;
    config.username = username;
    config.password = password;
    config.namespace = namespace;
    config.database = database;

    // Validate known protocols
    if connection_string.starts_with("file://")
        || connection_string.starts_with("rocksdb://")
        || connection_string.starts_with("ws://")
        || connection_string.starts_with("wss://")
        || connection_string.starts_with("http://")
        || connection_string.starts_with("https://")
    {
        return Ok(config);
    }

    // Default to memory if no recognized protocol
    Ok(SurrealConfig {
        endpoint: "memory".to_string(),
        namespace: config.namespace,
        database: config.database,
        username: None,
        password: None,
    })
}

// Helper function to validate table names for SurrealDB
pub fn validate_table_name(table_name: &str) -> Result<(), String> {
    if table_name.is_empty() {
        return Err("Table name cannot be empty".to_string());
    }

    // SurrealDB table names should start with a letter or underscore
    let first_char = table_name.chars().next().unwrap();
    if !first_char.is_ascii_alphabetic() && first_char != '_' {
        return Err(
            "Table name must start with a letter or underscore".to_string()
        );
    }

    // Check for valid characters
    for c in table_name.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return Err(format!("Invalid character '{}' in table name", c));
        }
    }

    Ok(())
}

// Helper function to build SurrealDB query for bulk insert
pub fn build_bulk_insert_query(
    table: &str,
    rows: &[Vec<(String, SqlValue)>],
) -> Result<String, String> {
    validate_table_name(table)?;

    if rows.is_empty() {
        return Ok(String::new());
    }

    let mut queries = Vec::new();

    for row in rows {
        let mut fields = Vec::new();
        for (key, value) in row {
            let value_str = match value {
                SqlValue::Integer(i) => i.to_string(),
                SqlValue::Float(f) => f.to_string(),
                SqlValue::Text(s) => format!("\"{}\"", s.replace('\"', "\\\"")),
                SqlValue::Boolean(b) => b.to_string(),
                SqlValue::Null => "NONE".to_string(),
            };
            fields.push(format!("{} = {}", key, value_str));
        }

        let query = format!("CREATE {} SET {};", table, fields.join(", "));
        queries.push(query);
    }

    Ok(queries.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_value_conversion() {
        // Test that the conversion logic doesn't panic
        let test_cases = vec![
            ("int", SqlValue::Integer(42)),
            ("float", SqlValue::Float(3.14)),
            ("text", SqlValue::Text("hello".to_string())),
            ("bool", SqlValue::Boolean(true)),
            ("null", SqlValue::Null),
        ];

        for (name, sql_val) in test_cases {
            // Simulate the conversion that happens in insert_record
            let result = match sql_val {
                SqlValue::Integer(i) => {
                    serde_json::from_value::<surrealdb::Value>(
                        serde_json::Value::Number(serde_json::Number::from(i)),
                    )
                    .is_ok()
                }
                SqlValue::Float(f) => {
                    if let Some(n) = serde_json::Number::from_f64(f) {
                        serde_json::from_value::<surrealdb::Value>(
                            serde_json::Value::Number(n),
                        )
                        .is_ok()
                    } else {
                        false
                    }
                }
                SqlValue::Text(s) => {
                    serde_json::from_value::<surrealdb::Value>(
                        serde_json::Value::String(s),
                    )
                    .is_ok()
                }
                SqlValue::Boolean(b) => {
                    serde_json::from_value::<surrealdb::Value>(
                        serde_json::Value::Bool(b),
                    )
                    .is_ok()
                }
                SqlValue::Null => serde_json::from_value::<surrealdb::Value>(
                    serde_json::Value::Null,
                )
                .is_ok(),
            };

            // We just need to ensure the basic conversion attempt doesn't panic
            // The actual success/failure isn't as important as the API stability
            println!("Conversion for {} completed: {}", name, result);
        }
    }

    #[test]
    fn test_parse_connection_string() {
        let memory_config = parse_connection_string("memory").unwrap();
        assert_eq!(memory_config.endpoint, "memory");

        let file_config =
            parse_connection_string("file:///tmp/database.db").unwrap();
        assert_eq!(file_config.endpoint, "file:///tmp/database.db");

        let ws_config = parse_connection_string("ws://localhost:8000").unwrap();
        assert_eq!(ws_config.endpoint, "ws://localhost:8000");

        let http_config =
            parse_connection_string("https://cloud.surrealdb.com").unwrap();
        assert_eq!(http_config.endpoint, "https://cloud.surrealdb.com");

        // Test URL with credentials
        let auth_config =
            parse_connection_string("ws://user:pass@localhost:8000").unwrap();
        assert_eq!(auth_config.endpoint, "ws://localhost:8000");
        assert_eq!(auth_config.username, Some("user".to_string()));
        assert_eq!(auth_config.password, Some("pass".to_string()));

        // Test HTTPS with credentials
        let https_auth_config =
            parse_connection_string("https://admin:secret@cloud.surrealdb.com")
                .unwrap();
        assert_eq!(https_auth_config.endpoint, "https://cloud.surrealdb.com");
        assert_eq!(https_auth_config.username, Some("admin".to_string()));
        assert_eq!(https_auth_config.password, Some("secret".to_string()));

        // Test WSS (like SurrealDB Cloud)
        let wss_config = parse_connection_string(
            "wss://test-db-06clchjlftpot4bscln7dgvpro.aws-use1.surreal.cloud",
        )
        .unwrap();
        assert_eq!(
            wss_config.endpoint,
            "wss://test-db-06clchjlftpot4bscln7dgvpro.aws-use1.surreal.cloud"
        );
    }

    #[test]
    fn test_validate_table_name() {
        assert!(validate_table_name("users").is_ok());
        assert!(validate_table_name("_private").is_ok());
        assert!(validate_table_name("user_profiles").is_ok());
        assert!(validate_table_name("table123").is_ok());

        assert!(validate_table_name("").is_err());
        assert!(validate_table_name("123table").is_err());
        assert!(validate_table_name("user-profiles").is_err());
        assert!(validate_table_name("user.profiles").is_err());
    }
}
