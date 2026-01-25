use crate::infrastructure::http::method::Method;
use crate::infrastructure::http::request::multipart::MultipartParser;
use crate::infrastructure::http::request::request::{Request, RequestBody};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

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
        (
            path_qs[..pos].to_string(),
            Some(path_qs[pos + 1..].to_string()),
        )
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
    let body_start = request_str
        .find("\r\n\r\n")
        .map(|p| p + 4)
        .unwrap_or(request_str.len());
    let content_length = headers
        .get("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let body_bytes =
        buffer[body_start..std::cmp::min(body_start + content_length, buffer.len())].to_vec();

    let mut request = Request::new(method, path);
    request.headers = headers;
    request.body = RequestBody::Buffered(body_bytes);
    request.query_params = query_params;
    request.query_string = query_string;

    Ok(request)
}

pub async fn parse_request_dynamic(
    read_half: OwnedReadHalf,
) -> Result<Request, Box<dyn std::error::Error + Send + Sync>> {
    let mut reader = BufReader::new(read_half);

    // Read first line
    let mut first_line = String::with_capacity(256);
    reader.read_line(&mut first_line).await?;
    let first_line = first_line.trim();

    if first_line.is_empty() {
        return Err("Empty request".into());
    }

    let mut parts = first_line.split_whitespace();
    let method_str = parts.next().ok_or("No method")?;
    let path_qs = parts.next().ok_or("No path")?;
    let _version = parts.next().ok_or("No version")?;

    let method = Method::from_str(method_str).ok_or("Invalid method")?;

    // Parse path and query
    let (path, query_string) = if let Some(pos) = path_qs.find('?') {
        (
            path_qs[..pos].to_string(),
            Some(path_qs[pos + 1..].to_string()),
        )
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
    let mut line_buffer = String::with_capacity(256);
    loop {
        line_buffer.clear();
        reader.read_line(&mut line_buffer).await?;
        let line = line_buffer.trim();
        if line.is_empty() {
            break;
        }
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    // Body handling
    let content_length = headers
        .get("Content-Length")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let threshold = 16 * 1024; // 16KB

    let mut request = Request::new(method, path);
    request.headers = headers;
    request.query_params = query_params;
    request.query_string = query_string;

    if content_length > threshold || request.is_multipart() {
        let body_stream = reader.take(content_length);
        request.body = RequestBody::Streaming(Arc::new(Mutex::new(Some(Box::new(body_stream)))));
    } else {
        let mut body = vec![0; content_length as usize];
        reader.read_exact(&mut body).await?;

        if request.is_multipart() {
            if let Some(ct) = request.header("Content-Type") {
                if let Some(boundary_pos) = ct.find("boundary=") {
                    let boundary = &ct[boundary_pos + 9..];
                    let (_form_data, files) = MultipartParser::parse(&body, boundary);
                    request.files = files;
                }
            }
        }

        request.body = RequestBody::Buffered(body);
    }

    Ok(request)
}
