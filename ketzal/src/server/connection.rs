use crate::routes::registry;
use http::{HeaderMap, Method};
use ketzal_http::Request;
use ketzal_http::Response;
use std::collections::HashMap;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn handle(&mut self) -> io::Result<()> {
        let mut buffer = vec![0u8; 8192];

        let n = self.stream.read(&mut buffer).await?;
        if n == 0 {
            return Ok(());
        }

        // Parse HTTP request
        let request_data = String::from_utf8_lossy(&buffer[..n]);
        let (method, path) = match parse_http_request(&request_data) {
            Some((method, path)) => (method, path),
            None => {
                let response = b"HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\n\r\nBad Request";
                self.stream.write_all(response).await?;
                self.stream.flush().await?;
                return Ok(());
            }
        };

        // Get router and find route
        let router = registry::get_router();
        
        // Create a basic request
        let request = Request::new(
            method.clone(),
            path.clone(),
            HashMap::new(),
            HeaderMap::new(),
            Vec::new(),
            HashMap::new(),
        );

        let response = if let Some(handler_future) = router.handle(method, &path, request) {
            // Execute the handler
            handler_future.await
        } else {
            Response::not_found()
        };

        // Send HTTP response
        let response_bytes = response.to_bytes();
        self.stream.write_all(&response_bytes).await?;
        self.stream.flush().await?;

        Ok(())
    }
}

/// Parse HTTP request line to extract method and path
fn parse_http_request(request: &str) -> Option<(Method, String)> {
    let line = request.lines().next()?;
    let parts: Vec<&str> = line.split_whitespace().collect::<Vec<_>>();
    
    if parts.len() < 2 {
        return None;
    }

    let method = match *parts.first()? {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        _ => return None,
    };

    let path = parts[1].to_string();
    Some((method, path))
}
