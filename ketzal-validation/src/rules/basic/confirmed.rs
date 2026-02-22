use crate::validator::Validator;
use serde_json::Value;

pub fn confirmed(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let Some(v) = value else {
        return Ok(());
    };

    let custom_message =
        validator.custom_messages.get(&format!("{field}.confirmed")).map(String::as_str);

    let confirmation_key = format!("{field}_confirmation");

    let Some(conf) = validator.data.get(&confirmation_key) else {
        return Err(custom_message.map_or_else(
            || format!("The confirmation of {field_name} does not match."),
            ToString::to_string,
        ));
    };

    if normalize(v) != normalize(conf) {
        return Err(custom_message.map_or_else(
            || format!("The confirmation of {field_name} does not match."),
            ToString::to_string,
        ));
    }

    Ok(())
}

fn normalize(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        _ => v.to_string(),
    }
}
