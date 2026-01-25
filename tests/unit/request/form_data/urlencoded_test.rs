#[cfg(test)]
mod tests {
    use ketzal::infrastructure::http::method::Method;
    use ketzal::infrastructure::http::request::request::{Request, RequestBody};

    #[tokio::test]
    async fn test_urlencoded_basic_parsing() {
        let mut request = Request::new(Method::POST, "/submit".to_string());
        request.body =
            RequestBody::Buffered(b"username=john_doe&password=secret123&remember=true".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("username"), Some(&"john_doe".to_string()));
        assert_eq!(data.get("password"), Some(&"secret123".to_string()));
        assert_eq!(data.get("remember"), Some(&"true".to_string()));
    }

    #[tokio::test]
    async fn test_urlencoded_with_spaces_and_special_chars() {
        let mut request = Request::new(Method::POST, "/contact".to_string());
        request.body = RequestBody::Buffered(
            b"name=John%20Doe&email=john.doe%40example.com&message=Hello%2C%20world%21".to_vec(),
        );
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("name"), Some(&"John Doe".to_string()));
        assert_eq!(data.get("email"), Some(&"john.doe@example.com".to_string()));
        assert_eq!(data.get("message"), Some(&"Hello, world!".to_string()));
    }

    #[tokio::test]
    async fn test_urlencoded_empty_fields() {
        let mut request = Request::new(Method::POST, "/form".to_string());
        request.body =
            RequestBody::Buffered(b"title=&description=Some%20description&tags=".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("title"), Some(&"".to_string()));
        assert_eq!(
            data.get("description"),
            Some(&"Some description".to_string())
        );
        assert_eq!(data.get("tags"), Some(&"".to_string()));
    }

    #[tokio::test]
    async fn test_urlencoded_numeric_values() {
        let mut request = Request::new(Method::POST, "/update".to_string());
        request.body = RequestBody::Buffered(b"id=123&count=0&price=19.99&active=1".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        let result = request.form_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.get("id"), Some(&"123".to_string()));
        assert_eq!(data.get("count"), Some(&"0".to_string()));
        assert_eq!(data.get("price"), Some(&"19.99".to_string()));
        assert_eq!(data.get("active"), Some(&"1".to_string()));
    }

    #[tokio::test]
    async fn test_urlencoded_caching() {
        let mut request = Request::new(Method::POST, "/test".to_string());
        request.body = RequestBody::Buffered(b"key=value".to_vec());
        request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        // First call
        let result1 = request.form_data().await;
        assert!(result1.is_ok());

        // Second call should use cache
        let result2 = request.form_data().await;
        assert!(result2.is_ok());

        assert_eq!(result1.unwrap(), result2.unwrap());
    }
}
