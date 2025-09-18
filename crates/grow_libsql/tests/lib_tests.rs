use grow_libsql::{escape_column_name, escape_table_name, SqlValue};

#[test]
fn test_escape_table_name() {
    // Simple table name
    assert_eq!(escape_table_name("users"), "\"users\"");

    // Table name with schema
    assert_eq!(escape_table_name("public.users"), "\"public\".\"users\"");

    // Table name with multiple parts
    assert_eq!(
        escape_table_name("database.schema.table"),
        "\"database\".\"schema\".\"table\""
    );

    // Table name with special characters
    assert_eq!(escape_table_name("user-table"), "\"user-table\"");

    // Empty table name
    assert_eq!(escape_table_name(""), "\"\"");
}

#[test]
fn test_escape_column_name() {
    // Simple column name
    assert_eq!(escape_column_name("id"), "\"id\"");

    // Column with special characters
    assert_eq!(escape_column_name("user_name"), "\"user_name\"");

    // Column with spaces
    assert_eq!(escape_column_name("full name"), "\"full name\"");

    // Column with special SQL keywords
    assert_eq!(escape_column_name("select"), "\"select\"");
    assert_eq!(escape_column_name("where"), "\"where\"");

    // Empty column name
    assert_eq!(escape_column_name(""), "\"\"");
}

#[test]
fn test_sql_value_to_libsql_conversion() {
    // Test that SqlValue variants can be converted to libsql::Value
    // We can't directly test the conversion function since it's private,
    // but we can test that our SqlValue types work correctly

    let integer_val = SqlValue::Integer(42);
    assert_eq!(format!("{}", integer_val), "42");

    let float_val = SqlValue::Float(3.14);
    assert_eq!(format!("{}", float_val), "3.14");

    let text_val = SqlValue::Text("hello".to_string());
    assert_eq!(format!("{}", text_val), "hello");

    let bool_val = SqlValue::Boolean(true);
    assert_eq!(format!("{}", bool_val), "true");

    let null_val = SqlValue::Null;
    assert_eq!(format!("{}", null_val), "NULL");
}

#[test]
fn test_sql_value_type_names() {
    assert_eq!(SqlValue::Integer(1).type_name(), "INTEGER");
    assert_eq!(SqlValue::Float(1.0).type_name(), "REAL");
    assert_eq!(SqlValue::Text("test".to_string()).type_name(), "TEXT");
    assert_eq!(SqlValue::Boolean(true).type_name(), "BOOLEAN");
    assert_eq!(SqlValue::Null.type_name(), "NULL");
}

#[test]
fn test_sql_value_null_check() {
    assert!(SqlValue::Null.is_null());
    assert!(!SqlValue::Integer(0).is_null());
    assert!(!SqlValue::Float(0.0).is_null());
    assert!(!SqlValue::Text("".to_string()).is_null());
    assert!(!SqlValue::Boolean(false).is_null());
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_table_structure_creation() {
        // Test creating a simple table structure
        let mut tables: BTreeMap<String, Vec<Vec<(String, SqlValue)>>> =
            BTreeMap::new();

        let mut rows = Vec::new();
        let row = vec![
            ("id".to_string(), SqlValue::Integer(1)),
            ("name".to_string(), SqlValue::Text("John Doe".to_string())),
            ("age".to_string(), SqlValue::Integer(30)),
            ("active".to_string(), SqlValue::Boolean(true)),
        ];
        rows.push(row);

        tables.insert("users".to_string(), rows);

        assert_eq!(tables.len(), 1);
        assert!(tables.contains_key("users"));

        let users_data = tables.get("users").unwrap();
        assert_eq!(users_data.len(), 1);
        assert_eq!(users_data[0].len(), 4);

        // Verify the data structure
        let first_row = &users_data[0];
        assert_eq!(first_row[0].0, "id");
        assert_eq!(first_row[0].1, SqlValue::Integer(1));
        assert_eq!(first_row[1].0, "name");
        assert_eq!(first_row[1].1, SqlValue::Text("John Doe".to_string()));
    }

    #[test]
    fn test_multiple_tables_structure() {
        let mut tables: BTreeMap<String, Vec<Vec<(String, SqlValue)>>> =
            BTreeMap::new();

        // Users table
        tables.insert(
            "users".to_string(),
            vec![vec![
                ("id".to_string(), SqlValue::Integer(1)),
                ("name".to_string(), SqlValue::Text("Alice".to_string())),
            ]],
        );

        // Products table
        tables.insert(
            "products".to_string(),
            vec![vec![
                ("id".to_string(), SqlValue::Integer(100)),
                ("name".to_string(), SqlValue::Text("Widget".to_string())),
                ("price".to_string(), SqlValue::Float(29.99)),
            ]],
        );

        assert_eq!(tables.len(), 2);
        assert!(tables.contains_key("users"));
        assert!(tables.contains_key("products"));
    }
}
