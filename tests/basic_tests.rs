#[cfg(test)]
mod tests {
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::Request;

    #[test]
    fn test_request_new() {
        let request = Request::new(Method::GET, "/test".to_string());

        assert_eq!(request.method, Method::GET);
        assert_eq!(request.path, "/test");
        assert!(request.headers.is_empty());
        assert!(request.body.is_empty());
        assert!(request.params.is_empty());
        assert!(request.query_params.is_empty());
        assert!(request.query_string.is_none());
    }

    #[test]
    fn test_request_param() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.params.insert("id".to_string(), "123".to_string());

        assert_eq!(request.param("id"), Some(&"123".to_string()));
        assert_eq!(request.param("nonexistent"), None);
    }

    #[test]
    fn test_request_query() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.query_params.insert("page".to_string(), "1".to_string());

        assert_eq!(request.query("page"), Some(&"1".to_string()));
        assert_eq!(request.query("nonexistent"), None);
    }

    #[test]
    fn test_request_query_or() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.query_params.insert("page".to_string(), "1".to_string());

        assert_eq!(request.query_or("page", "default"), "1");
        assert_eq!(request.query_or("nonexistent", "default"), "default");
    }

    #[test]
    fn test_request_body_as_string_success() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = b"Hello World".to_vec();

        let result = request.body_as_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World");
    }

    #[test]
    fn test_request_body_as_string_invalid_utf8() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = vec![0xff, 0xfe, 0xfd]; // Invalid UTF-8

        let result = request.body_as_string();
        assert!(result.is_err());
    }

    #[test]
    fn test_request_is_json() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());

        assert!(request.is_json());

        let mut request2 = Request::new(Method::POST, "/test".to_string());
        request2.headers.insert("Content-Type".to_string(), "text/html".to_string());

        assert!(!request2.is_json());
    }

    #[test]
    fn test_request_is_form() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.headers.insert("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string());

        assert!(request.is_form());

        let mut request2 = Request::new(Method::POST, "/test".to_string());
        request2.headers.insert("Content-Type".to_string(), "application/json".to_string());

        assert!(!request2.is_form());
    }

    #[test]
    fn test_request_full_url() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.query_string = Some("page=1&limit=10".to_string());

        assert_eq!(request.full_url(), "/test?page=1&limit=10");

        let request2 = Request::new(Method::GET, "/test".to_string());
        assert_eq!(request2.full_url(), "/test");
    }
}