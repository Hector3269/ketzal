use crate::infrastructure::http::middleware::{Middleware, MiddlewareFuture};
use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;

pub struct CorsMiddleware;

impl Middleware for CorsMiddleware {
    fn handle(
        &self,
        request: Request,
        next: crate::infrastructure::http::middleware::Next,
    ) -> MiddlewareFuture {
        Box::pin(async move {
            // Handle preflight
            if request.method == crate::infrastructure::http::method::Method::OPTIONS {
                return Response::new(204, "".to_string())
                    .with_header("Access-Control-Allow-Origin".to_string(), "*".to_string())
                    .with_header(
                        "Access-Control-Allow-Methods".to_string(),
                        "GET, POST, PUT, DELETE, PATCH, OPTIONS".to_string(),
                    )
                    .with_header(
                        "Access-Control-Allow-Headers".to_string(),
                        "Content-Type, Authorization".to_string(),
                    );
            }

            let mut response = next(request).await;

            response
                .headers
                .insert("Access-Control-Allow-Origin".to_string(), "*".to_string());

            response
        })
    }
}
