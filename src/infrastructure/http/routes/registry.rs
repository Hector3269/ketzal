use super::route_data::{Handler, RouteData};
use super::trie::RouteNode;
use crate::infrastructure::http::method::Method;
use crate::infrastructure::http::middleware::Middleware;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct RouteRegistry {
    root: RouteNode,
    cache: Mutex<
        HashMap<
            (Method, String),
            (
                Handler,
                Vec<Arc<dyn Middleware + Send + Sync>>,
                HashMap<String, String>,
            ),
        >,
    >,
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            root: RouteNode::new(),
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&mut self, route: RouteData) {
        self.root
            .insert(&route.path, route.method, route.handler, route.middlewares);
        // Clear cache when new routes are registered to avoid stale matches
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    pub fn match_route(
        &self,
        path: &str,
        method: Method,
    ) -> Option<(
        Handler,
        Vec<Arc<dyn Middleware + Send + Sync>>,
        HashMap<String, String>,
    )> {
        // Try cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some(cached) = cache.get(&(method, path.to_string())) {
                return Some(cached.clone());
            }
        }

        // Match using Trie
        if let Some(((handler, middlewares), params)) = self.root.find(path, method) {
            // Update cache
            if let Ok(mut cache) = self.cache.lock() {
                cache.insert(
                    (method, path.to_string()),
                    (handler.clone(), middlewares.clone(), params.clone()),
                );
            }
            return Some((handler, middlewares, params));
        }

        None
    }
}
