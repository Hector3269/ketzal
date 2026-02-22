use crate::{Route, Router};
use std::sync::{Arc, OnceLock, RwLock};

static ROUTES_WEB: OnceLock<RwLock<Vec<Route>>> = OnceLock::new();
static ROUTES_API: OnceLock<RwLock<Vec<Route>>> = OnceLock::new();

static ROUTER_WEB: OnceLock<RwLock<Option<Arc<Router>>>> = OnceLock::new();
static ROUTER_API: OnceLock<RwLock<Option<Arc<Router>>>> = OnceLock::new();

fn web_routes() -> &'static RwLock<Vec<Route>> {
    ROUTES_WEB.get_or_init(|| RwLock::new(Vec::new()))
}

fn api_routes() -> &'static RwLock<Vec<Route>> {
    ROUTES_API.get_or_init(|| RwLock::new(Vec::new()))
}

fn web_cache() -> &'static RwLock<Option<Arc<Router>>> {
    ROUTER_WEB.get_or_init(|| RwLock::new(None))
}

fn api_cache() -> &'static RwLock<Option<Arc<Router>>> {
    ROUTER_API.get_or_init(|| RwLock::new(None))
}

pub fn register_web(route: Route) {
    web_routes().write().unwrap().push(route);
    *web_cache().write().unwrap() = None;
}

pub fn register_api(route: Route) {
    api_routes().write().unwrap().push(route);
    *api_cache().write().unwrap() = None;
}

pub fn get_web_router() -> Arc<Router> {
    if let Some(router) = web_cache().read().unwrap().as_ref() {
        return router.clone();
    }
    let mut cache = web_cache().write().unwrap();
    if cache.is_none() {
        *cache = Some(Arc::new(build(web_routes())));
    }
    cache.as_ref().unwrap().clone()
}

pub fn get_api_router() -> Arc<Router> {
    if let Some(router) = api_cache().read().unwrap().as_ref() {
        return router.clone();
    }
    let mut cache = api_cache().write().unwrap();
    if cache.is_none() {
        *cache = Some(Arc::new(build(api_routes())));
    }
    cache.as_ref().unwrap().clone()
}

fn build(routes: &RwLock<Vec<Route>>) -> Router {
    let mut router = Router::new();
    for route in routes.read().unwrap().iter() {
        router.register(route.clone());
    }
    router
}
