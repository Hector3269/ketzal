//! Route module
//!
//! Provides the [`Route`] struct for defining HTTP routes.

use crate::handler::{into_boxed, BoxedHandler, Handler, HandlerFuture};
use crate::params::Params;
use http::Method;
use ketzal_http::Request;
use std::sync::Arc;

/// Represents a single HTTP route with a handler.
///
/// A route consists of:
/// - HTTP method (GET, POST, PUT, DELETE, PATCH)
/// - Path pattern (with optional `:param` syntax for dynamic segments)
/// - Handler function
/// - Optional name for identification
///
/// # Example
///
/// ```ignore
/// use ketzal_router::Route;
/// use ketzal_http::Response;
///
/// async fn hello() -> Response {
///     Response::ok("Hello!")
/// }
///
/// // Simple route
/// let route = Route::get("/", hello);
///
/// // Route with parameter
/// async fn greet(name: String) -> Response {
///     Response::ok(format!("Hello, {}!", name))
/// }
/// let route = Route::get("/hello/:name", greet);
///
/// // Named route
/// let route = Route::get("/users/:id", show_user).name("users.show");
/// ```
#[derive(Clone)]
pub struct Route {
    /// The HTTP method for this route
    pub method: Method,
    /// The path pattern (e.g., "/users/:id")
    pub path: String,
    /// The handler function
    pub handler: Arc<dyn BoxedHandler>,
    /// Optional route name for identification
    pub name: Option<String>,
}

impl Route {
    /// Creates a new route with the given method, path, and handler.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method
    /// * `path` - The path pattern
    /// * `handler` - The handler function
    ///
    /// # Example
    ///
    /// ```ignore
    /// use http::Method;
    /// use ketzal_router::Route;
    ///
    /// async fn handler() -> Response { ... }
    ///
    /// let route = Route::new(Method::GET, "/", handler);
    /// ```
    pub fn new<M: 'static>(method: Method, path: &str, handler: impl Handler<M>) -> Self {
        Self { method, path: path.to_string(), handler: Arc::from(into_boxed(handler)), name: None }
    }

    /// Creates a GET route.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Route::get("/", handler)
    /// ```
    pub fn get<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::GET, path, handler)
    }

    /// Creates a POST route.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Route::post("/users", create_user)
    /// ```
    pub fn post<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::POST, path, handler)
    }

    /// Creates a PUT route.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Route::put("/users/:id", update_user)
    /// ```
    pub fn put<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::PUT, path, handler)
    }

    /// Creates a DELETE route.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Route::delete("/users/:id", delete_user)
    /// ```
    pub fn delete<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::DELETE, path, handler)
    }

    /// Creates a PATCH route.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Route::patch("/users/:id", patch_user)
    /// ```
    pub fn patch<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::PATCH, path, handler)
    }

    /// Sets the name of the route.
    ///
    /// # Arguments
    ///
    /// * `name` - The route name
    ///
    /// # Example
    ///
    /// ```ignore
    /// Route::get("/users/:id", show_user).name("users.show")
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Calls the route handler with the given parameters and request.
    ///
    /// # Arguments
    ///
    /// * `params` - The extracted route parameters
    /// * `req` - The incoming request (if available)
    pub fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture {
        self.handler.call(params, req)
    }
}
