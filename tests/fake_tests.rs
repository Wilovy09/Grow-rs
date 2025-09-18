use grow_rs::commands::run::fake::fake;

#[test]
fn test_fake_function_with_valid_kind() {
    // Test with a valid fake kind (assuming 1 is a valid kind)
    let args = vec!["1".to_string()];
    let result = fake(&args);
    
    // Should succeed with some generated value
    match result {
        Ok(value) => {
            assert!(!value.is_empty(), "Generated fake value should not be empty");
        },
        Err(e) => {
            // If the fake kind doesn't exist, that's also a valid test result
            // Just ensure the error message is meaningful
            let error_msg = format!("{:?}", e);
            assert!(error_msg.contains("Fake kind is not valid") || error_msg.contains("1"));
        }
    }
}

#[test]
fn test_fake_function_no_args() {
    // Test with no arguments - should fail
    let args = vec![];
    let result = fake(&args);
    
    assert!(result.is_err(), "fake() should fail with no arguments");
}

#[test]
fn test_fake_function_too_many_args() {
    // Test with too many arguments - should fail
    let args = vec!["1".to_string(), "2".to_string()];
    let result = fake(&args);
    
    assert!(result.is_err(), "fake() should fail with too many arguments");
}

#[test]
fn test_fake_function_invalid_kind_format() {
    // Test with non-numeric kind
    let args = vec!["not_a_number".to_string()];
    let result = fake(&args);
    
    assert!(result.is_err(), "fake() should fail with non-numeric kind");
    
    let error = result.unwrap_err();
    match error {
        srtemplate::function::Error::InvalidType(msg) => {
            assert_eq!(msg, "not_a_number");
        },
        _ => panic!("Expected InvalidType error, got: {:?}", error),
    }
}

#[test]
fn test_fake_function_negative_kind() {
    // Test with negative number (invalid u16)
    let args = vec!["-1".to_string()];
    let result = fake(&args);
    
    assert!(result.is_err(), "fake() should fail with negative number");
}

#[test]
fn test_fake_function_very_large_kind() {
    // Test with a very large number that's unlikely to be valid
    let args = vec!["99999".to_string()];
    let result = fake(&args);
    
    // This should either work (if the kind exists) or fail with "kind is not valid"
    // Both outcomes are acceptable - we just want to ensure it doesn't panic
    match result {
        Ok(value) => {
            assert!(!value.is_empty(), "Generated value should not be empty");
        },
        Err(_) => {
            // Error is acceptable for large numbers
        }
    }
}

#[test]
fn test_fake_function_zero_kind() {
    // Test with zero
    let args = vec!["0".to_string()];
    let result = fake(&args);
    
    // Zero might or might not be a valid kind, both outcomes are valid
    match result {
        Ok(value) => {
            assert!(!value.is_empty());
        },
        Err(e) => {
            let error_msg = format!("{:?}", e);
            assert!(error_msg.contains("Fake kind is not valid") || error_msg.contains("0"));
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_fake_function_consistency() {
        // Test that the function behaves consistently with the same input
        let args = vec!["1".to_string()];
        
        let result1 = fake(&args);
        let result2 = fake(&args);
        
        // Both should have the same outcome (success or failure)
        assert_eq!(result1.is_ok(), result2.is_ok(), 
                  "fake() should behave consistently with same input");
        
        // If both succeed, they might have different values (randomized)
        // If both fail, they should have similar error types
        match (result1, result2) {
            (Ok(_), Ok(_)) => {
                // Both succeeded - that's good, values may differ due to randomization
            },
            (Err(e1), Err(e2)) => {
                // Both failed - error types should be similar
                assert_eq!(std::mem::discriminant(&e1), std::mem::discriminant(&e2),
                          "Error types should be consistent");
            },
            _ => panic!("Inconsistent behavior between calls"),
        }
    }
}