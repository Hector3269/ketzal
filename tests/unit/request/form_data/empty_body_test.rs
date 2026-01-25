#[cfg(test)]
mod tests {
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::{Request, RequestBody};

    #[tokio::test]
    async fn test_empty_body_returns_empty_map() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(Vec::new());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert!(data.is_empty());
    }

    #[tokio::test]
    async fn test_whitespace_only_body() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"   \n\t  ".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_empty_form_fields() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"field1=&field2=&field3=".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.len(), 3);
        assert_eq!(data.get("field1"), Some(&"".to_string()));
        assert_eq!(data.get("field2"), Some(&"".to_string()));
        assert_eq!(data.get("field3"), Some(&"".to_string()));
    }

    #[tokio::test]
    async fn test_only_equals_sign() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"=".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert!(data.is_empty() || data.contains_key(""));
    }

    #[tokio::test]
    async fn test_empty_key_with_value() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"=value".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get(""), Some(&"value".to_string()));
    }
}
