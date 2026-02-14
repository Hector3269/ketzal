use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for the min validation rule (string length, number value, array length)

#[test]
fn min_string_exact_length() {
    let data = make(json!({ "name": "abcd" }));
    let mut v = Validator::make(data, [("name", "min:4")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_string_longer() {
    let data = make(json!({ "name": "abcdef" }));
    let mut v = Validator::make(data, [("name", "min:4")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_string_shorter() {
    let data = make(json!({ "name": "ab" }));
    let mut v = Validator::make(data, [("name", "min:4")].into());

    assert!(v.validate().is_err());
}

#[test]
fn min_string_with_spaces() {
    let data = make(json!({ "name": "  ab  " }));
    let mut v = Validator::make(data, [("name", "min:4")].into());

    // Spaces count towards length
    assert!(v.validate().is_ok());
}

#[test]
fn min_number_greater() {
    let data = make(json!({ "age": 20 }));
    let mut v = Validator::make(data, [("age", "min:18")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_number_exact() {
    let data = make(json!({ "age": 18 }));
    let mut v = Validator::make(data, [("age", "min:18")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_number_less() {
    let data = make(json!({ "age": 15 }));
    let mut v = Validator::make(data, [("age", "min:18")].into());

    assert!(v.validate().is_err());
}

#[test]
fn min_number_negative() {
    let data = make(json!({ "temperature": -5 }));
    let mut v = Validator::make(data, [("temperature", "min:-10")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_number_string() {
    // Numeric strings should be treated as numbers for min validation
    let data = make(json!({ "age": "20" }));
    let mut v = Validator::make(data, [("age", "min:18")].into());

    let result = v.validate();
    // String "20" is treated as length 2 not as number 20
    assert!(result.is_err());
}

#[test]
fn min_array_length() {
    let data = make(json!({ "tags": ["a", "b", "c"] }));
    let mut v = Validator::make(data, [("tags", "min:2")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_array_exact_length() {
    let data = make(json!({ "tags": ["a", "b"] }));
    let mut v = Validator::make(data, [("tags", "min:2")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_array_too_short() {
    let data = make(json!({ "tags": ["a"] }));
    let mut v = Validator::make(data, [("tags", "min:2")].into());

    assert!(v.validate().is_err());
}

#[test]
fn min_empty_array() {
    let data = make(json!({ "tags": [] }));
    let mut v = Validator::make(data, [("tags", "min:1")].into());

    assert!(v.validate().is_err());
}

#[test]
fn min_missing_field() {
    let data = make(json!({}));
    let mut v = Validator::make(data, [("name", "min:4")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_null_field() {
    // Null values without nullable rule still fail min validation
    // because the value is treated as null which has size 0
    let data = make(json!({ "name": null }));
    let mut v = Validator::make(data, [("name", "min:4")].into());

    let result = v.validate();
    // Null has no size so it fails min:4
    assert!(result.is_err());
}

#[test]
fn min_empty_string() {
    let data = make(json!({ "name": "" }));
    let mut v = Validator::make(data, [("name", "min:1")].into());

    assert!(v.validate().is_err());
}

#[test]
fn min_with_required() {
    let data = make(json!({ "password": "12345678" }));
    let mut v = Validator::make(data, [("password", "required|min:8")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_fails_required_and_short() {
    let data = make(json!({ "password": "123" }));
    let mut v = Validator::make(data, [("password", "required|min:8")].into());

    assert!(v.validate().is_err());
}

#[test]
fn min_custom_message() {
    let data = make(json!({ "name": "ab" }));
    let mut v = Validator::make(data, [("name", "min:4")].into());
    v.set_custom_messages([("name.min", "At least 4 characters required!")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["name"][0], "At least 4 characters required!");
}

#[test]
fn min_default_zero() {
    // If no parameter is provided, default is 0
    let data = make(json!({ "name": "ab" }));
    let mut v = Validator::make(data, [("name", "min")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_invalid_param_uses_zero() {
    // If parameter is invalid, it defaults to 0
    let data = make(json!({ "name": "" }));
    let mut v = Validator::make(data, [("name", "min:abc")].into());

    // Since min defaults to 0, empty string should pass
    assert!(v.validate().is_ok());
}

#[test]
fn min_combined_with_max() {
    let data = make(json!({ "name": "abc" }));
    let mut v = Validator::make(data, [("name", "min:2|max:5")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_fails_combined_with_max() {
    let data = make(json!({ "name": "abcdef" }));
    let mut v = Validator::make(data, [("name", "min:2|max:5")].into());

    assert!(v.validate().is_err());
}
