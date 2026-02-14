use crate::validator::Validator;
use serde_json::Value;

pub fn required(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let custom_message = validator
        .custom_messages
        .get(&format!("{}.required", field))
        .map(|s| s.as_str());
    if value.is_none() || value == Some(&Value::Null) || value == Some(&Value::String("".into())) {
        return Err(custom_message
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("The field {} is required.", field_name)));
    }
    Ok(())
}
