//! Handler module
//!
//! Provides the [`Handler`] trait and related types for creating route handlers.
//!
//! ## Handler Signatures
//!
//! ketzal-router supports multiple handler function signatures:
//!
//! ### No Parameters
//! ```ignore
//! async fn handler() -> Response {
//!     Response::ok("Hello!")
//! }
//! ```
//!
//! ### With Request
//! ```ignore
//! async fn handler(req: Request) -> Response {
//!     Response::ok("Got request!")
//! }
//! ```
//!
//! ### With Path Parameters
//! ```ignore
//! async fn handler(id: i32) -> Response {
//!     Response::ok(format!("User {}", id))
//! }
//! ```
//!
//! ### With Request and Parameters
//! ```ignore
//! async fn handler(req: Request, id: i32) -> Response {
//!     Response::ok(format!("Request for user {}", id))
//! }
//! ```

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::params::Params;
use ketzal_http::{Request, Response};

/// The return type for all handlers - a pinned boxed future that produces a Response.
pub type HandlerFuture = Pin<Box<dyn Future<Output = Response> + Send>>;

/// Trait for extracting a single path parameter from a string.
///
/// Implement this trait for custom types that you want to use as route parameters.
///
/// # Example
///
/// ```ignore
/// impl FromParam for MyType {
///     fn from_param(value: &str) -> Result<Self, Response> {
///         // Parse the value
///         Ok(MyType::parse(value)?)
///     }
/// }
/// ```
pub trait FromParam: Sized {
    /// Attempts to convert a path parameter string into the implementing type.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value from the URL path
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` on success, or `Err(Response)` with a 400 Bad Request on failure.
    #[allow(clippy::result_large_err)]
    fn from_param(value: &str) -> Result<Self, Response>;
}

impl FromParam for i32 {
    fn from_param(v: &str) -> Result<Self, Response> {
        v.parse().map_err(|_| Response::bad_request("Invalid i32"))
    }
}

impl FromParam for u64 {
    fn from_param(v: &str) -> Result<Self, Response> {
        v.parse().map_err(|_| Response::bad_request("Invalid u64"))
    }
}

impl FromParam for String {
    fn from_param(v: &str) -> Result<Self, Response> {
        Ok(v.to_string())
    }
}

/// Trait for extracting multiple path parameters as a tuple.
///
/// This is implemented automatically for tuples of types that implement [`FromParam`].
pub trait FromParams: Sized {
    /// Attempts to extract parameters from the [`Params`].
    ///
    /// # Arguments
    ///
    /// * `params` - The extracted route parameters
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` on success, or `Err(Response)` on failure.
    #[allow(clippy::result_large_err)]
    fn from_params(params: &Params) -> Result<Self, Response>;
}

impl FromParams for () {
    fn from_params(_: &Params) -> Result<Self, Response> {
        Ok(())
    }
}

impl<T: FromParam> FromParams for (T,) {
    fn from_params(params: &Params) -> Result<Self, Response> {
        let mut it = params.all().values();
        let t = T::from_param(it.next().ok_or_else(|| Response::bad_request("Missing param 1"))?)?;
        Ok((t,))
    }
}

impl<T: FromParam, U: FromParam> FromParams for (T, U) {
    fn from_params(params: &Params) -> Result<Self, Response> {
        let mut it = params.all().values();
        let t = T::from_param(it.next().ok_or_else(|| Response::bad_request("Missing param 1"))?)?;
        let u = U::from_param(it.next().ok_or_else(|| Response::bad_request("Missing param 2"))?)?;
        Ok((t, u))
    }
}

impl<T: FromParam, U: FromParam, V: FromParam> FromParams for (T, U, V) {
    fn from_params(params: &Params) -> Result<Self, Response> {
        let mut it = params.all().values();
        let t = T::from_param(it.next().ok_or_else(|| Response::bad_request("Missing param 1"))?)?;
        let u = U::from_param(it.next().ok_or_else(|| Response::bad_request("Missing param 2"))?)?;
        let v = V::from_param(it.next().ok_or_else(|| Response::bad_request("Missing param 3"))?)?;
        Ok((t, u, v))
    }
}

/// Marker type for handlers with no parameters: `fn() -> Response`
pub struct Zero;

/// Marker type for handlers with only Request: `fn(Request) -> Response`
pub struct WithReq;

/// Marker type for handlers with one parameter: `fn(T) -> Response`
pub struct One<T>(PhantomData<T>);

/// Marker type for handlers with Request and one parameter: `fn(Request, T) -> Response`
pub struct WithReqOne<T>(PhantomData<T>);

/// Marker type for handlers with two parameters: `fn(T, U) -> Response`
pub struct Two<T, U>(PhantomData<(T, U)>);

/// Marker type for handlers with Request and two parameters: `fn(Request, T, U) -> Response`
pub struct WithReqTwo<T, U>(PhantomData<(T, U)>);

/// Marker type for handlers with three parameters: `fn(T, U, V) -> Response`
pub struct Three<T, U, V>(PhantomData<(T, U, V)>);

/// Marker type for handlers with Request and three parameters: `fn(Request, T, U, V) -> Response`
pub struct WithReqThree<T, U, V>(PhantomData<(T, U, V)>);

/// The core Handler trait.
///
/// This trait is implemented automatically for async functions with various signatures.
/// You typically won't implement this trait directly - just write async functions
/// with the desired signature.
///
/// # Example
///
/// ```ignore
/// // These handlers are automatically implement Handler trait
/// async fn no_params() -> Response { ... }
/// async fn with_req(req: Request) -> Response { ... }
/// async fn with_param(id: i32) -> Response { ... }
/// async fn with_req_and_param(req: Request, id: i32) -> Response { ... }
/// ```
pub trait Handler<M>: Send + Sync + 'static {
    /// Calls the handler with the given parameters and request.
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture;
}

// ── fn() -> Response ──────────────────────────────────────────────────────────
impl<F, Fut> Handler<Zero> for F
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    fn call(&self, _: &Params, _: Option<Request>) -> HandlerFuture {
        Box::pin(self())
    }
}

// ── fn(Request) -> Response ───────────────────────────────────────────────────
impl<F, Fut> Handler<WithReq> for F
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    fn call(&self, _: &Params, req: Option<Request>) -> HandlerFuture {
        let req = req.expect("WithReq handler requires a Request");
        Box::pin(self(req))
    }
}

// ── fn(T) -> Response ─────────────────────────────────────────────────────────
impl<F, Fut, T> Handler<One<T>> for F
where
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    T: FromParam + 'static,
{
    fn call(&self, params: &Params, _: Option<Request>) -> HandlerFuture {
        match <(T,)>::from_params(params) {
            Ok((t,)) => Box::pin(self(t)),
            Err(r) => Box::pin(async move { r }),
        }
    }
}

impl<F, Fut, T> Handler<WithReqOne<T>> for F
where
    F: Fn(Request, T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    T: FromParam + 'static,
{
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture {
        let req = req.expect("WithReqOne handler requires a Request");
        match <(T,)>::from_params(params) {
            Ok((t,)) => Box::pin(self(req, t)),
            Err(r) => Box::pin(async move { r }),
        }
    }
}

impl<F, Fut, T, U> Handler<Two<T, U>> for F
where
    F: Fn(T, U) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    T: FromParam + 'static,
    U: FromParam + 'static,
{
    fn call(&self, params: &Params, _: Option<Request>) -> HandlerFuture {
        match <(T, U)>::from_params(params) {
            Ok((t, u)) => Box::pin(self(t, u)),
            Err(r) => Box::pin(async move { r }),
        }
    }
}

impl<F, Fut, T, U> Handler<WithReqTwo<T, U>> for F
where
    F: Fn(Request, T, U) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    T: FromParam + 'static,
    U: FromParam + 'static,
{
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture {
        let req = req.expect("WithReqTwo handler requires a Request");
        match <(T, U)>::from_params(params) {
            Ok((t, u)) => Box::pin(self(req, t, u)),
            Err(r) => Box::pin(async move { r }),
        }
    }
}

impl<F, Fut, T, U, V> Handler<Three<T, U, V>> for F
where
    F: Fn(T, U, V) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    T: FromParam + 'static,
    U: FromParam + 'static,
    V: FromParam + 'static,
{
    fn call(&self, params: &Params, _: Option<Request>) -> HandlerFuture {
        match <(T, U, V)>::from_params(params) {
            Ok((t, u, v)) => Box::pin(self(t, u, v)),
            Err(r) => Box::pin(async move { r }),
        }
    }
}

/// Trait for boxed handlers that can be stored dynamically.
///
/// This is used internally to store handlers in the [`Route`](crate::route::Route) struct.
pub trait BoxedHandler: Send + Sync + 'static {
    /// Calls the boxed handler with the given parameters and request.
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture;
}

struct HandlerWrapper<F, M> {
    f: F,
    _m: PhantomData<M>,
}

unsafe impl<F: Send, M> Send for HandlerWrapper<F, M> {}
unsafe impl<F: Sync, M> Sync for HandlerWrapper<F, M> {}

impl<F, M> BoxedHandler for HandlerWrapper<F, M>
where
    F: Handler<M> + 'static,
    M: 'static,
{
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture {
        self.f.call(params, req)
    }
}

/// Converts a handler function into a boxed handler.
///
/// This is used internally when creating routes.
pub fn into_boxed<M: 'static>(f: impl Handler<M>) -> Box<dyn BoxedHandler> {
    Box::new(HandlerWrapper { f, _m: PhantomData })
}
