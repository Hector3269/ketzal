use crate::validator::Validator;
use serde_json::Value;

pub fn in_val(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    param: Option<&str>,
) -> Result<(), String> {
    let custom_message = validator.custom_messages.get(&format!("{field}.in")).map(String::as_str);
    let allowed: Vec<&str> = param.unwrap_or("").split(',').collect();
    if let Some(v) = value {
        let val_str = match v {
            Value::String(s) => s,
            _ => &v.to_string(),
        };
        if !allowed.contains(&val_str.as_str()) {
            return Err(custom_message
                .map(ToString::to_string)
                .unwrap_or_else(|| format!("The field {field_name} must be one of: {allowed:?}")));
        }
    }
    Ok(())
}
