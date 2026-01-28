use crate::infrastructure::http::middleware::MiddlewareFuture;
use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::routes::REGISTRY;
use crate::kernel::constants::headers::{ACCEPT_ENCODING, GZIP_ENCODING};

pub async fn dispatch_request(request: &mut Request) -> Response {
    let matched = {
        let registry = REGISTRY.lock().unwrap();
        registry.match_route(&request.path, request.method)
    };

    let mut response = if let Some((handler, middlewares, params)) = matched {
        request.params = params;

        let mut current_handler: Box<dyn FnOnce(Request) -> MiddlewareFuture + Send> =
            Box::new(move |req| (handler)(req));

        for mw in middlewares.into_iter().rev() {
            let next = current_handler;
            let mw_captured = mw.clone();
            current_handler = Box::new(move |req| mw_captured.handle(req, next));
        }

        current_handler(request.clone()).await
    } else {
        Response::not_found()
    };

    // Compress if client accepts gzip
    if let Some(accept_encoding) = request.header(ACCEPT_ENCODING) {
        if accept_encoding.contains(GZIP_ENCODING) {
            response = response.compress_gzip();
        }
    }

    response
}
