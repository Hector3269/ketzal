#[cfg(test)]
mod tests {
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::{Request, RequestBody};

    #[tokio::test]
    async fn test_parse_form_data_basic() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"name=John&email=john@example.com".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("name"), Some(&"John".to_string()));
        assert_eq!(data.get("email"), Some(&"john@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_parse_form_data_single_field() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"message=Hello%20World".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("message"), Some(&"Hello World".to_string()));
        assert_eq!(data.len(), 1);
    }

    #[tokio::test]
    async fn test_parse_form_data_empty_value() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"name=&email=test@example.com".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("name"), Some(&"".to_string()));
        assert_eq!(data.get("email"), Some(&"test@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_parse_form_data_special_characters() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"query=hello%20world%21&symbol=%40%23%24".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("query"), Some(&"hello world!".to_string()));
        assert_eq!(data.get("symbol"), Some(&"@#$".to_string()));
    }

    #[tokio::test]
    async fn test_parse_form_data_multiple_values_same_key() {
        // Note: url::form_urlencoded keeps only the last value for duplicate keys
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"item=first&item=second&item=third".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("item"), Some(&"third".to_string())); // Last value wins
        assert_eq!(data.len(), 1);
    }
}
