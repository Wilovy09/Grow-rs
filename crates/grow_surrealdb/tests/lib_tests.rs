use grow_surrealdb::{
    build_bulk_insert_query, parse_connection_string, validate_table_name,
    RenderedTable, SqlValue, SurrealConfig,
};
use std::collections::BTreeMap;

#[test]
fn test_parse_connection_string_memory() {
    let config = parse_connection_string("memory").unwrap();
    assert_eq!(config.endpoint, "memory");
    assert_eq!(config.namespace, "grow");
    assert_eq!(config.database, "seeders");
}

#[test]
fn test_parse_connection_string_file() {
    let config = parse_connection_string("file:///tmp/test.db").unwrap();
    assert_eq!(config.endpoint, "file:///tmp/test.db");
    assert_eq!(config.namespace, "grow");
    assert_eq!(config.database, "seeders");
}

#[test]
fn test_parse_connection_string_rocksdb() {
    let config = parse_connection_string("rocksdb://./database").unwrap();
    assert_eq!(config.endpoint, "rocksdb://./database");
}

#[test]
fn test_parse_connection_string_websocket() {
    let ws_config = parse_connection_string("ws://localhost:8000").unwrap();
    assert_eq!(ws_config.endpoint, "ws://localhost:8000");

    let wss_config =
        parse_connection_string("wss://cloud.surrealdb.com").unwrap();
    assert_eq!(wss_config.endpoint, "wss://cloud.surrealdb.com");
}

#[test]
fn test_parse_connection_string_http() {
    let http_config = parse_connection_string("http://localhost:8000").unwrap();
    assert_eq!(http_config.endpoint, "http://localhost:8000");

    let https_config =
        parse_connection_string("https://cloud.surrealdb.com").unwrap();
    assert_eq!(https_config.endpoint, "https://cloud.surrealdb.com");
}

#[test]
fn test_parse_connection_string_unknown_defaults_to_memory() {
    let config = parse_connection_string("unknown://something").unwrap();
    assert_eq!(config.endpoint, "memory");
}

#[test]
fn test_validate_table_name_valid() {
    assert!(validate_table_name("users").is_ok());
    assert!(validate_table_name("_private_table").is_ok());
    assert!(validate_table_name("user_profiles").is_ok());
    assert!(validate_table_name("table123").is_ok());
    assert!(validate_table_name("Table").is_ok());
    assert!(validate_table_name("TABLE").is_ok());
    assert!(validate_table_name("a").is_ok());
    assert!(validate_table_name("_").is_ok());
}

#[test]
fn test_validate_table_name_invalid() {
    assert!(validate_table_name("").is_err());
    assert!(validate_table_name("123table").is_err());
    assert!(validate_table_name("user-profiles").is_err());
    assert!(validate_table_name("user.profiles").is_err());
    assert!(validate_table_name("user profiles").is_err());
    assert!(validate_table_name("user@profiles").is_err());
    assert!(validate_table_name("user#profiles").is_err());
    assert!(validate_table_name("user$profiles").is_err());
}

#[test]
fn test_build_bulk_insert_query_empty() {
    let rows: Vec<Vec<(String, SqlValue)>> = vec![];
    let query = build_bulk_insert_query("users", &rows).unwrap();
    assert_eq!(query, "");
}

#[test]
fn test_build_bulk_insert_query_single_row() {
    let rows = vec![vec![
        ("id".to_string(), SqlValue::Integer(1)),
        ("name".to_string(), SqlValue::Text("Alice".to_string())),
        ("active".to_string(), SqlValue::Boolean(true)),
    ]];

    let query = build_bulk_insert_query("users", &rows).unwrap();
    let expected = "CREATE users SET id = 1, name = \"Alice\", active = true;";
    assert_eq!(query, expected);
}

#[test]
fn test_build_bulk_insert_query_multiple_rows() {
    let rows = vec![
        vec![
            ("id".to_string(), SqlValue::Integer(1)),
            ("name".to_string(), SqlValue::Text("Alice".to_string())),
        ],
        vec![
            ("id".to_string(), SqlValue::Integer(2)),
            ("name".to_string(), SqlValue::Text("Bob".to_string())),
        ],
    ];

    let query = build_bulk_insert_query("users", &rows).unwrap();
    let expected = "CREATE users SET id = 1, name = \"Alice\";\nCREATE users SET id = 2, name = \"Bob\";";
    assert_eq!(query, expected);
}

#[test]
fn test_build_bulk_insert_query_all_sql_value_types() {
    let rows = vec![vec![
        ("int_col".to_string(), SqlValue::Integer(42)),
        ("float_col".to_string(), SqlValue::Float(3.14)),
        (
            "text_col".to_string(),
            SqlValue::Text("hello world".to_string()),
        ),
        ("bool_col".to_string(), SqlValue::Boolean(false)),
        ("null_col".to_string(), SqlValue::Null),
    ]];

    let query = build_bulk_insert_query("mixed_table", &rows).unwrap();
    let expected = "CREATE mixed_table SET int_col = 42, float_col = 3.14, text_col = \"hello world\", bool_col = false, null_col = NONE;";
    assert_eq!(query, expected);
}

#[test]
fn test_build_bulk_insert_query_with_quotes_in_text() {
    let rows = vec![vec![(
        "message".to_string(),
        SqlValue::Text("He said \"Hello\"".to_string()),
    )]];

    let query = build_bulk_insert_query("messages", &rows).unwrap();
    let expected = "CREATE messages SET message = \"He said \\\"Hello\\\"\";";
    assert_eq!(query, expected);
}

#[test]
fn test_build_bulk_insert_query_invalid_table_name() {
    let rows = vec![vec![("id".to_string(), SqlValue::Integer(1))]];

    let result = build_bulk_insert_query("123invalid", &rows);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("must start with a letter or underscore"));
}

#[test]
fn test_surreal_config_default() {
    let config = SurrealConfig::default();
    assert_eq!(config.endpoint, "memory");
    assert_eq!(config.namespace, "grow");
    assert_eq!(config.database, "seeders");
    assert!(config.username.is_none());
    assert!(config.password.is_none());
}

#[test]
fn test_surreal_config_with_credentials() {
    let config = SurrealConfig {
        endpoint: "ws://localhost:8000".to_string(),
        namespace: "test_ns".to_string(),
        database: "test_db".to_string(),
        username: Some("admin".to_string()),
        password: Some("secret".to_string()),
    };

    assert_eq!(config.endpoint, "ws://localhost:8000");
    assert_eq!(config.namespace, "test_ns");
    assert_eq!(config.database, "test_db");
    assert_eq!(config.username.as_deref(), Some("admin"));
    assert_eq!(config.password.as_deref(), Some("secret"));
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_rendered_table_type() {
        let mut table: RenderedTable = Vec::new();

        let row = vec![
            ("id".to_string(), SqlValue::Integer(1)),
            ("name".to_string(), SqlValue::Text("Test User".to_string())),
            ("score".to_string(), SqlValue::Float(95.5)),
            ("verified".to_string(), SqlValue::Boolean(true)),
            ("notes".to_string(), SqlValue::Null),
        ];

        table.push(row);

        assert_eq!(table.len(), 1);
        assert_eq!(table[0].len(), 5);

        // Verify each field
        assert_eq!(table[0][0].0, "id");
        assert_eq!(table[0][0].1, SqlValue::Integer(1));
        assert_eq!(table[0][1].0, "name");
        assert_eq!(table[0][1].1, SqlValue::Text("Test User".to_string()));
        assert_eq!(table[0][4].1, SqlValue::Null);
    }

    #[test]
    fn test_multiple_tables_structure() {
        let mut tables: BTreeMap<String, RenderedTable> = BTreeMap::new();

        // Users table
        let users_table = vec![
            vec![
                ("id".to_string(), SqlValue::Integer(1)),
                ("username".to_string(), SqlValue::Text("alice".to_string())),
            ],
            vec![
                ("id".to_string(), SqlValue::Integer(2)),
                ("username".to_string(), SqlValue::Text("bob".to_string())),
            ],
        ];

        // Posts table
        let posts_table = vec![vec![
            ("id".to_string(), SqlValue::Integer(100)),
            (
                "title".to_string(),
                SqlValue::Text("Hello World".to_string()),
            ),
            ("user_id".to_string(), SqlValue::Integer(1)),
            ("published".to_string(), SqlValue::Boolean(true)),
        ]];

        tables.insert("users".to_string(), users_table);
        tables.insert("posts".to_string(), posts_table);

        assert_eq!(tables.len(), 2);
        assert!(tables.contains_key("users"));
        assert!(tables.contains_key("posts"));

        // Verify users table structure
        let users = tables.get("users").unwrap();
        assert_eq!(users.len(), 2); // 2 rows
        assert_eq!(users[0].len(), 2); // 2 columns per row

        // Verify posts table structure
        let posts = tables.get("posts").unwrap();
        assert_eq!(posts.len(), 1); // 1 row
        assert_eq!(posts[0].len(), 4); // 4 columns per row
    }

    #[test]
    fn test_query_building_workflow() {
        let mut tables: BTreeMap<String, RenderedTable> = BTreeMap::new();

        let users_data = vec![vec![
            ("name".to_string(), SqlValue::Text("Alice".to_string())),
            ("age".to_string(), SqlValue::Integer(30)),
        ]];

        tables.insert("users".to_string(), users_data);

        // Test that we can build queries for all tables
        for (table_name, rows) in tables.iter() {
            let query = build_bulk_insert_query(table_name, rows);
            assert!(
                query.is_ok(),
                "Failed to build query for table: {}",
                table_name
            );

            let query_string = query.unwrap();
            assert!(
                !query_string.is_empty(),
                "Query should not be empty for table: {}",
                table_name
            );
            assert!(
                query_string.contains("CREATE"),
                "Query should contain CREATE statement"
            );
            assert!(
                query_string.contains(table_name),
                "Query should contain table name"
            );
        }
    }
}
