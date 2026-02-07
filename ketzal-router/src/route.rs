use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use http::Method;

pub type HandlerFuture<Res> =
    Pin<Box<dyn Future<Output = Res> + Send>>;

pub type Handler<Req, Res> =
    dyn Fn(Req) -> HandlerFuture<Res> + Send + Sync;

#[derive(Clone)]
pub struct Route<Req, Res> {
    pub method: Method,
    pub path: String,
    pub name: Option<String>,
    pub handler: Arc<Handler<Req, Res>>,
}

impl<Req, Res> Route<Req, Res>
where
    Req: Send + 'static,
    Res: Send + 'static,
{
    fn new<F, Fut>(method: Method, path: &str, handler: F) -> Self
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
    {
        let handler = Arc::new(move |req: Req| {
            Box::pin(handler(req)) as HandlerFuture<Res>
        });

        Self {
            method,
            path: path.to_string(),
            name: None,
            handler,
        }
    }

    /// GET route
    pub fn get<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
    {
        Self::new(Method::GET, path, handler)
    }

    /// POST route
    pub fn post<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
    {
        Self::new(Method::POST, path, handler)
    }
    /// PUT route
    pub fn put<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
    {
        Self::new(Method::PUT, path, handler)
    }
    /// DELETE route
    pub fn delete<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
    {
        Self::new(Method::DELETE, path, handler)
    }
    /// PATCH route
    pub fn patch<F, Fut>(path: &str, handler: F) -> Self
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
    {
        Self::new(Method::PATCH, path, handler)
    }

    /// route name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}
