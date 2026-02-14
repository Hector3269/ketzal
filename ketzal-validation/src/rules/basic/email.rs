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

    let v = match value {
        Some(val) => val,
        None => return Ok(()),
    };

    let s = v.as_str().ok_or_else(|| {
        format!(
            "{}",
            validator
                .custom_messages
                .get(&format!("{}.string", field))
                .cloned()
                .unwrap_or_else(|| format!("The field {} must be a string.", field_name))
        )
    })?;

    let s = s.trim();

    if !email_regex.is_match(s) {
        return Err(validator
            .custom_messages
            .get(&format!("{}.email", field))
            .cloned()
            .unwrap_or_else(|| format!("The field {} must be a valid email.", field_name)));
    }

    Ok(())
}
