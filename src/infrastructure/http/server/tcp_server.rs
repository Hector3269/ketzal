use std::net::TcpListener;
use std::io::{Read, Write};

use crate::kernel::config::server::ServerConfig;
use crate::infrastructure::http::response::Response;
use super::parser::parse_request;

pub fn run(config: ServerConfig) {
    let addr = config.addr();
    let listener = TcpListener::bind(&addr)
        .expect("Failed to bind TCP server");

    println!("🚀 Server running on http://{}", addr);

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0; 4096];
            let len = match stream.read(&mut buffer) {
                Ok(len) => len,
                Err(_) => continue,
            };

            // Parse request
            let response = match parse_request(&buffer[..len]) {
                Ok(request) => {
                    match request.path.as_str() {
                        //temporarily
                        "/" => Response::ok_text("Hello, world!".to_string()),
                        _ => Response::not_found(),
                    }
                }
                Err(e) => Response::bad_request(format!("Error parsing request: {}", e)),
            };

            let response_str = response.to_http_string();
            if let Err(e) = stream.write_all(response_str.as_bytes()) {
                eprintln!("Error writing response: {}", e);
            }
            let _ = stream.flush();
        }
    }
}
