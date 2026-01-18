use super::route_data::{Handler, RouteData};
use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::method::Method;
use std::sync::Arc;

pub struct Route {
    method: Method,
    path: String,
    handler: Handler,
}

impl Route {
    pub fn get<F>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        Self {
            method: Method::GET,
            path: path.to_string(),
            handler: Arc::new(handler),
        }
    }

    pub fn post<F>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        Self {
            method: Method::POST,
            path: path.to_string(),
            handler: Arc::new(handler),
        }
    }

    pub fn put<F>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        Self {
            method: Method::PUT,
            path: path.to_string(),
            handler: Arc::new(handler),
        }
    }

    pub fn delete<F>(path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        Self {
            method: Method::DELETE,
            path: path.to_string(),
            handler: Arc::new(handler),
        }
    }

    pub fn into_data(self) -> RouteData {
        RouteData::new(self.method, self.path, self.handler)
    }
}
