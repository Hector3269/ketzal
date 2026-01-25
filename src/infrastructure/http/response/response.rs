use flate2::{write::GzEncoder, Compression};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;

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
            "Content-Type".to_string(),
            "text/html; charset=utf-8".to_string(),
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
            .insert("Content-Type".to_string(), "application/json".to_string());
        response
    }

    pub fn created(data: Value) -> Self {
        let mut response = Self::new(201, data.to_string());
        response
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
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
            .insert("Content-Type".to_string(), "application/json".to_string());
        response
    }

    pub fn to_http_string(&self) -> String {
        let status_text = Self::status_text(self.status);
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status, status_text);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        response.push_str("\r\n");
        response.push_str(&self.body);

        response
    }

    pub fn compress_gzip(mut self) -> Self {
        if self.body.len() > 1024 {
            // Only compress if body > 1KB
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(self.body.as_bytes()).unwrap();
            let compressed = encoder.finish().unwrap();
            self.body = String::from_utf8_lossy(&compressed).to_string();
            self.headers
                .insert("Content-Encoding".to_string(), "gzip".to_string());
        }
        self
    }

    pub fn status_text(code: u16) -> &'static str {
        match code {
            // 1xx Informational
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",
            103 => "Early Hints",
            // 2xx Success
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            208 => "Already Reported",
            226 => "IM Used",
            // 3xx Redirection
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            305 => "Use Proxy",
            306 => "(Unused)",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            // 4xx Client Error
            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Range Not Satisfiable",
            417 => "Expectation Failed",
            418 => "I'm a teapot",
            421 => "Misdirected Request",
            422 => "Unprocessable Entity",
            423 => "Locked",
            424 => "Failed Dependency",
            425 => "Too Early",
            426 => "Upgrade Required",
            428 => "Precondition Required",
            429 => "Too Many Requests",
            431 => "Request Header Fields Too Large",
            451 => "Unavailable For Legal Reasons",
            // 5xx Server Error
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            506 => "Variant Also Negotiates",
            507 => "Insufficient Storage",
            508 => "Loop Detected",
            510 => "Not Extended",
            511 => "Network Authentication Required",
            _ => "Unknown Status",
        }
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
            .insert("Content-Type".to_string(), "application/json".to_string());
        response
    }

    pub fn custom(status: u16, body: String) -> Self {
        Self::new(status, body)
    }

    pub fn internal_server_error_json(data: Value) -> Self {
        let mut response = Self::new(500, data.to_string());
        response
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        response
    }

    pub fn redirect(status: u16, location: &str) -> Self {
        let mut response = Self::new(status, "".to_string());
        response
            .headers
            .insert("Location".to_string(), location.to_string());
        response
    }

    pub fn sse() -> Self {
        let mut response = Self::new(200, "".to_string());
        response
            .headers
            .insert("Content-Type".to_string(), "text/event-stream".to_string());
        response
            .headers
            .insert("Cache-Control".to_string(), "no-cache".to_string());
        response
            .headers
            .insert("Connection".to_string(), "keep-alive".to_string());
        response
    }
}
