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

    let is_empty = match value {
        None => true,
        Some(Value::Null) => true,
        Some(Value::String(s)) if s.is_empty() => true,
        _ => false,
    };

    if is_empty {
        return Err(custom_message
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("The field {} is required.", field_name)));
    }

    Ok(())
}
