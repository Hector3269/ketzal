use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::kernel::config::server::ServerConfig;
use crate::infrastructure::http::method::Method;
use crate::infrastructure::http::request::request::Request;
use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::server::dispatcher::dispatch_request;
use crate::infrastructure::http::server::logger::log_request;
use crate::infrastructure::http::server::parser::parse_request_dynamic;
use crate::infrastructure::http::server::writer::write_response;

pub fn run(config: ServerConfig) {
    let addr = config.addr();

    let listener = TcpListener::bind(&addr)
        .expect("Failed to bind TCP server");

    println!("🚀 Server running on http://{}", addr);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Connection error: {}", e);
                continue;
            }
        };

        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: TcpStream) {
    let client_addr = stream.peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    match parse_request_dynamic(&mut stream) {
        Ok(mut request) => {
            let response = dispatch_request(&mut request);
            log_request(&request, &client_addr, response.status,);
            let _ = write_response(&mut stream, &response);
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            let response = Response::bad_request(format!("Bad Request: {}", e));
            log_request(&Request::new(Method::GET, "/".to_string()), &client_addr, response.status);
            let _ = write_response(&mut stream, &response);
        }
    }
}