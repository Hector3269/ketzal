use crate::validator::Validator;
use serde_json::Value;

pub fn numeric(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let custom_message =
        validator.custom_messages.get(&format!("{field}.numeric")).map(String::as_str);

    if let Some(v) = value {
        let is_numeric = v.is_number() || v.as_str().is_some_and(|s| s.parse::<f64>().is_ok());

        if !is_numeric {
            return Err(custom_message
                .map(str::to_string)
                .unwrap_or_else(|| format!("The field {field_name} must be numeric.")));
        }
    }

    Ok(())
}
