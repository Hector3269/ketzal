use crate::utils::get_email_regex;
use crate::validator::Validator;
use serde_json::Value;

pub fn email(
    field: &str,
    field_name: &str,
    value: Option<&Value>,
    validator: &Validator,
    _: Option<&str>,
) -> Result<(), String> {
    let email_regex = get_email_regex();

    let Some(v) = value else {
        return Ok(());
    };

    let s = v.as_str().ok_or_else(|| {
        validator
            .custom_messages
            .get(&format!("{field}.string"))
            .cloned()
            .unwrap_or_else(|| format!("The field {field_name} must be a string."))
    })?;

    let s = s.trim();

    if !email_regex.is_match(s) {
        return Err(validator
            .custom_messages
            .get(&format!("{field}.email"))
            .cloned()
            .unwrap_or_else(|| format!("The field {field_name} must be a valid email.")));
    }

    Ok(())
}
