use grow_core::SqlValue;

#[test]
fn test_sql_value_creation() {
    assert_eq!(SqlValue::text("hello"), SqlValue::Text("hello".to_string()));
    assert_eq!(SqlValue::integer(42), SqlValue::Integer(42));
    assert_eq!(SqlValue::float(3.14), SqlValue::Float(3.14));
    assert_eq!(SqlValue::boolean(true), SqlValue::Boolean(true));
    assert_eq!(SqlValue::null(), SqlValue::Null);
}

#[test]
fn test_from_implementations() {
    let val: SqlValue = 42i64.into();
    assert_eq!(val, SqlValue::Integer(42));

    let val: SqlValue = "test".into();
    assert_eq!(val, SqlValue::Text("test".to_string()));

    let val: SqlValue = true.into();
    assert_eq!(val, SqlValue::Boolean(true));
}

#[test]
fn test_is_null() {
    assert!(SqlValue::Null.is_null());
    assert!(!SqlValue::Integer(1).is_null());
}

#[test]
fn test_display() {
    assert_eq!(SqlValue::Integer(42).to_string(), "42");
    assert_eq!(SqlValue::Text("hello".to_string()).to_string(), "hello");
    assert_eq!(SqlValue::Null.to_string(), "NULL");
}

#[test]
fn test_type_name() {
    assert_eq!(SqlValue::Integer(42).type_name(), "INTEGER");
    assert_eq!(SqlValue::Float(3.14).type_name(), "REAL");
    assert_eq!(SqlValue::Text("hello".to_string()).type_name(), "TEXT");
    assert_eq!(SqlValue::Boolean(true).type_name(), "BOOLEAN");
    assert_eq!(SqlValue::Null.type_name(), "NULL");
}

#[test]
fn test_option_conversion() {
    let some_int: Option<i64> = Some(42);
    let none_int: Option<i64> = None;

    assert_eq!(SqlValue::from(some_int), SqlValue::Integer(42));
    assert_eq!(SqlValue::from(none_int), SqlValue::Null);

    let some_str: Option<&str> = Some("test");
    let none_str: Option<&str> = None;

    assert_eq!(SqlValue::from(some_str), SqlValue::Text("test".to_string()));
    assert_eq!(SqlValue::from(none_str), SqlValue::Null);
}
