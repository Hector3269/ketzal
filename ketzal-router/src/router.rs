//! Router module
//!
//! Provides the [`Router`] struct for managing and dispatching routes.

use crate::handler::HandlerFuture;
use crate::params::{match_path, Params};
use crate::route::Route;
use http::Method;
use ketzal_http::Request;

/// The main router struct that holds all registered routes.
///
/// # Example
///
/// ```ignore
/// use ketzal_router::{Router, Route};
/// use ketzal_http::{Request, Response};
///
/// async fn hello() -> Response {
///     Response::ok("Hello!")
/// }
///
/// let mut router = Router::new();
/// router.register(Route::get("/", hello));
///
/// // Handle a request
/// let req = Request::new(http::Method::GET, "/", None);
/// if let Some(future) = router.handle(&http::Method::GET, "/", req) {
///     // Execute handler
/// }
/// ```
#[derive(Clone, Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /// Creates a new empty Router.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let router = Router::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a route to the router.
    ///
    /// # Arguments
    ///
    /// * `route` - The [`Route`] to register
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut router = Router::new();
    /// router.register(Route::get("/", handler));
    /// ```
    pub fn register(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Handles an incoming request by matching it against registered routes.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method
    /// * `path` - The request path
    /// * `req` - The incoming request
    ///
    /// # Returns
    ///
    /// Returns `Some(HandlerFuture)` if a matching route is found,
    /// or `None` if no route matches.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let req = Request::new(Method::GET, "/users/42", None);
    /// if let Some(handler) = router.handle(&Method::GET, "/users/42", req) {
    ///     // Execute handler
    /// }
    /// ```
    pub fn handle(&self, method: &Method, path: &str, req: Request) -> Option<HandlerFuture> {
        for route in &self.routes {
            if route.method != method {
                continue;
            }
            if route.path == path {
                return Some(route.call(&Params::new(), Some(req)));
            }
            if let Some(params) = match_path(&route.path, path) {
                return Some(route.call(&params, Some(req)));
            }
        }
        None
    }
}
