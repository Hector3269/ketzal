use crate::validator::Validator;
use serde_json::Value;

pub fn string(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let custom_message =
        validator.custom_messages.get(&format!("{field}.string")).map(String::as_str);
    if let Some(v) = value {
        if !v.is_string() {
            return Err(custom_message
                .map(ToString::to_string)
                .unwrap_or_else(|| format!("The field {field_name} must be a string.")));
        }
    }
    Ok(())
}
