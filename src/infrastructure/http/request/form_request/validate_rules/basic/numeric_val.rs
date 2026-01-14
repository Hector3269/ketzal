use crate::infrastructure::http::request::form_request::validator::Validator;
use serde_json::Value;

pub fn numeric(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let custom_message = validator
        .custom_messages
        .get(&format!("{}.numeric", field))
        .map(|s| s.as_str());
    if let Some(v) = value {
        let is_numeric = v.is_number() || v.as_str().map_or(false, |s| s.parse::<f64>().is_ok());
        if !is_numeric {
            return Err(custom_message
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("The field {} must be numeric.", field_name)));
        }
    }
    Ok(())
}
