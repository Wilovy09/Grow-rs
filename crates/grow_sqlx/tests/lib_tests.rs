use grow_sqlx::{
    escape_column_name, escape_table_name, sql_value_from_external,
    ExternalSqlValue, RenderedTable, SqlValue,
};
use std::collections::BTreeMap;

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
fn test_sql_value_from_external_integer() {
    let external = ExternalSqlValue {
        integer: Some(42),
        float: None,
        text: None,
        boolean: None,
        null: false,
    };

    let result = sql_value_from_external(external);
    assert_eq!(result, SqlValue::Integer(42));
}

#[test]
fn test_sql_value_from_external_float() {
    let external = ExternalSqlValue {
        integer: None,
        float: Some(3.14),
        text: None,
        boolean: None,
        null: false,
    };

    let result = sql_value_from_external(external);
    assert_eq!(result, SqlValue::Float(3.14));
}

#[test]
fn test_sql_value_from_external_text() {
    let external = ExternalSqlValue {
        integer: None,
        float: None,
        text: Some("hello world".to_string()),
        boolean: None,
        null: false,
    };

    let result = sql_value_from_external(external);
    assert_eq!(result, SqlValue::Text("hello world".to_string()));
}

#[test]
fn test_sql_value_from_external_boolean() {
    let external = ExternalSqlValue {
        integer: None,
        float: None,
        text: None,
        boolean: Some(true),
        null: false,
    };

    let result = sql_value_from_external(external);
    assert_eq!(result, SqlValue::Boolean(true));
}

#[test]
fn test_sql_value_from_external_null() {
    let external = ExternalSqlValue {
        integer: None,
        float: None,
        text: None,
        boolean: None,
        null: true,
    };

    let result = sql_value_from_external(external);
    assert_eq!(result, SqlValue::Null);
}

#[test]
fn test_sql_value_from_external_null_takes_precedence() {
    // Even if other fields are set, null=true should take precedence
    let external = ExternalSqlValue {
        integer: Some(42),
        float: Some(3.14),
        text: Some("test".to_string()),
        boolean: Some(true),
        null: true,
    };

    let result = sql_value_from_external(external);
    assert_eq!(result, SqlValue::Null);
}

#[test]
fn test_sql_value_from_external_empty_defaults_to_null() {
    // If no fields are set and null is false, should default to Null
    let external = ExternalSqlValue {
        integer: None,
        float: None,
        text: None,
        boolean: None,
        null: false,
    };

    let result = sql_value_from_external(external);
    assert_eq!(result, SqlValue::Null);
}

#[test]
fn test_external_sql_value_creation() {
    // Test creating ExternalSqlValue instances
    let integer_external = ExternalSqlValue {
        integer: Some(100),
        float: None,
        text: None,
        boolean: None,
        null: false,
    };

    assert_eq!(integer_external.integer, Some(100));
    assert!(!integer_external.null);

    let null_external = ExternalSqlValue {
        integer: None,
        float: None,
        text: None,
        boolean: None,
        null: true,
    };

    assert!(null_external.null);
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_rendered_table_type() {
        // Test that RenderedTable type works correctly
        let mut table: RenderedTable = Vec::new();

        let row = vec![
            ("id".to_string(), SqlValue::Integer(1)),
            ("name".to_string(), SqlValue::Text("Alice".to_string())),
            ("score".to_string(), SqlValue::Float(95.5)),
            ("active".to_string(), SqlValue::Boolean(true)),
            ("note".to_string(), SqlValue::Null),
        ];

        table.push(row);

        assert_eq!(table.len(), 1);
        assert_eq!(table[0].len(), 5);

        // Verify each field
        assert_eq!(table[0][0].0, "id");
        assert_eq!(table[0][0].1, SqlValue::Integer(1));
        assert_eq!(table[0][1].0, "name");
        assert_eq!(table[0][1].1, SqlValue::Text("Alice".to_string()));
        assert_eq!(table[0][4].1, SqlValue::Null);
    }

    #[test]
    fn test_multiple_tables_with_rendered_table() {
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

        // Products table
        let products_table = vec![vec![
            ("id".to_string(), SqlValue::Integer(100)),
            ("name".to_string(), SqlValue::Text("Widget".to_string())),
            ("price".to_string(), SqlValue::Float(29.99)),
            ("in_stock".to_string(), SqlValue::Boolean(true)),
        ]];

        tables.insert("users".to_string(), users_table);
        tables.insert("products".to_string(), products_table);

        assert_eq!(tables.len(), 2);
        assert!(tables.contains_key("users"));
        assert!(tables.contains_key("products"));

        // Verify users table structure
        let users = tables.get("users").unwrap();
        assert_eq!(users.len(), 2); // 2 rows
        assert_eq!(users[0].len(), 2); // 2 columns per row

        // Verify products table structure
        let products = tables.get("products").unwrap();
        assert_eq!(products.len(), 1); // 1 row
        assert_eq!(products[0].len(), 4); // 4 columns per row
    }

    #[test]
    fn test_external_sql_value_conversion_workflow() {
        // Test a complete workflow of converting external values
        let external_values = vec![
            ExternalSqlValue {
                integer: Some(1),
                float: None,
                text: None,
                boolean: None,
                null: false,
            },
            ExternalSqlValue {
                integer: None,
                float: None,
                text: Some("test".to_string()),
                boolean: None,
                null: false,
            },
            ExternalSqlValue {
                integer: None,
                float: None,
                text: None,
                boolean: None,
                null: true,
            },
        ];

        let converted: Vec<SqlValue> = external_values
            .into_iter()
            .map(sql_value_from_external)
            .collect();

        assert_eq!(converted.len(), 3);
        assert_eq!(converted[0], SqlValue::Integer(1));
        assert_eq!(converted[1], SqlValue::Text("test".to_string()));
        assert_eq!(converted[2], SqlValue::Null);
    }
}
