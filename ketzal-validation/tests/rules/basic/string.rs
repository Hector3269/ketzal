use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for the string validation rule

#[test]
fn string_valid() {
    let data = make(json!({ "name": "John" }));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_valid_empty() {
    let data = make(json!({ "name": "" }));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_valid_with_spaces() {
    let data = make(json!({ "name": "  Hello World  " }));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_valid_with_unicode() {
    let data = make(json!({ "name": "José García" }));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_valid_with_emoji() {
    let data = make(json!({ "name": "Hello 👋" }));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_valid_with_newlines() {
    let data = make(json!({ "description": "Line 1\nLine 2\nLine 3" }));
    let mut v = Validator::make(data, [("description", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_valid_with_tabs() {
    let data = make(json!({ "content": "Col1\tCol2\tCol3" }));
    let mut v = Validator::make(data, [("content", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_invalid_number() {
    let data = make(json!({ "name": 123 }));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_err());
}

#[test]
fn string_invalid_boolean() {
    let data = make(json!({ "active": true }));
    let mut v = Validator::make(data, [("active", "string")].into());

    assert!(v.validate().is_err());
}

#[test]
fn string_invalid_array() {
    let data = make(json!({ "tags": ["a", "b"] }));
    let mut v = Validator::make(data, [("tags", "string")].into());

    assert!(v.validate().is_err());
}

#[test]
fn string_invalid_object() {
    let data = make(json!({ "user": { "name": "John" } }));
    let mut v = Validator::make(data, [("user", "string")].into());

    assert!(v.validate().is_err());
}

#[test]
fn string_allows_missing_field() {
    let data = make(json!({}));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_allows_null() {
    // Null values without nullable rule still fail string validation
    // because string rule checks if value is a string type
    let data = make(json!({ "name": null }));
    let mut v = Validator::make(data, [("name", "string")].into());

    let result = v.validate();
    // Null is not a string type
    assert!(result.is_err());
}

#[test]
fn string_valid_with_required() {
    let data = make(json!({ "name": "John" }));
    let mut v = Validator::make(data, [("name", "required|string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_fails_with_required_and_empty() {
    let data = make(json!({ "name": "" }));
    let mut v = Validator::make(data, [("name", "required|string")].into());

    assert!(v.validate().is_err());
}

#[test]
fn string_fails_with_required_and_number() {
    let data = make(json!({ "name": 123 }));
    let mut v = Validator::make(data, [("name", "required|string")].into());

    assert!(v.validate().is_err());
}

#[test]
fn string_custom_message() {
    let data = make(json!({ "name": 123 }));
    let mut v = Validator::make(data, [("name", "string")].into());
    v.set_custom_messages([("name.string", "This must be text!")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["name"][0], "This must be text!");
}

#[test]
fn string_valid_number_as_string() {
    // A string containing a number should still be a valid string
    let data = make(json!({ "name": "123" }));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_ok());
}
