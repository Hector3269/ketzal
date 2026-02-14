use crate::validator::Validator;
use serde_json::Value;

pub fn confirmed(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let custom_message = validator
        .custom_messages
        .get(&format!("{}.confirmed", field))
        .map(|s| s.as_str());

    let confirmation = format!("{}_confirmation", field);

    let v = value.ok_or_else(|| {
        custom_message
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("The field {} is required.", field_name))
    })?;

    let conf = validator.data.get(&confirmation).ok_or_else(|| {
        custom_message
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("The confirmation of {} does not match.", field_name))
    })?;

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
