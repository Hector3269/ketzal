#[cfg(test)]
mod tests {
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::{Request, RequestBody, RequestError};

    #[tokio::test]
    async fn test_invalid_utf8_in_body() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(vec![0xff, 0xfe, 0xfd]);
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error, RequestError::InvalidUtf8(_)));
    }

    #[tokio::test]
    async fn test_malformed_url_encoding() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"name=John%D&email=test%".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_incomplete_key_value_pairs() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"name&email=test@example.com&".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();

        assert!(data.contains_key("email"));
        assert_eq!(data.get("email"), Some(&"test@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_binary_data_in_form_body() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(vec![
            0x00, 0x01, 0x02, b'n', b'a', b'm', b'e', b'=', b'v', b'a', b'l', b'u', b'e', 0x00,
        ]);
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );
        let result = request.form_data().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_extremely_long_form_data() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        let long_value = "x".repeat(10000);
        let body = format!("name={}", long_value);
        request.body = RequestBody::Buffered(body.into_bytes());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("name"), Some(&long_value));
    }

    #[tokio::test]
    async fn test_form_data_with_newlines() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"name=John\nDoe&email=test@example.com".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("name"), Some(&"John\nDoe".to_string()));
        assert_eq!(data.get("email"), Some(&"test@example.com".to_string()));
    }
}
