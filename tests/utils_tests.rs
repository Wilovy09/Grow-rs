use grow_rs::utils::{map_io_error};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[test]
fn test_map_io_error_not_found() {
    let path = PathBuf::from("/some/test/path");
    let error_mapper = map_io_error(&path);
    
    let not_found_error = Error::new(ErrorKind::NotFound, "File not found");
    let result = error_mapper(not_found_error);
    
    assert_eq!(result, "File \"/some/test/path\" not found");
}

#[test]
fn test_map_io_error_other_errors() {
    let path = PathBuf::from("/some/test/path");
    let error_mapper = map_io_error(&path);
    
    let permission_error = Error::new(ErrorKind::PermissionDenied, "Permission denied");
    let result = error_mapper(permission_error);
    
    assert!(result.contains("\"/some/test/path\""));
    assert!(result.contains("Permission denied"));
}

#[test]
fn test_map_io_error_with_string_path() {
    let path = "test_file.ron";
    let error_mapper = map_io_error(path);
    
    let not_found_error = Error::new(ErrorKind::NotFound, "File not found");
    let result = error_mapper(not_found_error);
    
    assert_eq!(result, "File \"test_file.ron\" not found");
}

#[test]
fn test_map_io_error_with_different_error_kinds() {
    let path = PathBuf::from("/test/path");
    let error_mapper = map_io_error(&path);
    
    // Test different error kinds
    let errors = [
        (ErrorKind::InvalidInput, "Invalid input"),
        (ErrorKind::InvalidData, "Invalid data"),
        (ErrorKind::TimedOut, "Operation timed out"),
        (ErrorKind::Interrupted, "Operation interrupted"),
    ];
    
    for (kind, message) in errors.iter() {
        let error = Error::new(*kind, *message);
        let result = error_mapper(error);
        
        assert!(result.contains("\"/test/path\""));
        assert!(result.contains(message));
    }
}