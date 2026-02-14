use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for the max validation rule (string length, number value, array length)

#[test]
fn max_string_exact_length() {
    let data = make(json!({ "name": "abcd" }));
    let mut v = Validator::make(data, [("name", "max:4")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_string_shorter() {
    let data = make(json!({ "name": "ab" }));
    let mut v = Validator::make(data, [("name", "max:4")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_string_longer() {
    let data = make(json!({ "name": "abcdef" }));
    let mut v = Validator::make(data, [("name", "max:4")].into());

    assert!(v.validate().is_err());
}

#[test]
fn max_string_with_spaces() {
    // Spaces count towards length
    let data = make(json!({ "name": "  abcdef  " }));
    let mut v = Validator::make(data, [("name", "max:8")].into());

    // "  abcdef  " has 10 characters including spaces, which exceeds max:8
    assert!(v.validate().is_err());
}

#[test]
fn max_number_less() {
    let data = make(json!({ "age": 15 }));
    let mut v = Validator::make(data, [("age", "max:18")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_number_exact() {
    let data = make(json!({ "age": 18 }));
    let mut v = Validator::make(data, [("age", "max:18")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_number_greater() {
    let data = make(json!({ "age": 20 }));
    let mut v = Validator::make(data, [("age", "max:18")].into());

    assert!(v.validate().is_err());
}

#[test]
fn max_number_negative() {
    // Negative numbers: max:-10 means value must be <= -10
    let data = make(json!({ "temperature": -15 }));
    let mut v = Validator::make(data, [("temperature", "max:-10")].into());

    // -15 is less than -10, which passes (<= -10 means -15 <= -10 is true)
    assert!(v.validate().is_ok());
}

#[test]
fn max_number_string() {
    let data = make(json!({ "age": "15" }));
    let mut v = Validator::make(data, [("age", "max:18")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_array_shorter() {
    let data = make(json!({ "tags": ["a", "b"] }));
    let mut v = Validator::make(data, [("tags", "max:3")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_array_exact_length() {
    let data = make(json!({ "tags": ["a", "b", "c"] }));
    let mut v = Validator::make(data, [("tags", "max:3")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_array_too_long() {
    let data = make(json!({ "tags": ["a", "b", "c", "d"] }));
    let mut v = Validator::make(data, [("tags", "max:3")].into());

    assert!(v.validate().is_err());
}

#[test]
fn max_empty_array() {
    let data = make(json!({ "tags": [] }));
    let mut v = Validator::make(data, [("tags", "max:1")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_missing_field() {
    let data = make(json!({}));
    let mut v = Validator::make(data, [("name", "max:4")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_null_field() {
    let data = make(json!({ "name": null }));
    let mut v = Validator::make(data, [("name", "max:4")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_empty_string() {
    let data = make(json!({ "name": "" }));
    let mut v = Validator::make(data, [("name", "max:1")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_with_required() {
    let data = make(json!({ "password": "12345678" }));
    let mut v = Validator::make(data, [("password", "required|max:20")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_fails_required_and_too_long() {
    let data = make(json!({ "password": "123456789012345678901" }));
    let mut v = Validator::make(data, [("password", "required|max:20")].into());

    assert!(v.validate().is_err());
}

#[test]
fn max_custom_message() {
    let data = make(json!({ "name": "abcdefgh" }));
    let mut v = Validator::make(data, [("name", "max:4")].into());
    v.set_custom_messages([("name.max", "Maximum 4 characters allowed!")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["name"][0], "Maximum 4 characters allowed!");
}

#[test]
fn max_default_zero() {
    // If no parameter is provided, default is 0
    let data = make(json!({ "name": "" }));
    let mut v = Validator::make(data, [("name", "max")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_invalid_param_uses_zero() {
    // If parameter is invalid, it defaults to 0
    let data = make(json!({ "name": "a" }));
    let mut v = Validator::make(data, [("name", "max:abc")].into());

    // Since max defaults to 0, any non-empty string should fail
    assert!(v.validate().is_err());
}

#[test]
fn max_combined_with_min() {
    let data = make(json!({ "name": "abc" }));
    let mut v = Validator::make(data, [("name", "min:2|max:5")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_fails_combined_with_min() {
    let data = make(json!({ "name": "a" }));
    let mut v = Validator::make(data, [("name", "min:2|max:5")].into());

    assert!(v.validate().is_err());
}

#[test]
fn max_username_example() {
    // Common use case: username between 3 and 20 characters
    let data = make(json!({ "username": "john_doe_123" }));
    let mut v = Validator::make(data, [("username", "min:3|max:20")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_password_example() {
    // Common use case: password max 128 characters
    let data = make(json!({ "password": "a".repeat(128) }));
    let mut v = Validator::make(data, [("password", "min:8|max:128")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_password_too_long() {
    // Common use case: password max 128 characters
    let data = make(json!({ "password": "a".repeat(129) }));
    let mut v = Validator::make(data, [("password", "min:8|max:128")].into());

    assert!(v.validate().is_err());
}
