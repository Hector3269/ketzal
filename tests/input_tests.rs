#[cfg(test)]
mod tests {
    use serde_json::Value;
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::Request;

    #[test]
    fn test_request_input_from_query() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.query_params.insert("name".to_string(), "John".to_string());

        assert_eq!(request.input("name"), Some("John".to_string()));
        assert_eq!(request.input("nonexistent"), None);
    }

    #[test]
    fn test_request_input_from_json() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());
        request.body = r#"{"name": "John", "age": 25}"#.as_bytes().to_vec();

        assert_eq!(request.input("name"), Some("John".to_string()));
        assert_eq!(request.input("age"), Some("25".to_string()));
        assert_eq!(request.input("nonexistent"), None);
    }

    #[test]
    fn test_request_input_from_form() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.headers.insert("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string());
        request.body = b"name=John&age=25".to_vec();

        assert_eq!(request.input("name"), Some("John".to_string()));
        assert_eq!(request.input("age"), Some("25".to_string()));
        assert_eq!(request.input("nonexistent"), None);
    }

    #[test]
    fn test_request_input_or() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.query_params.insert("name".to_string(), "John".to_string());

        assert_eq!(request.input_or("name", "default"), "John");
        assert_eq!(request.input_or("nonexistent", "default"), "default");
    }

    #[test]
    fn test_request_all_inputs() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.query_params.insert("page".to_string(), "1".to_string());
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());
        request.body = r#"{"name": "John", "active": true}"#.as_bytes().to_vec();

        let inputs = request.all_inputs();

        assert_eq!(inputs.get("page"), Some(&Value::String("1".to_string())));
        assert_eq!(inputs.get("name"), Some(&Value::String("John".to_string())));
        assert_eq!(inputs.get("active"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_request_only() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.query_params.insert("page".to_string(), "1".to_string());
        request.query_params.insert("limit".to_string(), "10".to_string());
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());
        request.body = r#"{"name": "John", "age": 25}"#.as_bytes().to_vec();

        let inputs = request.only(&["name", "page"]);

        assert_eq!(inputs.len(), 2);
        assert!(inputs.contains_key("name"));
        assert!(inputs.contains_key("page"));
        assert!(!inputs.contains_key("age"));
        assert!(!inputs.contains_key("limit"));
    }

    #[test]
    fn test_request_except() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.query_params.insert("page".to_string(), "1".to_string());
        request.query_params.insert("limit".to_string(), "10".to_string());
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());
        request.body = r#"{"name": "John", "age": 25}"#.as_bytes().to_vec();

        let inputs = request.except(&["limit", "age"]);

        assert_eq!(inputs.len(), 2);
        assert!(inputs.contains_key("page"));
        assert!(inputs.contains_key("name"));
        assert!(!inputs.contains_key("age"));
        assert!(!inputs.contains_key("limit"));
    }
}