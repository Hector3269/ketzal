#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::Value;
    use ketzal::infrastructure::http::request::form_request::Validator;

    #[test]
    fn test_validator_make() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Value::String("John".to_string()));
        data.insert("email".to_string(), Value::String("john@example.com".to_string()));

        let mut rules = HashMap::new();
        rules.insert("name", "required|string");
        rules.insert("email", "required|email");

        let validator = Validator::make(data, rules);

        assert_eq!(validator.data.get("name").unwrap(), &Value::String("John".to_string()));
        // Note: rules field is private, so we can't test it directly
    }

    #[test]
    fn test_validator_set_custom_messages() {
        let data = HashMap::new();
        let rules = HashMap::new();
        let mut validator = Validator::make(data, rules);

        let mut messages = HashMap::new();
        messages.insert("name.required", "Name is required");
        messages.insert("email.email", "Invalid email format");

        validator.set_custom_messages(messages);

        assert_eq!(validator.custom_messages.get("name.required").unwrap(), "Name is required");
        assert_eq!(validator.custom_messages.get("email.email").unwrap(), "Invalid email format");
    }

    #[test]
    fn test_validator_set_custom_attributes() {
        let data = HashMap::new();
        let rules = HashMap::new();
        let mut validator = Validator::make(data, rules);

        let mut attributes = HashMap::new();
        attributes.insert("name", "Full Name");
        attributes.insert("email", "Email Address");

        validator.set_custom_attributes(attributes);

        assert_eq!(validator.custom_attributes.get("name").unwrap(), "Full Name");
        assert_eq!(validator.custom_attributes.get("email").unwrap(), "Email Address");
    }

    #[test]
    fn test_validator_validate_success() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Value::String("John".to_string()));
        data.insert("email".to_string(), Value::String("john@example.com".to_string()));

        let mut rules = HashMap::new();
        rules.insert("name", "required|string");
        rules.insert("email", "required|email");

        let mut validator = Validator::make(data, rules);

        let result = validator.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validator_validate_failure() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Value::String("".to_string()));
        data.insert("email".to_string(), Value::String("invalid-email".to_string()));

        let mut rules = HashMap::new();
        rules.insert("name", "required");
        rules.insert("email", "required|email");

        let mut validator = Validator::make(data, rules);

        let result = validator.validate();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.contains_key("name"));
        assert!(errors.contains_key("email"));
    }

    #[test]
    fn test_validator_validated_data() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Value::String("John".to_string()));
        data.insert("email".to_string(), Value::String("john@example.com".to_string()));
        data.insert("extra".to_string(), Value::String("not validated".to_string()));

        let mut rules = HashMap::new();
        rules.insert("name", "required");
        rules.insert("email", "required");

        let validator = Validator::make(data, rules);

        let validated = validator.validated_data();
        assert_eq!(validated.len(), 2);
        assert!(validated.contains_key("name"));
        assert!(validated.contains_key("email"));
        assert!(!validated.contains_key("extra"));
    }

    #[test]
    fn test_validator_nullable_rule() {
        let data = HashMap::new();
        // name is not present in data

        let mut rules = HashMap::new();
        rules.insert("name", "nullable|string");

        let mut validator = Validator::make(data, rules);

        let result = validator.validate();
        assert!(result.is_ok());
    }
}