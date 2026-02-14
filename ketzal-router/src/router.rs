use crate::route::{HandlerFuture, Route};
use http::Method;

#[derive(Clone)]
pub struct Router<Req, Res> {
    routes: Vec<Route<Req, Res>>,
}

impl<Req, Res> Router<Req, Res>
where
    Req: Send + 'static,
    Res: Send + 'static,
{
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn register(&mut self, route: Route<Req, Res>) {
        self.routes.push(route);
    }

    pub fn find(&self, method: &Method, path: &str) -> Option<&Route<Req, Res>> {
        self.routes
            .iter()
            .find(|r| &r.method == method && r.path == path)
    }

    pub fn handle(&self, method: Method, path: &str, request: Req) -> Option<HandlerFuture<Res>> {
        self.routes
            .iter()
            .find(|r| r.method == method && r.path == path)
            .map(|route| (route.handler)(request))
    }
}
