#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS",
            Method::HEAD => "HEAD",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "PATCH" => Some(Method::PATCH),
            "DELETE" => Some(Method::DELETE),
            "OPTIONS" => Some(Method::OPTIONS),
            "HEAD" => Some(Method::HEAD),
            _ => None,
        }
    }
}
