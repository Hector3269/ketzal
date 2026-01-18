use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::method::Method;
use std::sync::Arc;

pub type Handler = Arc<dyn Fn(Request) -> Response + Send + Sync>;

#[derive(Clone)]
pub struct RouteData {
    pub method: Method,
    pub path: String,
    pub handler: Handler,
}

impl RouteData {
    pub fn new(method: Method, path: String, handler: Handler) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
