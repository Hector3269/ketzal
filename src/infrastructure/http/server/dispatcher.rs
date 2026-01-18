use std::collections::HashMap;

use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::routes::REGISTRY;

pub fn dispatch_request(request: &mut Request) -> Response {
    let registry = REGISTRY
        .lock()
        .expect("Route registry poisoned");

    for route in registry.get_routes() {
        if route.method == request.method {
            if let Some(params) = match_route(&route.path, &request.path) {
                request.params = params;
                return (route.handler)(request.clone());
            }
        }
    }
    Response::not_found()
}

fn match_route(route_path: &str, request_path: &str) -> Option<HashMap<String, String>> {
    let route_segments: Vec<&str> = route_path.split('/').filter(|s| !s.is_empty()).collect();
    let request_segments: Vec<&str> = request_path.split('/').filter(|s| !s.is_empty()).collect();

    if route_segments.len() != request_segments.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (route_seg, req_seg) in route_segments.iter().zip(request_segments.iter()) {
        if route_seg.starts_with('{') && route_seg.ends_with('}') {
            let param_name = &route_seg[1..route_seg.len() - 1];
            params.insert(param_name.to_string(), req_seg.to_string());
        } else if route_seg != req_seg {
            return None;
        }
    }
    Some(params)
}