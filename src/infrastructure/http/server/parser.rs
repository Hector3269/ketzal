use crate::infrastructure::http::method::Method;
use crate::infrastructure::http::request::request::Request;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

pub fn parse_request(buffer: &[u8]) -> Result<Request, Box<dyn std::error::Error>> {
    let request_str = std::str::from_utf8(buffer)?;
    let mut lines = request_str.lines();

    // First line: METHOD PATH VERSION
    let first_line = lines.next().ok_or("No request line")?;
    let mut parts = first_line.split_whitespace();
    let method_str = parts.next().ok_or("No method")?;
    let path_qs = parts.next().ok_or("No path")?;
    let _version = parts.next().ok_or("No version")?;

    let method = Method::from_str(method_str).ok_or("Invalid method")?;

    // Parse path and query
    let (path, query_string) = if let Some(pos) = path_qs.find('?') {
        (path_qs[..pos].to_string(), Some(path_qs[pos + 1..].to_string()))
    } else {
        (path_qs.to_string(), None)
    };

    // Parse query_params
    let mut query_params = HashMap::new();
    if let Some(qs) = &query_string {
        for pair in qs.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = urlencoding::decode(&pair[..eq_pos])?.to_string();
                let value = urlencoding::decode(&pair[eq_pos + 1..])?.to_string();
                query_params.insert(key, value);
            }
        }
    }

    // Headers
    let mut headers = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    // Body: the rest
    let body_start = request_str.find("\r\n\r\n").map(|p| p + 4).unwrap_or(request_str.len());
    let content_length = headers.get("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let body = buffer[body_start..body_start + content_length].to_vec();

    // Params empty for now
    let params = HashMap::new();

    let mut request = Request::new(method, path);
    request.headers = headers;
    request.body = body;
    request.params = params;
    request.query_params = query_params;
    request.query_string = query_string;

    Ok(request)
}

pub fn parse_request_dynamic(stream: &mut TcpStream) -> Result<Request, Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream);

    // Read first line
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let first_line = first_line.trim();

    let mut parts = first_line.split_whitespace();
    let method_str = parts.next().ok_or("No method")?;
    let path_qs = parts.next().ok_or("No path")?;
    let _version = parts.next().ok_or("No version")?;

    let method = Method::from_str(method_str).ok_or("Invalid method")?;

    // Parse path and query
    let (path, query_string) = if let Some(pos) = path_qs.find('?') {
        (path_qs[..pos].to_string(), Some(path_qs[pos + 1..].to_string()))
    } else {
        (path_qs.to_string(), None)
    };

    // Parse query_params
    let mut query_params = HashMap::new();
    if let Some(qs) = &query_string {
        for pair in qs.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = urlencoding::decode(&pair[..eq_pos])?.to_string();
                let value = urlencoding::decode(&pair[eq_pos + 1..])?.to_string();
                query_params.insert(key, value);
            }
        }
    }

    // Headers
    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    // Body
    let content_length = headers.get("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body)?;

    // Params empty
    let params = HashMap::new();

    let mut request = Request::new(method, path);
    request.headers = headers;
    request.body = body;
    request.params = params;
    request.query_params = query_params;
    request.query_string = query_string;

    Ok(request)
}