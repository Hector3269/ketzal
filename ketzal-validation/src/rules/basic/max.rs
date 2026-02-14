use crate::validator::Validator;
use serde_json::Value;

pub fn max(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    param: Option<&str>,
) -> Result<(), String> {
    let custom_message = validator
        .custom_messages
        .get(&format!("{}.max", field))
        .map(|s| s.as_str());
    let max_val = param.unwrap_or("0").parse().unwrap_or(0);
    if let Some(v) = value {
        let size = match v {
            Value::String(s) => s.len(),
            Value::Number(n) => n.as_f64().unwrap_or(0.0) as usize,
            Value::Array(arr) => arr.len(),
            _ => 0,
        };
        if size > max_val {
            return Err(custom_message.map(|s| s.to_string()).unwrap_or_else(|| {
                format!(
                    "The field {} may not be greater than {} characters.",
                    field_name, max_val
                )
            }));
        }
    }
    Ok(())
}
