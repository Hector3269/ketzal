use std::io::{Read, Write};
use std::net::TcpListener;

use crate::kernel::config::server::ServerConfig;
use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::routes::REGISTRY;
use crate::infrastructure::http::server::parser::parse_request;

pub fn run(config: ServerConfig) {
    let addr = config.addr();

    let listener = TcpListener::bind(&addr)
        .expect("Failed to bind TCP server");

    println!("🚀 Server running on http://{}", addr);

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut buffer = [0; 4096];
        let len = match stream.read(&mut buffer) {
            Ok(len) => len,
            Err(_) => continue,
        };

        let response = match parse_request(&buffer[..len]) {
            Ok(request) => {
                let registry = REGISTRY
                    .lock()
                    .expect("Route registry poisoned");

                match registry.get_routes().iter().find(|route| {
                    route.method == request.method
                        && route.path == request.path
                }) {
                    Some(route) => (route.handler)(request),
                    None => Response::not_found(),
                }
            }
            Err(err) => Response::bad_request(
                format!("Bad Request: {}", err)
            ),
        };

        let http = response.to_http_string();

        if let Err(err) = stream.write_all(http.as_bytes()) {
            eprintln!("Write error: {}", err);
        }

        let _ = stream.flush();
    }
}
