use core::panic;

use laravel_index::{models::AppError, parser};

#[test]
fn test_parse_valid_line() {
    let line = "[2021-12-06 09:32:59] local.ERROR: SQLSTATE[HY000] [2002] Connection refused";
    let result = parser::parse_line(line, 1);

    assert!(result.is_some());
    let entry = result.unwrap();
    assert_eq!(entry.level, "local");
}

#[test]
fn test_parse_invalid_line() {
    let line = "Ini bukan format log Laravel yang benar";
    let result = parser::parse_line(line, 1);

    assert!(result.is_none());
}

#[test]
fn test_parse_database_error() {
    let line = "[2021-12-06 09:32:59] local.ERROR: SQLSTATE[HY000] [2002] Connection refused";
    let result = parser::parse_line(line, 1);

    assert!(result.is_some());
    let entry = result.unwrap();

    assert_eq!(entry.level, "local");
    match entry.error_type {
        AppError::DatabaseError(_) => assert!(true),
        _ => panic!("Should be DatabaseError!"),
    }
}

#[test]
fn test_parse_invalid_format() {
    let line = "This is unformated line";
    let result = parser::parse_line(line, 1);

    assert!(result.is_none());
}

#[test]
fn test_parse_multiline_context() {
    let line = "[2022-05-24 08:45:46] development.ERROR: Unable to locate Mix file {\"exception\":\"...\"}";
    let result = parser::parse_line(line, 1).unwrap();

    match result.error_type {
        AppError::ViewError(_) => assert!(true),
        _ => panic!("Should be ViewError!"),
    }
}
