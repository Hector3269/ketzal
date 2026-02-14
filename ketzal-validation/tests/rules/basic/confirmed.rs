use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for the confirmed validation rule (field must match {field}_confirmation)

#[test]
fn confirmed_valid_same_value() {
    let data = make(json!({
        "password": "secret123",
        "password_confirmation": "secret123"
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_invalid_different_values() {
    let data = make(json!({
        "password": "secret123",
        "password_confirmation": "different"
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    assert!(v.validate().is_err());
}

#[test]
fn confirmed_valid_with_numbers() {
    let data = make(json!({
        "pin": "1234",
        "pin_confirmation": "1234"
    }));
    let mut v = Validator::make(data, [("pin", "confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_valid_with_mixed_case() {
    // The comparison is case-sensitive for strings
    let data = make(json!({
        "password": "Secret123",
        "password_confirmation": "Secret123"
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_invalid_case_mismatch() {
    let data = make(json!({
        "password": "Secret123",
        "password_confirmation": "secret123"
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    assert!(v.validate().is_err());
}

#[test]
fn confirmed_valid_empty_strings() {
    let data = make(json!({
        "password": "",
        "password_confirmation": ""
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_valid_with_required() {
    let data = make(json!({
        "password": "secret123",
        "password_confirmation": "secret123"
    }));
    let mut v = Validator::make(data, [("password", "required|confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_fails_required_without_confirmation() {
    let data = make(json!({
        "password": "secret123"
    }));
    let mut v = Validator::make(data, [("password", "required|confirmed")].into());

    assert!(v.validate().is_err());
}

#[test]
fn confirmed_fails_required_without_field() {
    let data = make(json!({
        "password_confirmation": "secret123"
    }));
    let mut v = Validator::make(data, [("password", "required|confirmed")].into());

    assert!(v.validate().is_err());
}

#[test]
fn confirmed_fails_confirmation_missing() {
    let data = make(json!({
        "password": "secret123"
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors["password"][0].contains("confirmation"));
}

#[test]
fn confirmed_custom_message() {
    let data = make(json!({
        "password": "secret123",
        "password_confirmation": "different"
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());
    v.set_custom_messages([("password.confirmed", "Passwords do not match!")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["password"][0], "Passwords do not match!");
}

#[test]
fn confirmed_with_email_example() {
    // Common use case: email confirmation
    let data = make(json!({
        "email": "user@example.com",
        "email_confirmation": "user@example.com"
    }));
    let mut v = Validator::make(data, [("email", "email|confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_valid_with_boolean_true() {
    let data = make(json!({
        "terms": true,
        "terms_confirmation": true
    }));
    let mut v = Validator::make(data, [("terms", "confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_valid_with_boolean_false() {
    let data = make(json!({
        "terms": false,
        "terms_confirmation": false
    }));
    let mut v = Validator::make(data, [("terms", "confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_invalid_boolean_mismatch() {
    let data = make(json!({
        "terms": true,
        "terms_confirmation": false
    }));
    let mut v = Validator::make(data, [("terms", "confirmed")].into());

    assert!(v.validate().is_err());
}

#[test]
fn confirmed_with_null_confirmation() {
    let data = make(json!({
        "password": "secret123",
        "password_confirmation": null
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    assert!(v.validate().is_err());
}

#[test]
fn confirmed_with_numeric_values() {
    let data = make(json!({
        "amount": 100,
        "amount_confirmation": 100
    }));
    let mut v = Validator::make(data, [("amount", "confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_numeric_mismatch() {
    let data = make(json!({
        "amount": 100,
        "amount_confirmation": 200
    }));
    let mut v = Validator::make(data, [("amount", "confirmed")].into());

    assert!(v.validate().is_err());
}

#[test]
fn confirmed_with_whitespace_mismatch() {
    let data = make(json!({
        "password": "secret123",
        "password_confirmation": " secret123"
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    // Whitespace matters in comparison
    assert!(v.validate().is_err());
}

#[test]
fn confirmed_missing_confirmation_field() {
    // When confirmation field is completely missing
    let data = make(json!({ "password": "secret123" }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    let result = v.validate();
    assert!(result.is_err());
}

#[test]
fn confirmed_with_min_and_confirmed() {
    let data = make(json!({
        "password": "secret123",
        "password_confirmation": "secret123"
    }));
    let mut v = Validator::make(data, [("password", "min:8|confirmed")].into());

    assert!(v.validate().is_ok());
}
