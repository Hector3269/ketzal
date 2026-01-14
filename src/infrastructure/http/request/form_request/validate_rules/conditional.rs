use crate::infrastructure::http::request::form_request::validator::Validator;
use serde_json::Value;

pub fn required_if(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    params: Option<&str>,
) -> Result<(), String> {
    let custom_message = validator
        .custom_messages
        .get(&format!("{}.required_if", field))
        .map(|s| s.as_str());

    let params = match params {
        Some(p) => p,
        None => return Ok(()),
    };

    let mut split = params.splitn(2, ',');
    let other_field = split.next().unwrap();
    let expected_value = split.next().unwrap_or("");

    let other_val = validator.data.get(other_field);

    if other_val.map(|v| normalize(v)) == Some(expected_value.to_string()) {
        if value.is_none() || value == Some(&Value::Null) {
            return Err(custom_message.map(|s| s.to_string()).unwrap_or_else(|| {
                format!(
                    "The field {} is required when {} is {}.",
                    field_name, other_field, expected_value
                )
            }));
        }
    }

    Ok(())
}

fn normalize(v: &Value) -> String {
    match v {
        Value::String(s) => s.trim().to_lowercase(),
        _ => v.to_string().trim().to_lowercase(),
    }
}
