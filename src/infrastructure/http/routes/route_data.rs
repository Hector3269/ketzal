use crate::infrastructure::http::method::Method;
use crate::infrastructure::http::middleware::Middleware;
use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type HandlerFuture = Pin<Box<dyn Future<Output = Response> + Send>>;
pub type Handler = Arc<dyn Fn(Request) -> HandlerFuture + Send + Sync>;

#[derive(Clone)]
pub struct RouteData {
    pub method: Method,
    pub path: String,
    pub handler: Handler,
    pub middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
}

impl RouteData {
    pub fn new(method: Method, path: String, handler: Handler) -> Self {
        Self {
            method,
            path,
            handler,
            middlewares: Vec::new(),
        }
    }
}
