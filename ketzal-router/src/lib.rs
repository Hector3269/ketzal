//! # ketzal-router
//!
//! A lightweight, fast routing library for the Ketzal web framework.
//!
//! ## Features
//!
//! - HTTP Method Routing (GET, POST, PUT, DELETE, PATCH)
//! - Path Parameters with `:param` syntax
//! - Type-safe parameter extraction
//! - Flexible handler signatures
//! - Route naming support
//!
//! ## Quick Example
//!
//! ```ignore
//! use ketzal_router::{Router, Route};
//! use ketzal_http::{Request, Response};
//!
//! async fn hello() -> Response {
//!     Response::ok("Hello, World!")
//! }
//!
//! let mut router = Router::new();
//! router.register(Route::get("/", hello));
//! ```

pub mod handler;
pub mod params;
pub mod route;
pub mod route_group;
pub mod route_node;
pub mod router;

pub use handler::{BoxedHandler, FromParam, FromParams, Handler, HandlerFuture};
pub use route::Route;
pub use router::Router;
