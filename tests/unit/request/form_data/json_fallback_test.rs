#[cfg(test)]
mod tests {
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::{Request, RequestBody};

    #[tokio::test]
    async fn test_json_request_form_data_does_not_crash() {
        let mut request = Request::new(Method::POST, "/api".to_string());
        request.body = RequestBody::Buffered(
            r#"{"name": "John", "email": "john@example.com"}"#.as_bytes().to_vec(),
        );
        request
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());

        let result = request.form_data().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mixed_content_type_json() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(r#"{"key": "value"}"#.as_bytes().to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/json; charset=utf-8".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_content_as_form_data() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"invalid content { }".to_vec());
        request
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());

        let result = request.form_data().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_content_with_equals_as_form_data() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"some=content&other=value".to_vec());
        request
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());

        let result = request.form_data().await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.get("some"), Some(&"content".to_string()));
        assert_eq!(data.get("other"), Some(&"value".to_string()));
    }
}
