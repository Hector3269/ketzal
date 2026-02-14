use crate::validator::Validator;
use serde_json::Value;

pub fn string(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let custom_message = validator
        .custom_messages
        .get(&format!("{}.string", field))
        .map(|s| s.as_str());
    if let Some(v) = value {
        if !v.is_string() {
            return Err(custom_message
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("The field {} must be a string.", field_name)));
        }
    }
    Ok(())
}
