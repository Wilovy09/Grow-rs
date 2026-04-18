use grow_rs::commands::run::{entry::Entry, template};
use grow_rs::SqlValue;
use std::collections::BTreeMap;

const MOCK_DB: &str = "mock://";

#[test]
fn test_template_start() {
    let templating = template::start();

    assert!(templating.render("simple text").is_ok());
    assert!(templating.render("text with {placeholder}").is_err());
}

#[tokio::test]
async fn test_render_tables_static_entry() {
    let mut fields = BTreeMap::new();
    fields.insert("id".to_string(), SqlValue::Integer(1));
    fields.insert("name".to_string(), SqlValue::Text("Alice".to_string()));
    fields.insert("active".to_string(), SqlValue::Boolean(true));

    let mut values = Vec::new();
    values.push(fields.clone());

    let mut fields2 = BTreeMap::new();
    fields2.insert("id".to_string(), SqlValue::Integer(2));
    fields2.insert("name".to_string(), SqlValue::Text("Bob".to_string()));
    fields2.insert("active".to_string(), SqlValue::Boolean(false));
    values.push(fields2);

    let entry = Entry::Static {
        table_name: "users".to_string(),
        values,
    };

    let entries = vec![entry];
    let result = template::render_tables(entries, MOCK_DB).await;

    assert!(result.is_ok());
    let tables = result.unwrap();

    assert_eq!(tables.len(), 1);
    assert!(tables.contains_key("users"));

    let users_table = tables.get("users").unwrap();
    assert_eq!(users_table.len(), 2);

    assert_eq!(users_table[0].len(), 3);
    assert_eq!(users_table[0][0].0, "active");
    assert_eq!(users_table[0][0].1, SqlValue::Boolean(true));
    assert_eq!(users_table[0][1].0, "id");
    assert_eq!(users_table[0][1].1, SqlValue::Integer(1));
    assert_eq!(users_table[0][2].0, "name");
    assert_eq!(users_table[0][2].1, SqlValue::Text("Alice".to_string()));

    assert_eq!(users_table[1][0].1, SqlValue::Boolean(false));
    assert_eq!(users_table[1][1].1, SqlValue::Integer(2));
    assert_eq!(users_table[1][2].1, SqlValue::Text("Bob".to_string()));
}

#[tokio::test]
async fn test_render_tables_repeat_entry() {
    let mut fields = BTreeMap::new();
    fields.insert("id".to_string(), SqlValue::Integer(100));
    fields.insert("name".to_string(), SqlValue::Text("Product".to_string()));
    fields.insert("price".to_string(), SqlValue::Float(29.99));

    let entry = Entry::Repeat {
        count: 3,
        table_name: "products".to_string(),
        fields,
    };

    let entries = vec![entry];
    let result = template::render_tables(entries, MOCK_DB).await;

    assert!(result.is_ok());
    let tables = result.unwrap();

    assert_eq!(tables.len(), 1);
    assert!(tables.contains_key("products"));

    let products_table = tables.get("products").unwrap();
    assert_eq!(products_table.len(), 3);

    for row in products_table {
        assert_eq!(row.len(), 3);
        assert_eq!(row[0].0, "id");
        assert_eq!(row[0].1, SqlValue::Integer(100));
        assert_eq!(row[1].0, "name");
        assert_eq!(row[1].1, SqlValue::Text("Product".to_string()));
        assert_eq!(row[2].0, "price");
        assert_eq!(row[2].1, SqlValue::Float(29.99));
    }
}

#[tokio::test]
async fn test_render_tables_multiple_tables() {
    let mut users_fields = BTreeMap::new();
    users_fields.insert("id".to_string(), SqlValue::Integer(1));
    users_fields
        .insert("username".to_string(), SqlValue::Text("admin".to_string()));

    let users_entry = Entry::Static {
        table_name: "users".to_string(),
        values: vec![users_fields],
    };

    let mut products_fields = BTreeMap::new();
    products_fields.insert("product_id".to_string(), SqlValue::Integer(1000));
    products_fields.insert("in_stock".to_string(), SqlValue::Boolean(true));

    let products_entry = Entry::Repeat {
        count: 2,
        table_name: "products".to_string(),
        fields: products_fields,
    };

    let entries = vec![users_entry, products_entry];
    let result = template::render_tables(entries, MOCK_DB).await;

    assert!(result.is_ok());
    let tables = result.unwrap();

    assert_eq!(tables.len(), 2);
    assert!(tables.contains_key("users"));
    assert!(tables.contains_key("products"));

    assert_eq!(tables.get("users").unwrap().len(), 1);
    assert_eq!(tables.get("products").unwrap().len(), 2);
}

#[tokio::test]
async fn test_render_tables_empty_entries() {
    let result = template::render_tables(vec![], MOCK_DB).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[tokio::test]
async fn test_render_tables_mixed_sql_value_types() {
    let mut fields = BTreeMap::new();
    fields.insert("int_col".to_string(), SqlValue::Integer(42));
    fields.insert("float_col".to_string(), SqlValue::Float(3.14159));
    fields.insert(
        "text_col".to_string(),
        SqlValue::Text("Hello World".to_string()),
    );
    fields.insert("bool_col".to_string(), SqlValue::Boolean(true));
    fields.insert("null_col".to_string(), SqlValue::Null);

    let entry = Entry::Static {
        table_name: "mixed_types".to_string(),
        values: vec![fields],
    };

    let result = template::render_tables(vec![entry], MOCK_DB).await;
    assert!(result.is_ok());
    let tables = result.unwrap();

    let mixed_table = tables.get("mixed_types").unwrap();
    assert_eq!(mixed_table.len(), 1);
    assert_eq!(mixed_table[0].len(), 5);

    let row = &mixed_table[0];
    assert!(row
        .iter()
        .any(|(k, v)| k == "int_col" && matches!(v, SqlValue::Integer(42))));
    assert!(row.iter().any(|(k, v)| k == "float_col"
        && matches!(v, SqlValue::Float(f) if (*f - 3.14159).abs() < f64::EPSILON)));
    assert!(row.iter().any(|(k, v)| k == "text_col"
        && matches!(v, SqlValue::Text(s) if s == "Hello World")));
    assert!(row
        .iter()
        .any(|(k, v)| k == "bool_col" && matches!(v, SqlValue::Boolean(true))));
    assert!(row
        .iter()
        .any(|(k, v)| k == "null_col" && matches!(v, SqlValue::Null)));
}

#[test]
fn test_rendered_table_type_alias() {
    // Test that RenderedTable type alias works correctly
    let mut table: template::RenderedTable = Vec::new();

    let row = vec![
        ("id".to_string(), SqlValue::Integer(1)),
        ("name".to_string(), SqlValue::Text("Test".to_string())),
    ];

    table.push(row);

    assert_eq!(table.len(), 1);
    assert_eq!(table[0].len(), 2);
    assert_eq!(table[0][0].0, "id");
    assert_eq!(table[0][0].1, SqlValue::Integer(1));
}
