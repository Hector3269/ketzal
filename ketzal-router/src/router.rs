use http::Method;
use std::collections::HashMap;

use crate::route::Route;

pub struct Router<Req, Res> {
    routes: Vec<Route<Req, Res>>,
    named_routes: HashMap<String, String>,
}

impl<Req, Res> Router<Req, Res>
where
    Req: Send + 'static,
    Res: Send + 'static,
{
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            named_routes: HashMap::new(),
        }
    }

    pub fn register(&mut self, route: Route<Req, Res>) {
        if let Some(name) = &route.name {
            self.named_routes.insert(name.clone(), route.path.clone());
        }
        self.routes.push(route);
    }

    pub fn url(&self, name: &str) -> Option<&String> {
        self.named_routes.get(name)
    }

    pub fn find(&self, method: &Method, path: &str) -> Option<&Route<Req, Res>> {
        self.routes
            .iter()
            .find(|r| &r.method == method && r.path == path)
    }
}
