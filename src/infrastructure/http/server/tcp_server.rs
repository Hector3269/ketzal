use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;

use crate::infrastructure::http::error::ServerError;
use crate::infrastructure::http::method::Method;
use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::server::dispatcher::dispatch_request;
use crate::infrastructure::http::server::logger::log_request;
use crate::infrastructure::http::server::parser::parse_request_dynamic;
use crate::infrastructure::http::server::writer::write_response;
use crate::kernel::config::server::ServerConfig;

pub async fn run(config: ServerConfig) -> Result<(), ServerError> {
    let addr = config.addr();

    let listener = TcpListener::bind(&addr)
        .await
        .map_err(ServerError::BindError)?;

    let semaphore = Arc::new(Semaphore::new(config.max_connections));

    println!(
        "🚀 Server running on http://{} (max connections: {})",
        addr, config.max_connections
    );

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let permit = semaphore.clone().acquire_owned().await.unwrap();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream).await {
                        eprintln!("Connection error: {}", e);
                    }
                    drop(permit);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client_addr = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let (read_half, mut write_half) = stream.into_split();

    match parse_request_dynamic(read_half).await {
        Ok(mut request) => {
            let response = dispatch_request(&mut request).await;
            log_request(&request, &client_addr, response.status);
            let _ = write_response(&mut write_half, &response).await;
        }
        Err(e) => {
            eprintln!("Parse error from {}: {}", client_addr, e);
            let response = Response::bad_request(format!("Bad Request: {}", e));
            log_request(
                &Request::new(Method::GET, "/".to_string()),
                &client_addr,
                response.status,
            );
            let _ = write_response(&mut write_half, &response).await;
        }
    }
    Ok(())
}
