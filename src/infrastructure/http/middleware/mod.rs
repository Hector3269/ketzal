use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use std::future::Future;
use std::pin::Pin;

pub type MiddlewareFuture = Pin<Box<dyn Future<Output = Response> + Send>>;
pub type Next = Box<dyn FnOnce(Request) -> MiddlewareFuture + Send>;

pub trait Middleware: Send + Sync {
    fn handle(&self, request: Request, next: Next) -> MiddlewareFuture;
}

pub mod cors;
pub mod logging;
