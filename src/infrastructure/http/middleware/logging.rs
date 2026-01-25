use crate::infrastructure::http::middleware::{Middleware, MiddlewareFuture};
use crate::infrastructure::http::request::request::Request;

pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn handle(
        &self,
        request: Request,
        next: crate::infrastructure::http::middleware::Next,
    ) -> MiddlewareFuture {
        let method = request.method;
        let path = request.path.clone();

        Box::pin(async move {
            println!("Start processing requests: {:?} {}", method, path);
            let start = std::time::Instant::now();

            let response = next(request).await;

            let duration = start.elapsed();
            println!(
                "End processing requests: {:?} {} - Status: {} - Duration: {:?}",
                method, path, response.status, duration
            );

            response
        })
    }
}
