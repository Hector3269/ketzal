use std::collections::HashMap;

use serde_json::Value;

use ketzal_validation::{ValidationErrors, Validator};

/// Validates data with given rules and returns validated data on success
pub fn validate(
    data: HashMap<String, Value>,
    rules: HashMap<&'static str, &'static str>,
) -> Result<HashMap<String, Value>, ValidationErrors> {
    let mut validator = Validator::make(data, rules);
    validator.validate()?;
    Ok(validator.validated_data())
}

/// Validates data with given rules and custom messages
pub fn validate_with_messages(
    data: HashMap<String, Value>,
    rules: HashMap<&'static str, &'static str>,
    messages: HashMap<&'static str, &'static str>,
) -> Result<HashMap<String, Value>, ValidationErrors> {
    let mut validator = Validator::make(data, rules);
    validator.set_custom_messages(messages);
    validator.validate()?;
    Ok(validator.validated_data())
}

/// Validates data with given rules and custom attributes
pub fn validate_with_attributes(
    data: HashMap<String, Value>,
    rules: HashMap<&'static str, &'static str>,
    attributes: HashMap<&'static str, &'static str>,
) -> Result<HashMap<String, Value>, ValidationErrors> {
    let mut validator = Validator::make(data, rules);
    validator.set_custom_attributes(attributes);
    validator.validate()?;
    Ok(validator.validated_data())
}

/// Validates data and returns the validator to inspect errors
pub fn validate_full(
    data: HashMap<String, Value>,
    rules: HashMap<&'static str, &'static str>,
) -> Result<Validator, ValidationErrors> {
    let mut validator = Validator::make(data, rules);
    validator.validate()?;
    Ok(validator)
}

/// Creates a validator for inspection (doesn't validate)
pub fn make_validator(
    data: HashMap<String, Value>,
    rules: HashMap<&'static str, &'static str>,
) -> Validator {
    Validator::make(data, rules)
}
