use crate::infrastructure::http::method::Method;
use crate::infrastructure::http::middleware::Middleware;
use crate::infrastructure::http::routes::route_data::Handler;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RouteNode {
    pub static_children: HashMap<String, RouteNode>,
    pub dynamic_child: Option<Box<DynamicNode>>,
    pub handlers: HashMap<Method, (Handler, Vec<Arc<dyn Middleware + Send + Sync>>)>,
}

pub struct DynamicNode {
    pub name: String,
    pub node: RouteNode,
}

impl RouteNode {
    pub fn new() -> Self {
        Self {
            static_children: HashMap::new(),
            dynamic_child: None,
            handlers: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        path: &str,
        method: Method,
        handler: Handler,
        middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
    ) {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        self.insert_recursive(&segments, method, handler, middlewares);
    }

    fn insert_recursive(
        &mut self,
        segments: &[&str],
        method: Method,
        handler: Handler,
        middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
    ) {
        if segments.is_empty() {
            self.handlers.insert(method, (handler, middlewares));
            return;
        }

        let first = segments[0];
        if first.starts_with('{') && first.ends_with('}') {
            let name = &first[1..first.len() - 1];
            if self.dynamic_child.is_none() {
                self.dynamic_child = Some(Box::new(DynamicNode {
                    name: name.to_string(),
                    node: RouteNode::new(),
                }));
            }
            self.dynamic_child.as_mut().unwrap().node.insert_recursive(
                &segments[1..],
                method,
                handler,
                middlewares,
            );
        } else {
            let child = self
                .static_children
                .entry(first.to_string())
                .or_insert_with(RouteNode::new);
            child.insert_recursive(&segments[1..], method, handler, middlewares);
        }
    }

    pub fn find(
        &self,
        path: &str,
        method: Method,
    ) -> Option<(
        (Handler, Vec<Arc<dyn Middleware + Send + Sync>>),
        HashMap<String, String>,
    )> {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut params = HashMap::new();
        let result = self.find_recursive(&segments, method, &mut params)?;
        Some((result, params))
    }

    fn find_recursive(
        &self,
        segments: &[&str],
        method: Method,
        params: &mut HashMap<String, String>,
    ) -> Option<(Handler, Vec<Arc<dyn Middleware + Send + Sync>>)> {
        if segments.is_empty() {
            return self.handlers.get(&method).cloned();
        }

        let first = segments[0];

        // Try static match first
        if let Some(child) = self.static_children.get(first) {
            if let Some(result) = child.find_recursive(&segments[1..], method, params) {
                return Some(result);
            }
        }

        // Try dynamic match
        if let Some(dynamic) = &self.dynamic_child {
            params.insert(dynamic.name.clone(), first.to_string());
            if let Some(handler) = dynamic.node.find_recursive(&segments[1..], method, params) {
                return Some(handler);
            }
            // Backtrack params if dynamic match failed
            params.remove(&dynamic.name);
        }

        None
    }
}
