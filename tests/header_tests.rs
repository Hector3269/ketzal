#[cfg(test)]
mod tests {
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::Request;

    #[test]
    fn test_request_header() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());
        request.headers.insert("Authorization".to_string(), "Bearer token123".to_string());

        assert_eq!(request.header("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(request.header("Authorization"), Some(&"Bearer token123".to_string()));
        assert_eq!(request.header("Nonexistent"), None);
    }

    #[test]
    fn test_request_header_case_insensitive() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());

        assert_eq!(request.header("content-type"), Some(&"application/json".to_string()));
        assert_eq!(request.header("CONTENT-TYPE"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_request_bearer_token_success() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.headers.insert("Authorization".to_string(), "Bearer abc123def456".to_string());

        let token = request.bearer_token();
        assert_eq!(token, Some("abc123def456".to_string()));
    }

    #[test]
    fn test_request_bearer_token_no_header() {
        let request = Request::new(Method::GET, "/test".to_string());

        let token = request.bearer_token();
        assert_eq!(token, None);
    }

    #[test]
    fn test_request_bearer_token_invalid_format() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.headers.insert("Authorization".to_string(), "Basic dXNlcjpwYXNz".to_string());

        let token = request.bearer_token();
        assert_eq!(token, None);
    }

    #[test]
    fn test_request_bearer_token_case_insensitive() {
        let mut request = Request::new(Method::GET, "/test".to_string());
        request.headers.insert("authorization".to_string(), "Bearer token123".to_string());

        let token = request.bearer_token();
        assert_eq!(token, Some("token123".to_string()));
    }
}