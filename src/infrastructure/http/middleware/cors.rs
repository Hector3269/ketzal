use crate::infrastructure::http::middleware::{Middleware, MiddlewareFuture};
use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use crate::kernel::constants::cors::{ALLOW_ALL_ORIGINS, ALLOW_METHODS, ALLOW_HEADERS};
use crate::kernel::constants::headers::{
    ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_HEADERS
};
use crate::kernel::constants::status_code::NO_CONTENT;

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
                return Response::new(NO_CONTENT, "".to_string())
                    .with_header(ACCESS_CONTROL_ALLOW_ORIGIN.to_string(), ALLOW_ALL_ORIGINS.to_string())
                    .with_header(
                        ACCESS_CONTROL_ALLOW_METHODS.to_string(),
                        ALLOW_METHODS.to_string(),
                    )
                    .with_header(
                        ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
                        ALLOW_HEADERS.to_string(),
                    );
            }

            let mut response = next(request).await;

            response
                .headers
                .insert(ACCESS_CONTROL_ALLOW_ORIGIN.to_string(), ALLOW_ALL_ORIGINS.to_string());

            response
        })
    }
}
