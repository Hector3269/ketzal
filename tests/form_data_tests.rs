#[cfg(test)]
mod tests {
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::Request;

    #[test]
    fn test_request_form_data_success() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = b"name=John+Doe&email=john%40example.com&age=25".to_vec();

        let result = request.form_data();
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("name"), Some(&"John Doe".to_string()));
        assert_eq!(data.get("email"), Some(&"john@example.com".to_string()));
        assert_eq!(data.get("age"), Some(&"25".to_string()));
    }

    #[test]
    fn test_request_form_data_empty() {
        let request = Request::new(Method::POST, "/test".to_string());

        let result = request.form_data();
        assert!(result.is_ok());

        let data = result.unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_request_form_data_invalid_utf8() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = vec![0xff, 0xfe, 0xfd]; // Invalid UTF-8

        let result = request.form_data();
        assert!(result.is_err());
    }

    #[test]
    fn test_request_form_data_special_characters() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = b"message=Hello%20World%21&symbol=%40%23%24".to_vec();

        let result = request.form_data();
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("message"), Some(&"Hello World!".to_string()));
        assert_eq!(data.get("symbol"), Some(&"@#$".to_string()));
    }

    #[test]
    fn test_request_form_data_multiple_values() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = b"hobby=reading&hobby=swimming&hobby=coding".to_vec();

        let result = request.form_data();
        assert!(result.is_ok());

        let data = result.unwrap();
        // Note: url::form_urlencoded only keeps the last value for duplicate keys
        assert_eq!(data.get("hobby"), Some(&"coding".to_string()));
    }

    #[test]
    fn test_request_form_data_caching() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = b"name=John".to_vec();

        // First call
        let result1 = request.form_data();
        assert!(result1.is_ok());

        // Modify body (should not affect cached result)
        request.body = b"name=Jane".to_vec();

        // Second call should return cached result
        let result2 = request.form_data();
        assert!(result2.is_ok());

        let data1 = result1.unwrap();
        let data2 = result2.unwrap();

        assert_eq!(data1.get("name"), Some(&"John".to_string()));
        assert_eq!(data2.get("name"), Some(&"John".to_string()));
    }
}