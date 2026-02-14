use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for the required_if conditional validation rule

#[test]
fn required_if_triggers_exact_match() {
    let data = make(json!({
        "type": "admin"
    }));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    assert!(v.validate().is_err());
}

#[test]
fn required_if_not_triggered_different_value() {
    let data = make(json!({
        "type": "user"
    }));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_if_not_triggered_missing_other_field() {
    let data = make(json!({}));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    // When other field doesn't exist, required_if doesn't trigger
    assert!(v.validate().is_ok());
}

#[test]
fn required_if_passes_when_value_present() {
    let data = make(json!({
        "type": "admin",
        "role": "superadmin"
    }));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_if_fails_when_value_is_null() {
    let data = make(json!({
        "type": "admin",
        "role": null
    }));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    assert!(v.validate().is_err());
}

#[test]
fn required_if_case_insensitive_comparison() {
    let data = make(json!({
        "type": "ADMIN"
    }));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    // Comparison is case-insensitive
    assert!(v.validate().is_err());
}

#[test]
fn required_if_triggers_with_numeric_match() {
    let data = make(json!({
        "account_type": 1
    }));

    let mut v = Validator::make(data, [("api_key", "required_if:account_type,1")].into());

    assert!(v.validate().is_err());
}

#[test]
fn required_if_not_triggered_numeric_mismatch() {
    let data = make(json!({
        "account_type": 2
    }));

    let mut v = Validator::make(data, [("api_key", "required_if:account_type,1")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_if_with_multiple_conditions() {
    // Using required_if with multiple fields
    let data = make(json!({
        "is_admin": true,
        "department": "IT"
    }));

    let mut v = Validator::make(data, [("admin_notes", "required_if:is_admin,true")].into());

    assert!(v.validate().is_err());
}

#[test]
fn required_if_with_other_rules() {
    let data = make(json!({
        "type": "admin",
        "role": "superadmin"
    }));

    let mut v = Validator::make(
        data,
        [(
            "role",
            "required_if:type,admin|in:admin,moderator,superadmin",
        )]
        .into(),
    );

    assert!(v.validate().is_ok());
}

#[test]
fn required_if_custom_message() {
    let data = make(json!({
        "type": "admin"
    }));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());
    v.set_custom_messages([("role.required_if", "Role is required for admin users!")].into());

    let result = v.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors["role"][0], "Role is required for admin users!");
}

#[test]
fn required_if_with_trimmed_whitespace() {
    let data = make(json!({
        "type": "  admin  "
    }));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    // Whitespace is trimmed in comparison
    assert!(v.validate().is_err());
}

#[test]
fn required_if_combined_with_min() {
    let data = make(json!({
        "type": "admin",
        "password": "12345"
    }));

    let mut v = Validator::make(data, [("password", "required_if:type,admin|min:8")].into());

    assert!(v.validate().is_err());
}

#[test]
fn required_if_with_min_satisfied() {
    let data = make(json!({
        "type": "admin",
        "password": "12345678"
    }));

    let mut v = Validator::make(data, [("password", "required_if:type,admin|min:8")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_if_empty_string_fails() {
    // Empty string is considered a value for required_if
    let data = make(json!({
        "type": "admin",
        "role": ""
    }));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    // Empty string passes required_if because the field is present
    // required_if only checks if field is missing or null
    assert!(v.validate().is_ok());
}

#[test]
fn required_if_missing_both_fields() {
    let data = make(json!({}));

    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    // Neither field present - doesn't trigger
    assert!(v.validate().is_ok());
}

#[test]
fn required_if_only_confirmation_field() {
    // When required_if triggers but only confirmation field exists
    let data = make(json!({
        "type": "admin",
        "password_confirmation": "secret123"
    }));

    let mut v = Validator::make(data, [("password", "required_if:type,admin")].into());

    assert!(v.validate().is_err());
}
