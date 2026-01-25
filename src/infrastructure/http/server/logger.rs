use chrono::Utc;

use crate::infrastructure::http::request::request::Request;

pub fn log_request(request: &Request, client_addr: &str, status: u16) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
    println!(
        "[{}] {} {} {} {}",
        timestamp,
        request.method.as_str(),
        request.path,
        client_addr,
        status
    );
}
