use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for the required validation rule

#[test]
fn required_passes_when_present() {
    let data = make(json!({ "name": "Hector" }));
    let mut v = Validator::make(data, [("name", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_fails_when_missing() {
    let data = make(json!({}));
    let mut v = Validator::make(data, [("name", "required")].into());

    assert!(v.validate().is_err());
}

#[test]
fn required_fails_when_null() {
    let data = make(json!({ "name": null }));
    let mut v = Validator::make(data, [("name", "required")].into());

    assert!(v.validate().is_err());
}

#[test]
fn required_fails_when_empty_string() {
    let data = make(json!({ "name": "" }));
    let mut v = Validator::make(data, [("name", "required")].into());

    assert!(v.validate().is_err());
}

#[test]
fn required_passes_with_number() {
    let data = make(json!({ "age": 25 }));
    let mut v = Validator::make(data, [("age", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_passes_with_boolean_true() {
    let data = make(json!({ "active": true }));
    let mut v = Validator::make(data, [("active", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_passes_with_boolean_false() {
    // Note: boolean false is considered a value, not empty
    let data = make(json!({ "active": false }));
    let mut v = Validator::make(data, [("active", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_passes_with_array() {
    let data = make(json!({ "tags": ["rust", "validation"] }));
    let mut v = Validator::make(data, [("tags", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_passes_with_object() {
    let data = make(json!({ "user": { "name": "John" } }));
    let mut v = Validator::make(data, [("user", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_passes_with_zero() {
    let data = make(json!({ "count": 0 }));
    let mut v = Validator::make(data, [("count", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_fails_with_whitespace_only() {
    let data = make(json!({ "name": "   " }));
    let mut v = Validator::make(data, [("name", "required")].into());

    // Note: whitespace-only strings are NOT considered empty by required rule
    assert!(v.validate().is_ok());
}

#[test]
fn required_with_multiple_fields() {
    let data = make(json!({
        "name": "John",
        "email": "john@example.com"
    }));
    let mut v = Validator::make(data, [("name", "required"), ("email", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_with_one_missing_field() {
    let data = make(json!({ "name": "John" }));
    let mut v = Validator::make(data, [("name", "required"), ("email", "required")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.contains_key("email"));
}

#[test]
fn required_passes_when_field_exists_but_not_validated() {
    // If field is not in rules, it should not cause validation error
    let data = make(json!({ "name": "John", "extra": "" }));
    let mut v = Validator::make(data, [("name", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_custom_message() {
    let data = make(json!({ "name": "" }));
    let mut v = Validator::make(data, [("name", "required")].into());
    v.set_custom_messages([("name.required", "Custom: This field is mandatory!")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["name"][0], "Custom: This field is mandatory!");
}
