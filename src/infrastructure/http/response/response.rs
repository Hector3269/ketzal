use flate2::{write::GzEncoder, Compression};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;
use crate::kernel::constants::{
    headers::{CONTENT_TYPE, CONTENT_LENGTH, CONTENT_ENCODING, LOCATION, CACHE_CONTROL, CONNECTION, GZIP_ENCODING, NO_CACHE, KEEP_ALIVE},
    content_types::{TEXT_HTML, APPLICATION_JSON, TEXT_EVENT_STREAM},
    limits::GZIP_MIN_SIZE_BYTES,
    protocol::{HTTP_VERSION_1_1, CRLF, HEADER_SEPARATOR},
    status_code::reason_phrase
};

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(status: u16, body: String) -> Self {
        let mut headers = HashMap::new();
        headers.insert(
            CONTENT_TYPE.to_string(),
            TEXT_HTML.to_string(),
        );

        Self {
            status,
            headers,
            body,
        }
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn json(data: Value) -> Self {
        let mut response = Self::new(200, data.to_string());
        response
            .headers
            .insert(CONTENT_TYPE.to_string(), APPLICATION_JSON.to_string());
        response
    }

    pub fn created(data: Value) -> Self {
        let mut response = Self::new(201, data.to_string());
        response
            .headers
            .insert(CONTENT_TYPE.to_string(), APPLICATION_JSON.to_string());
        response
    }

    pub fn validation_error(errors: HashMap<String, Vec<String>>) -> Self {
        let json = serde_json::json!({
            "message": "Validation failed",
            "errors": errors
        });
        let mut response = Self::new(422, json.to_string());
        response
            .headers
            .insert(CONTENT_TYPE.to_string(), APPLICATION_JSON.to_string());
        response
    }

    pub fn to_http_string(&self) -> String {
        let status_text = reason_phrase(self.status);
        let mut response = format!("{} {} {}{}", HTTP_VERSION_1_1, self.status, status_text, CRLF);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}{}{}{}", key, HEADER_SEPARATOR, value, CRLF));
        }

        response.push_str(&format!("{}{}{}{}", CONTENT_LENGTH, HEADER_SEPARATOR, self.body.len(), CRLF));
        response.push_str(CRLF);
        response.push_str(&self.body);

        response
    }

    pub fn compress_gzip(mut self) -> Self {
        if self.body.len() > GZIP_MIN_SIZE_BYTES {
            // Only compress if body > threshold
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(self.body.as_bytes()).unwrap();
            let compressed = encoder.finish().unwrap();
            self.body = String::from_utf8_lossy(&compressed).to_string();
            self.headers
                .insert(CONTENT_ENCODING.to_string(), GZIP_ENCODING.to_string());
        }
        self
    }


    pub fn ok_text(body: String) -> Self {
        Self::new(200, body)
    }

    pub fn not_found() -> Self {
        Self::new(404, "Not Found".to_string())
    }

    pub fn unauthorized(message: Option<String>) -> Self {
        Self::new(401, message.unwrap_or_else(|| "Unauthorized".to_string()))
    }

    pub fn forbidden(message: Option<String>) -> Self {
        Self::new(403, message.unwrap_or_else(|| "Forbidden".to_string()))
    }

    pub fn bad_request(message: String) -> Self {
        Self::new(400, message)
    }

    pub fn internal_error(message: String) -> Self {
        Self::new(500, message)
    }

    pub fn method_not_allowed() -> Self {
        Self::new(405, "Method Not Allowed".to_string())
    }

    pub fn internal_server_error() -> Self {
        Self::new(500, "Internal Server Error".to_string())
    }

    pub fn unprocessable_entity(body: String) -> Self {
        let mut response = Self::new(422, body);
        response
            .headers
            .insert(CONTENT_TYPE.to_string(), APPLICATION_JSON.to_string());
        response
    }

    pub fn custom(status: u16, body: String) -> Self {
        Self::new(status, body)
    }

    pub fn internal_server_error_json(data: Value) -> Self {
        let mut response = Self::new(500, data.to_string());
        response
            .headers
            .insert(CONTENT_TYPE.to_string(), APPLICATION_JSON.to_string());
        response
    }

    pub fn redirect(status: u16, location: &str) -> Self {
        let mut response = Self::new(status, "".to_string());
        response
            .headers
            .insert(LOCATION.to_string(), location.to_string());
        response
    }

    pub fn sse() -> Self {
        let mut response = Self::new(200, "".to_string());
        response
            .headers
            .insert(CONTENT_TYPE.to_string(), TEXT_EVENT_STREAM.to_string());
        response
            .headers
            .insert(CACHE_CONTROL.to_string(), NO_CACHE.to_string());
        response
            .headers
            .insert(CONNECTION.to_string(), KEEP_ALIVE.to_string());
        response
    }
}
