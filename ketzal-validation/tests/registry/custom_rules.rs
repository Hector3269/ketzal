use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests demonstrating custom rules usage patterns

// Note: The current implementation uses a static registry with OnceLock.
// Custom rules would need to be added at compile time or through internal APIs.
// These tests demonstrate how the validation works with the built-in rules.

#[test]
fn complex_registration_form() {
    let data = make(json!({
        "username": "john_doe",
        "email": "john@example.com",
        "password": "SecurePass123!",
        "password_confirmation": "SecurePass123!",
        "age": 25,
        "country": "US",
        "terms": true
    }));

    let mut v = Validator::make(
        data,
        [
            ("username", "required|string|min:3|max:20"),
            ("email", "required|email"),
            ("password", "required|min:8|confirmed"),
            ("age", "required|numeric|min:18"),
            ("country", "required|in:US,CA,MX,UK,AU"),
            ("terms", "required"),
        ]
        .into(),
    );

    assert!(v.validate().is_ok());
}

#[test]
fn login_form_validation() {
    let data = make(json!({
        "email": "user@example.com",
        "password": "mypassword"
    }));

    let mut v =
        Validator::make(data, [("email", "required|email"), ("password", "required|min:6")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn profile_update_validation() {
    let data = make(json!({
        "name": "John Doe",
        "bio": "Software developer",
        "website": "https://example.com",
        "twitter": "@johndoe"
    }));

    let mut v = Validator::make(
        data,
        [
            ("name", "required|string|min:2|max:100"),
            ("bio", "nullable|string|max:500"),
            ("website", "nullable|string"),
            ("twitter", "nullable|string"),
        ]
        .into(),
    );

    assert!(v.validate().is_ok());
}

#[test]
fn product_creation_validation() {
    let data = make(json!({
        "name": "Super Widget",
        "price": 99.99,
        "category": "electronics",
        "sku": "WID-001",
        "in_stock": true
    }));

    let mut v = Validator::make(
        data,
        [
            ("name", "required|string|min:3|max:200"),
            ("price", "required|numeric|min:0.01"),
            ("category", "required|in:electronics,clothing,food,books"),
            ("sku", "required|string|min:3|max:50"),
            ("in_stock", "required"),
        ]
        .into(),
    );

    assert!(v.validate().is_ok());
}

#[test]
fn payment_form_validation() {
    let data = make(json!({
        "card_number": "4111111111111111",
        "expiry": "12/25",
        "cvv": "123",
        "amount": 100.00
    }));

    let mut v = Validator::make(
        data,
        [
            ("card_number", "required|string|min:13|max:19"),
            ("expiry", "required|string|min:4|max:5"),
            ("cvv", "required|string|min:3|max:4"),
            ("amount", "required|numeric|min:0.01"),
        ]
        .into(),
    );

    assert!(v.validate().is_ok());
}

#[test]
fn nested_object_validation_not_supported() {
    // Current implementation doesn't support nested objects
    let data = make(json!({
        "user": {
            "name": "John",
            "email": "john@example.com"
        }
    }));

    let mut v = Validator::make(data, [("user.name", "required")].into());

    // This will fail because user.name is not found as a top-level key
    // and empty string is considered present (not null)
    assert!(v.validate().is_err());
}

#[test]
fn optional_fields_with_nullable() {
    // Nullable only skips when field is MISSING
    let data = make(json!({
        "required_field": "value",
        "optional_field": null,
        "another_optional": null
    }));

    let mut v = Validator::make(
        data,
        [
            ("required_field", "required"),
            ("optional_field", "nullable|email"),
            ("another_optional", "nullable|string"),
        ]
        .into(),
    );

    // Null values fail their respective rules
    assert!(v.validate().is_err());
}

#[test]
fn multiple_validation_failures() {
    let data = make(json!({
        "email": "not-an-email",
        "age": "not-a-number",
        "status": "invalid_status"
    }));

    let mut v = Validator::make(
        data,
        [
            ("email", "required|email"),
            ("age", "required|numeric"),
            ("status", "required|in:active,inactive"),
        ]
        .into(),
    );

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 3);
}

#[test]
fn first_rule_failure_stops_chain() {
    // When a rule fails, subsequent rules on same field may not run
    // But we can't easily test internal behavior
    let data = make(json!({
        "email": "not-an-email"
    }));

    let mut v = Validator::make(data, [("email", "required|email|min:10")].into());

    let result = v.validate();
    assert!(result.is_err());
    // We just know it fails - exact error depends on rule order
}

#[test]
fn empty_data_with_required_fields() {
    let data = make(json!({}));

    let mut v = Validator::make(data, [("email", "required|email"), ("name", "required")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
}

#[test]
fn partial_data_passes_partial_validation() {
    let data = make(json!({
        "email": "test@example.com"
    }));

    let mut v = Validator::make(
        data,
        [("email", "required|email"), ("name", "required"), ("age", "required|numeric")].into(),
    );

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();

    // Only name and age should have errors, email passed
    assert!(!errors.contains_key("email"));
    assert!(errors.contains_key("name"));
    assert!(errors.contains_key("age"));
}

#[test]
fn validated_data_filters_unvalidated() {
    let data = make(json!({
        "email": "test@example.com",
        "name": "John",
        "age": 25,
        "extra_field": "should not appear"
    }));

    let mut v = Validator::make(
        data,
        [("email", "required|email"), ("name", "required"), ("age", "required|numeric")].into(),
    );

    v.validate().unwrap();
    let validated = v.validated_data();

    // Only validated fields should be present
    assert_eq!(validated.len(), 3);
    assert!(validated.contains_key("email"));
    assert!(validated.contains_key("name"));
    assert!(validated.contains_key("age"));
    assert!(!validated.contains_key("extra_field"));
}

#[test]
fn custom_message_with_placeholder() {
    let data = make(json!({ "age": 15 }));
    let mut v = Validator::make(data, [("age", "min:18")].into());
    v.set_custom_messages([("age.min", "You must be at least 18 years old")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors["age"][0].contains("18"));
}

#[test]
fn multiple_errors_same_field() {
    let data = make(json!({ "name": "A" }));
    let mut v = Validator::make(data, [("name", "required|min:3|max:10")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();

    // Should have at least one error for name
    assert!(!errors["name"].is_empty());
}
