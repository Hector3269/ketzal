#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::Value;
    use ketzal::infrastructure::http::request::form_request::validate_rules::basic;
    use ketzal::infrastructure::http::request::form_request::Validator;

    fn create_validator(data: HashMap<String, Value>) -> Validator {
        let rules = HashMap::new();
        Validator::make(data, rules)
    }

    #[test]
    fn test_required_rule_success() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Value::String("John".to_string()));
        let validator = create_validator(data);

        let result = basic::required::required("name", "name", Some(&Value::String("John".to_string())), &validator, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_required_rule_failure() {
        let validator = create_validator(HashMap::new());

        let result = basic::required::required("name", "name", None, &validator, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("is required"));
    }

    #[test]
    fn test_string_rule_success() {
        let validator = create_validator(HashMap::new());

        let result = basic::string_val::string("name", "name", Some(&Value::String("John".to_string())), &validator, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_string_rule_failure() {
        let validator = create_validator(HashMap::new());

        let result = basic::string_val::string("name", "name", Some(&Value::Number(123.into())), &validator, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_numeric_rule_success() {
        let validator = create_validator(HashMap::new());

        let result = basic::numeric_val::numeric("age", "age", Some(&Value::Number(25.into())), &validator, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_numeric_rule_failure() {
        let validator = create_validator(HashMap::new());

        let result = basic::numeric_val::numeric("age", "age", Some(&Value::String("not a number".to_string())), &validator, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_email_rule_success() {
        let validator = create_validator(HashMap::new());

        let result = basic::email_val::email("email", "email", Some(&Value::String("test@example.com".to_string())), &validator, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_email_rule_failure() {
        let validator = create_validator(HashMap::new());

        let result = basic::email_val::email("email", "email", Some(&Value::String("invalid-email".to_string())), &validator, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_min_rule_success() {
        let validator = create_validator(HashMap::new());

        let result = basic::min_val::min("name", "name", Some(&Value::String("John".to_string())), &validator, Some("3"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_rule_failure() {
        let validator = create_validator(HashMap::new());

        let result = basic::min_val::min("name", "name", Some(&Value::String("Hi".to_string())), &validator, Some("3"));
        assert!(result.is_err());
    }

    #[test]
    fn test_max_rule_success() {
        let validator = create_validator(HashMap::new());

        let result = basic::max_val::max("name", "name", Some(&Value::String("John".to_string())), &validator, Some("10"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_rule_failure() {
        let validator = create_validator(HashMap::new());

        let result = basic::max_val::max("name", "name", Some(&Value::String("VeryLongName".to_string())), &validator, Some("10"));
        assert!(result.is_err());
    }

    #[test]
    fn test_in_rule_success() {
        let validator = create_validator(HashMap::new());

        let result = basic::in_val::in_val("status", "status", Some(&Value::String("active".to_string())), &validator, Some("active,inactive"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_in_rule_failure() {
        let validator = create_validator(HashMap::new());

        let result = basic::in_val::in_val("status", "status", Some(&Value::String("pending".to_string())), &validator, Some("active,inactive"));
        assert!(result.is_err());
    }

    #[test]
    fn test_confirmed_rule_success() {
        let mut data = HashMap::new();
        data.insert("password".to_string(), Value::String("secret".to_string()));
        data.insert("password_confirmation".to_string(), Value::String("secret".to_string()));
        let validator = create_validator(data);

        let result = basic::confirmed_val::confirmed("password", "password", Some(&Value::String("secret".to_string())), &validator, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_confirmed_rule_failure() {
        let mut data = HashMap::new();
        data.insert("password".to_string(), Value::String("secret".to_string()));
        data.insert("password_confirmation".to_string(), Value::String("different".to_string()));
        let validator = create_validator(data);

        let result = basic::confirmed_val::confirmed("password", "password", Some(&Value::String("secret".to_string())), &validator, None);
        assert!(result.is_err());
    }
}