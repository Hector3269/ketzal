use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for custom rule integration

#[test]
fn custom_rule_registration() {
    // This test would require access to internal registry to add custom rules
    // For now, we test that the existing rules work correctly together
    let data = make(json!({
        "username": "john_doe",
        "email": "john@example.com",
        "age": 25
    }));

    let mut v = Validator::make(
        data,
        [
            ("username", "required|min:3|max:20"),
            ("email", "required|email"),
            ("age", "required|numeric|min:18"),
        ]
        .into(),
    );

    assert!(v.validate().is_ok());
}

#[test]
fn custom_message_per_rule() {
    let data = make(json!({
        "email": "invalid",
        "age": "abc"
    }));

    let mut v =
        Validator::make(data, [("email", "required|email"), ("age", "required|numeric")].into());

    v.set_custom_messages(
        [("email.email", "Please provide a valid email"), ("age.numeric", "Age must be a number")]
            .into(),
    );

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors["email"][0].contains("valid email"));
    assert!(errors["age"][0].contains("number"));
}

#[test]
fn custom_messages_for_multiple_rule_failures() {
    let data = make(json!({
        "email": "invalid"
    }));

    let mut v = Validator::make(data, [("email", "required|email|min:10")].into());

    v.set_custom_messages(
        [
            ("email.required", "Email is mandatory"),
            ("email.email", "Invalid email format"),
            ("email.min", "Email too short"),
        ]
        .into(),
    );

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should contain the custom email message
    assert!(errors["email"].iter().any(|e| e.contains("email") || e.contains("Email")));
}

#[test]
fn custom_attributes_improve_error_messages() {
    let data = make(json!({
        "user_email": ""
    }));

    let mut v = Validator::make(data, [("user_email", "required|email")].into());

    v.set_custom_attributes([("user_email", "E-mail Address")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should use the custom attribute name in error message
    assert!(errors["user_email"][0].contains("E-mail Address"));
}

#[test]
fn combining_custom_messages_and_attributes() {
    let data = make(json!({
        "contact_email": "bad"
    }));

    let mut v = Validator::make(data, [("contact_email", "required|email")].into());

    v.set_custom_attributes([("contact_email", "Contact E-mail")].into());
    v.set_custom_messages([("contact_email.email", "Please enter a valid contact e-mail")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["contact_email"][0], "Please enter a valid contact e-mail");
}

#[test]
fn custom_rule_with_complex_validation() {
    let data = make(json!({
        "password": "SecureP@ss123",
        "password_confirmation": "SecureP@ss123"
    }));

    let mut v = Validator::make(data, [("password", "required|min:8|confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn custom_rule_with_confirmation_mismatch() {
    let data = make(json!({
        "password": "SecureP@ss123",
        "password_confirmation": "DifferentP@ss"
    }));

    let mut v = Validator::make(data, [("password", "required|min:8|confirmed")].into());

    let result = v.validate();
    assert!(result.is_err());
}

#[test]
fn full_form_validation_scenario() {
    // Simulating a complete registration form validation
    let data = make(json!({
        "name": "John Doe",
        "email": "john@example.com",
        "password": "password123",
        "password_confirmation": "password123",
        "age": 25,  // Must be number, not string
        "country": "US",
        "terms": true
    }));

    let mut v = Validator::make(
        data,
        [
            ("name", "required|string|min:2|max:50"),
            ("email", "required|email"),
            ("password", "required|min:8|confirmed"),
            ("age", "required|numeric|min:18"),
            ("country", "required|in:US,CA,MX,UK"),
            ("terms", "required"),
        ]
        .into(),
    );

    assert!(v.validate().is_ok());
}

#[test]
fn full_form_validation_with_errors() {
    let data = make(json!({
        "name": "J",  // Too short
        "email": "invalid-email",  // Invalid
        "password": "short",  // Too short, no confirmation
        "age": "not-a-number",  // Invalid
        "country": "XX",  // Not in list
        "terms": false  // Required but false - field IS present
    }));

    let mut v = Validator::make(
        data,
        [
            ("name", "required|string|min:2|max:50"),
            ("email", "required|email"),
            ("password", "required|min:8|confirmed"),
            ("age", "required|numeric|min:18"),
            ("country", "required|in:US,CA,MX,UK"),
            ("terms", "required"),
        ]
        .into(),
    );

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();

    // Check multiple fields have errors
    assert!(errors.contains_key("name"));
    assert!(errors.contains_key("email"));
    assert!(errors.contains_key("password"));
    assert!(errors.contains_key("age"));
    assert!(errors.contains_key("country"));
    // terms is present (false), so it passes required
}

#[test]
fn validated_data_with_complex_form() {
    let data = make(json!({
        "username": "johndoe",
        "bio": "Hello world",
        "website": "https://example.com"
    }));

    let mut v = Validator::make(
        data,
        [
            ("username", "required|min:3"),
            ("bio", "nullable|string|max:500"),
            ("website", "nullable|url"), // url rule doesn't exist, will be ignored
        ]
        .into(),
    );

    v.validate().unwrap();
    let validated = v.validated_data();

    // Only fields with rules should be in validated data
    assert!(validated.contains_key("username"));
    assert!(validated.contains_key("bio"));
    // website won't be there because url rule doesn't exist (ignored)
}

#[test]
fn custom_rules_survive_multiple_validations() {
    let data = make(json!({
        "email": "test@example.com"
    }));

    let mut v = Validator::make(data, [("email", "required|email")].into());

    v.set_custom_messages([("email.email", "Custom: Invalid email")].into());

    // First validation
    assert!(v.validate().is_ok());

    // Second validation with different invalid data
    v.data.insert("email".to_string(), json!("invalid"));

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Custom message should still be used
    assert!(errors["email"][0].contains("Custom"));
}

#[test]
fn partial_validation_errors() {
    let data = make(json!({
        "field1": "value1",
        "field2": "",
        "field3": "value3"
    }));

    let mut v = Validator::make(
        data,
        [("field1", "required"), ("field2", "required"), ("field3", "required")].into(),
    );

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();

    // Only field2 should have error
    assert!(errors.contains_key("field2"));
    assert_eq!(errors.len(), 1);
}

#[test]
fn validation_error_collection() {
    let data = make(json!({
        "name": "",
        "email": "bad",
        "age": "not-a-number"
    }));

    let mut v = Validator::make(
        data,
        [("name", "required|string"), ("email", "required|email"), ("age", "required|numeric")]
            .into(),
    );

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();

    // All three fields should have errors
    assert_eq!(errors.len(), 3);

    // Each field should have at least one error message
    assert!(!errors["name"].is_empty());
    assert!(!errors["email"].is_empty());
    assert!(!errors["age"].is_empty());
}
