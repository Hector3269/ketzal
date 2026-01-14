#[cfg(test)]
mod tests {
    use serde_json::Value;
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::Request;

    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct TestUser {
        name: String,
        age: u32,
        active: bool,
    }

    #[test]
    fn test_request_json_success() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = r#"{"name": "John", "age": 25, "active": true}"#.as_bytes().to_vec();

        let result: Result<TestUser, _> = request.json();
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.name, "John");
        assert_eq!(user.age, 25);
        assert_eq!(user.active, true);
    }

    #[test]
    fn test_request_json_invalid_json() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = b"invalid json".to_vec();

        let result: Result<TestUser, _> = request.json();
        assert!(result.is_err());
    }

    #[test]
    fn test_request_json_value_success() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = r#"{"name": "John", "numbers": [1, 2, 3]}"#.as_bytes().to_vec();

        let result = request.json_value();
        assert!(result.is_ok());

        let value = result.unwrap();
        if let Value::Object(map) = value {
            assert_eq!(map.get("name"), Some(&Value::String("John".to_string())));
            assert_eq!(map.get("numbers"), Some(&Value::Array(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
                Value::Number(3.into()),
            ])));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_request_json_value_invalid_json() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = b"not json".to_vec();

        let result = request.json_value();
        assert!(result.is_err());
    }

    #[test]
    fn test_request_json_value_caching() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = r#"{"name": "John"}"#.as_bytes().to_vec();

        // First call
        let result1 = request.json_value();
        assert!(result1.is_ok());

        // Modify body (should not affect cached result)
        request.body = r#"{"name": "Jane"}"#.as_bytes().to_vec();

        // Second call should return cached result
        let result2 = request.json_value();
        assert!(result2.is_ok());

        let value1 = result1.unwrap();
        let value2 = result2.unwrap();

        if let (Value::Object(map1), Value::Object(map2)) = (value1, value2) {
            assert_eq!(map1.get("name"), Some(&Value::String("John".to_string())));
            assert_eq!(map2.get("name"), Some(&Value::String("John".to_string())));
        } else {
            panic!("Expected objects");
        }
    }

    #[test]
    fn test_request_json_empty_body() {
        let request = Request::new(Method::POST, "/test".to_string());

        let result: Result<TestUser, _> = request.json();
        assert!(result.is_err());
    }

    #[test]
    fn test_request_json_partial_data() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = r#"{"name": "John"}"#.as_bytes().to_vec();

        let result: Result<TestUser, _> = request.json();
        assert!(result.is_err()); // Missing required fields
    }
}