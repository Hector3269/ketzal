use crate::validator::Validator;
use serde_json::Value;

pub fn required_if(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    params: Option<&str>,
) -> Result<(), String> {
    let custom_message =
        validator.custom_messages.get(&format!("{field}.required_if")).map(String::as_str);

    let Some(params) = params else {
        return Ok(());
    };

    let mut split = params.splitn(2, ',');
    let other_field = split.next().unwrap();
    let expected_value = split.next().unwrap_or("");

    let other_val = validator.data.get(other_field);

    if other_val.map(normalize) == Some(expected_value.to_string())
        && (value.is_none() || value == Some(&Value::Null))
    {
        return Err(custom_message.map(str::to_string).unwrap_or_else(|| {
            format!("The field {field_name} is required when {other_field} is {expected_value}.")
        }));
    }

    Ok(())
}

fn normalize(v: &Value) -> String {
    match v {
        Value::String(s) => s.trim().to_lowercase(),
        _ => v.to_string().trim().to_lowercase(),
    }
}
