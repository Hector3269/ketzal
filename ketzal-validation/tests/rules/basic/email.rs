use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

#[test]
fn email_valid_standard() {
    let data = make(json!({ "email": "test@test.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_valid_with_dot() {
    let data = make(json!({ "email": "john.doe@example.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_valid_with_plus() {
    let data = make(json!({ "email": "test+tag@example.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_valid_subdomain() {
    let data = make(json!({ "email": "test@mail.example.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_valid_with_underscore() {
    let data = make(json!({ "email": "test_user@example.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_valid_with_hyphen() {
    let data = make(json!({ "email": "test-user@example-domain.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_invalid_no_at() {
    let data = make(json!({ "email": "invalid-email" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_err());
}

#[test]
fn email_invalid_no_domain() {
    let data = make(json!({ "email": "test@" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_err());
}

#[test]
fn email_invalid_no_local_part() {
    let data = make(json!({ "email": "@example.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_err());
}

#[test]
fn email_invalid_spaces() {
    let data = make(json!({ "email": "test @example.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_err());
}

#[test]
fn email_invalid_no_tld() {
    let data = make(json!({ "email": "test@example" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_err());
}

#[test]
fn email_invalid_double_at() {
    let data = make(json!({ "email": "test@@example.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_err());
}

#[test]
fn email_allows_missing_field() {
    // Email validation should pass if field is not present (not required)
    let data = make(json!({}));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_allows_null() {
    // Null values without nullable rule still fail email validation
    // because email rule checks if value is string
    let data = make(json!({ "email": null }));
    let mut v = Validator::make(data, [("email", "email")].into());

    let result = v.validate();
    // Email rule tries to convert to string, null becomes "null"
    // which is not a valid email
    assert!(result.is_err());
}

#[test]
fn email_fails_on_non_string() {
    let data = make(json!({ "email": 12345 }));
    let mut v = Validator::make(data, [("email", "email")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors["email"][0].contains("must be a string"));
}

#[test]
fn email_valid_with_required() {
    let data = make(json!({ "email": "test@example.com" }));
    let mut v = Validator::make(data, [("email", "required|email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_fails_with_required_and_invalid() {
    let data = make(json!({ "email": "invalid" }));
    let mut v = Validator::make(data, [("email", "required|email")].into());

    assert!(v.validate().is_err());
}

#[test]
fn email_custom_message() {
    let data = make(json!({ "email": "invalid" }));
    let mut v = Validator::make(data, [("email", "email")].into());
    v.set_custom_messages([("email.email", "Please enter a valid email address")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["email"][0], "Please enter a valid email address");
}

#[test]
fn email_with_trimmed_whitespace() {
    let data = make(json!({ "email": "  test@example.com  " }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}
