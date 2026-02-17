use crate::validator::Validator;
use serde_json::Value;
pub fn confirmed(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let v = match value {
        Some(v) => v,
        None => return Ok(()),
    };

    let custom_message = validator
        .custom_messages
        .get(&format!("{}.confirmed", field))
        .map(|s| s.as_str());

    let confirmation_key = format!("{}_confirmation", field);

    let conf = match validator.data.get(&confirmation_key) {
        Some(v) => v,
        None => {
            return Err(custom_message
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("The confirmation of {} does not match.", field_name)));
        }
    };

    if normalize(v) != normalize(conf) {
        return Err(custom_message
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("The confirmation of {} does not match.", field_name)));
    }

    Ok(())
}

fn normalize(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "".to_string(),
        _ => v.to_string(),
    }
}
