use super::route_data::{Handler, RouteData};
use crate::infrastructure::http::method::Method;
use crate::infrastructure::http::middleware::Middleware;
use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use std::future::Future;
use std::sync::Arc;

pub struct Route {
    method: Method,
    path: String,
    handler: Handler,
    middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
}

impl Route {
    pub fn get<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        Self {
            method: Method::GET,
            path: path.to_string(),
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        }
    }

    pub fn post<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        Self {
            method: Method::POST,
            path: path.to_string(),
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        }
    }

    pub fn put<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        Self {
            method: Method::PUT,
            path: path.to_string(),
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        }
    }

    pub fn delete<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        Self {
            method: Method::DELETE,
            path: path.to_string(),
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        }
    }

    pub fn patch<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        Self {
            method: Method::PATCH,
            path: path.to_string(),
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        }
    }

    pub fn options<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        Self {
            method: Method::OPTIONS,
            path: path.to_string(),
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        }
    }

    pub fn head<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        Self {
            method: Method::HEAD,
            path: path.to_string(),
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        }
    }

    pub fn middleware(mut self, middleware: Arc<dyn Middleware + Send + Sync>) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn middlewares(mut self, middlewares: Vec<Arc<dyn Middleware + Send + Sync>>) -> Self {
        self.middlewares.extend(middlewares);
        self
    }

    pub fn into_data(self) -> RouteData {
        let mut data = RouteData::new(self.method, self.path, self.handler);
        data.middlewares = self.middlewares;
        data
    }
}
