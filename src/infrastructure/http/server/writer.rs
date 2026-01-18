use std::io::Write;
use std::net::TcpStream;

use crate::infrastructure::http::response::response::Response;

pub fn write_response(stream: &mut TcpStream, response: &Response) -> Result<(), std::io::Error> {
    let http = response.to_http_string();
    stream.write_all(http.as_bytes())?;
    stream.flush()
}