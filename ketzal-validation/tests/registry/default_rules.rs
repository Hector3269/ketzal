use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for default rules available in the registry

#[test]
fn required_rule_exists() {
    let data = make(json!({ "name": "John" }));
    let mut v = Validator::make(data, [("name", "required")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn nullable_rule_exists() {
    let data = make(json!({ "name": null }));
    let mut v = Validator::make(data, [("name", "nullable")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn string_rule_exists() {
    let data = make(json!({ "name": "John" }));
    let mut v = Validator::make(data, [("name", "string")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn numeric_rule_exists() {
    let data = make(json!({ "age": 25 }));
    let mut v = Validator::make(data, [("age", "numeric")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn email_rule_exists() {
    let data = make(json!({ "email": "test@example.com" }));
    let mut v = Validator::make(data, [("email", "email")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn min_rule_exists() {
    let data = make(json!({ "name": "John" }));
    let mut v = Validator::make(data, [("name", "min:3")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn max_rule_exists() {
    let data = make(json!({ "name": "John" }));
    let mut v = Validator::make(data, [("name", "max:10")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn in_rule_exists() {
    let data = make(json!({ "status": "active" }));
    let mut v = Validator::make(data, [("status", "in:active,inactive")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn confirmed_rule_exists() {
    let data = make(json!({
        "password": "secret",
        "password_confirmation": "secret"
    }));
    let mut v = Validator::make(data, [("password", "confirmed")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn required_if_rule_exists() {
    let data = make(json!({ "type": "admin", "role": "superadmin" }));
    let mut v = Validator::make(data, [("role", "required_if:type,admin")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn all_default_rules_work_together() {
    let data = make(json!({
        "name": "John Doe",
        "email": "john@example.com",
        "age": 25,
        "status": "active",
        "password": "secret123",
        "password_confirmation": "secret123"
    }));

    let mut v = Validator::make(
        data,
        [
            ("name", "required|string|min:2|max:50"),
            ("email", "required|email"),
            ("age", "required|numeric|min:18"),
            ("status", "required|in:active,inactive,pending"),
            ("password", "required|min:8|confirmed"),
        ]
        .into(),
    );

    assert!(v.validate().is_ok());
}

#[test]
fn split_rule_parses_correctly() {
    use ketzal_validation::split_rule;

    let (name, param) = split_rule("required");
    assert_eq!(name, "required");
    assert_eq!(param, None);

    let (name, param) = split_rule("min:5");
    assert_eq!(name, "min");
    assert_eq!(param, Some("5"));

    let (name, param) = split_rule("in:a,b,c");
    assert_eq!(name, "in");
    assert_eq!(param, Some("a,b,c"));

    let (name, param) = split_rule("required_if:type,admin");
    assert_eq!(name, "required_if");
    assert_eq!(param, Some("type,admin"));
}

#[test]
fn unknown_rules_are_ignored() {
    // Unknown rules don't cause errors - they're silently skipped
    let data = make(json!({ "name": "John" }));
    let mut v = Validator::make(data, [("name", "unknown_rule|another_unknown")].into());

    assert!(v.validate().is_ok());
}

#[test]
fn rule_parameters_are_passed_correctly() {
    // Test that min:3 means minimum of 3
    let data_short = make(json!({ "name": "ab" }));
    let mut v_short = Validator::make(data_short, [("name", "min:3")].into());
    assert!(v_short.validate().is_err());

    let data_long = make(json!({ "name": "abc" }));
    let mut v_long = Validator::make(data_long, [("name", "min:3")].into());
    assert!(v_long.validate().is_ok());
}

#[test]
fn rule_parameters_with_colons() {
    // Test in rule with multiple values
    let data = make(json!({ "status": "pending" }));
    let mut v = Validator::make(data, [("status", "in:active,inactive,pending")].into());
    assert!(v.validate().is_ok());

    let data_bad = make(json!({ "status": "unknown" }));
    let mut v_bad = Validator::make(data_bad, [("status", "in:active,inactive,pending")].into());
    assert!(v_bad.validate().is_err());
}
