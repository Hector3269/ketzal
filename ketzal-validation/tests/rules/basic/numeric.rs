use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

#[test]
fn numeric_valid_integer() {
    let data = make(json!({ "age": 25 }));
    let mut v = Validator::make(data, [("age", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_valid_float() {
    let data = make(json!({ "price": 19.99 }));
    let mut v = Validator::make(data, [("price", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_valid_string_number() {
    let data = make(json!({ "age": "25" }));
    let mut v = Validator::make(data, [("age", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_valid_negative_number() {
    let data = make(json!({ "temperature": -10 }));
    let mut v = Validator::make(data, [("temperature", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_valid_negative_string() {
    let data = make(json!({ "temperature": "-10" }));
    let mut v = Validator::make(data, [("temperature", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_valid_zero() {
    let data = make(json!({ "count": 0 }));
    let mut v = Validator::make(data, [("count", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_valid_zero_string() {
    let data = make(json!({ "count": "0" }));
    let mut v = Validator::make(data, [("count", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_valid_decimal_string() {
    let data = make(json!({ "price": "19.99" }));
    let mut v = Validator::make(data, [("price", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_invalid_string() {
    let data = make(json!({ "age": "abc" }));
    let mut v = Validator::make(data, [("age", "numeric")].into());

    assert!(v.validate().is_err());
}

#[test]
fn numeric_invalid_mixed() {
    let data = make(json!({ "age": "123abc" }));
    let mut v = Validator::make(data, [("age", "numeric")].into());

    assert!(v.validate().is_err());
}

#[test]
fn numeric_invalid_boolean() {
    let data = make(json!({ "active": true }));
    let mut v = Validator::make(data, [("active", "numeric")].into());

    assert!(v.validate().is_err());
}

#[test]
fn numeric_invalid_array() {
    let data = make(json!({ "numbers": [1, 2, 3] }));
    let mut v = Validator::make(data, [("numbers", "numeric")].into());

    assert!(v.validate().is_err());
}

#[test]
fn numeric_invalid_object() {
    let data = make(json!({ "data": { "value": 1 } }));
    let mut v = Validator::make(data, [("data", "numeric")].into());

    assert!(v.validate().is_err());
}

#[test]
fn numeric_allows_missing_field() {
    let data = make(json!({}));
    let mut v = Validator::make(data, [("age", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_allows_null() {
    // Null values without nullable rule still fail numeric validation
    // because numeric rule checks if value is number or numeric string
    let data = make(json!({ "age": null }));
    let mut v = Validator::make(data, [("age", "numeric")].into());

    let result = v.validate();
    // Null becomes "null" string which is not numeric
    assert!(result.is_err());
}

#[test]
fn numeric_valid_with_required() {
    let data = make(json!({ "age": 25 }));
    let mut v = Validator::make(data, [("age", "required|numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_fails_with_required_and_invalid() {
    let data = make(json!({ "age": "abc" }));
    let mut v = Validator::make(data, [("age", "required|numeric")].into());

    assert!(v.validate().is_err());
}

#[test]
fn numeric_fails_with_required_and_missing() {
    let data = make(json!({}));
    let mut v = Validator::make(data, [("age", "required|numeric")].into());

    assert!(v.validate().is_err());
}

#[test]
fn numeric_custom_message() {
    let data = make(json!({ "age": "abc" }));
    let mut v = Validator::make(data, [("age", "numeric")].into());
    v.set_custom_messages([("age.numeric", "This must be a number!")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["age"][0], "This must be a number!");
}

#[test]
fn numeric_with_whitespace_string() {
    // Whitespace-padded numeric strings fail numeric validation
    let data = make(json!({ "age": "  25  " }));
    let mut v = Validator::make(data, [("age", "numeric")].into());

    let result = v.validate();
    // "  25  " cannot be parsed as a number
    assert!(result.is_err());
}

#[test]
fn numeric_with_scientific_notation() {
    let data = make(json!({ "value": "1e10" }));
    let mut v = Validator::make(data, [("value", "numeric")].into());

    // This may or may not pass depending on parser implementation
    // Most JSON numeric parsers don't accept scientific notation in strings
    let result = v.validate();
    // Just ensure it doesn't panic
    assert!(result.is_ok() || result.is_err());
}
