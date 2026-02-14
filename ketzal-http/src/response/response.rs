use http::{HeaderMap, HeaderValue, StatusCode};

#[derive(Clone, Debug)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            body: Vec::new(),
        }
    }

    pub fn with_body(status: StatusCode, body: impl Into<Vec<u8>>) -> Self {
        let mut res = Self::new(status);
        res.body = body.into();
        res
    }

    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self::with_body(StatusCode::OK, body)
    }

    pub fn not_found() -> Self {
        Self::with_body(StatusCode::NOT_FOUND, b"Not Found".to_vec())
    }

    pub fn bad_request(body: impl Into<Vec<u8>>) -> Self {
        Self::with_body(StatusCode::BAD_REQUEST, body)
    }

    pub fn header(mut self, key: &'static str, value: &'static str) -> Self {
        self.headers.insert(key, HeaderValue::from_static(value));
        self
    }
    pub fn to_http_string(&self) -> String {
        let reason = self.status.canonical_reason().unwrap_or("Unknown");

        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status.as_u16(), reason);

        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));

        for (key, value) in self.headers.iter() {
            if let Ok(value_str) = value.to_str() {
                response.push_str(&format!("{}: {}\r\n", key.as_str(), value_str));
            }
        }

        response.push_str("\r\n");

        response.push_str(&String::from_utf8_lossy(&self.body));

        response
    }

    /// Convert response to HTTP bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_http_string().into_bytes()
    }
}
