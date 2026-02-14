use ketzal_http::{Request, Response};
use ketzal_router::{Route, Router};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref ROUTES: Mutex<Vec<Route<Request, Response>>> =
        Mutex::new(Vec::new());

    static ref ROUTER_CACHE: Mutex<Option<Arc<Router<Request, Response>>>> =
        Mutex::new(None);
}

pub fn init_router() {
    let router = build_router_internal();
    let mut cache = ROUTER_CACHE.lock().unwrap();
    *cache = Some(Arc::new(router));
}


pub fn get_router() -> Arc<Router<Request, Response>> {
    let mut cache = ROUTER_CACHE.lock().unwrap();
    if cache.is_none() {
        let router = build_router_internal();
        *cache = Some(Arc::new(router));
    }
    cache.as_ref().unwrap().clone()
}

pub fn register(route: Route<Request, Response>) {
    ROUTES.lock().unwrap().push(route);
    invalidate_router_cache();
}

fn invalidate_router_cache() {
    let mut cache = ROUTER_CACHE.lock().unwrap();
    *cache = None;
}

fn build_router_internal() -> Router<Request, Response> {
    let mut router = Router::new();
    let routes = ROUTES.lock().unwrap();

    for route in routes.iter() {
        router.register(route.clone());
    }

    router
}
