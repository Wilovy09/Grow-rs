use grow_rs::SqlValue;

#[test]
fn test_sql_value_re_export() {
    // Test that SqlValue is properly re-exported from grow_core
    let integer_val = SqlValue::Integer(42);
    let text_val = SqlValue::Text("test".to_string());
    let boolean_val = SqlValue::Boolean(true);
    let float_val = SqlValue::Float(3.14);
    let null_val = SqlValue::Null;
    
    assert_eq!(integer_val, SqlValue::Integer(42));
    assert_eq!(text_val, SqlValue::Text("test".to_string()));
    assert_eq!(boolean_val, SqlValue::Boolean(true));
    assert_eq!(float_val, SqlValue::Float(3.14));
    assert_eq!(null_val, SqlValue::Null);
}

#[test]
fn test_sql_value_from_conversions() {
    // Test From trait implementations work through re-export
    let from_i64: SqlValue = 100i64.into();
    assert_eq!(from_i64, SqlValue::Integer(100));
    
    let from_f64: SqlValue = 2.718.into();
    assert_eq!(from_f64, SqlValue::Float(2.718));
    
    let from_string: SqlValue = "hello".into();
    assert_eq!(from_string, SqlValue::Text("hello".to_string()));
    
    let from_bool: SqlValue = false.into();
    assert_eq!(from_bool, SqlValue::Boolean(false));
}

#[test]
fn test_sql_value_display() {
    // Test Display trait works through re-export
    assert_eq!(SqlValue::Integer(123).to_string(), "123");
    assert_eq!(SqlValue::Float(4.56).to_string(), "4.56");
    assert_eq!(SqlValue::Text("world".to_string()).to_string(), "world");
    assert_eq!(SqlValue::Boolean(true).to_string(), "true");
    assert_eq!(SqlValue::Null.to_string(), "NULL");
}

#[test]
fn test_sql_value_helper_methods() {
    // Test helper methods work through re-export
    assert_eq!(SqlValue::integer(789), SqlValue::Integer(789));
    assert_eq!(SqlValue::float(1.23), SqlValue::Float(1.23));
    assert_eq!(SqlValue::text("test"), SqlValue::Text("test".to_string()));
    assert_eq!(SqlValue::boolean(false), SqlValue::Boolean(false));
    assert_eq!(SqlValue::null(), SqlValue::Null);
}

#[test]
fn test_sql_value_type_checks() {
    // Test type checking methods work through re-export
    assert!(SqlValue::Null.is_null());
    assert!(!SqlValue::Integer(1).is_null());
    
    assert_eq!(SqlValue::Integer(1).type_name(), "INTEGER");
    assert_eq!(SqlValue::Float(1.0).type_name(), "REAL");
    assert_eq!(SqlValue::Text("test".to_string()).type_name(), "TEXT");
    assert_eq!(SqlValue::Boolean(true).type_name(), "BOOLEAN");
    assert_eq!(SqlValue::Null.type_name(), "NULL");
}

#[cfg(test)]
mod module_visibility_tests {
    use grow_rs::{utils, commands};
    
    #[test]
    fn test_utils_module_accessible() {
        // Test that utils module is properly exported
        let path = "/test/path";
        let error_mapper = utils::map_io_error(path);
        
        let not_found_error = std::io::Error::new(std::io::ErrorKind::NotFound, "Test error");
        let result = error_mapper(not_found_error);
        
        assert!(result.contains("not found"));
    }
    
    #[test]
    fn test_commands_modules_accessible() {
        // Test that command modules are accessible
        let templating = commands::run::template::start();
        assert!(templating.render("simple text").is_ok());
    }
    
    #[test]
    fn test_rendered_table_type_accessible() {
        // Test that RenderedTable type is accessible
        let mut table: commands::run::template::RenderedTable = Vec::new();
        
        let row = vec![
            ("id".to_string(), grow_rs::SqlValue::Integer(1)),
            ("name".to_string(), grow_rs::SqlValue::Text("Test".to_string())),
        ];
        
        table.push(row);
        
        assert_eq!(table.len(), 1);
        assert_eq!(table[0].len(), 2);
    }
}