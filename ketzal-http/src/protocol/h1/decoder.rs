use crate::request::Request;
use http::{HeaderMap, Method};
use std::collections::HashMap;
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt};

pub async fn decode<S>(stream: &mut S) -> io::Result<Option<Request>>
where
    S: AsyncRead + Unpin,
{
    let mut buffer = Vec::new();
    let mut temp = [0u8; 1024];

    loop {
        let n = stream.read(&mut temp).await?;
        if n == 0 {
            return Ok(None);
        }

        buffer.extend_from_slice(&temp[..n]);

        if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }

    let header_end = match buffer.windows(4).position(|w| w == b"\r\n\r\n") {
        Some(pos) => pos,
        None => return Ok(None),
    };

    let headers_bytes = &buffer[..header_end];
    let mut body = buffer[header_end + 4..].to_vec();

    let headers_str = String::from_utf8_lossy(headers_bytes);

    let (method, path) = parse_request_line(&headers_str)?;

    let mut headers = HeaderMap::new();
    let mut content_length = 0usize;

    for line in headers_str.lines().skip(1) {
        if let Some((key, value)) = line.split_once(": ") {
            if key.eq_ignore_ascii_case("Content-Length") {
                content_length = value.trim().parse().unwrap_or(0);
            }

            if let (Ok(name), Ok(val)) = (
                http::header::HeaderName::from_bytes(key.as_bytes()),
                value.trim().parse(),
            ) {
                headers.insert(name, val);
            }
        }
    }

    while body.len() < content_length {
        let n = stream.read(&mut temp).await?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&temp[..n]);
    }

    Ok(Some(Request::new(
        method,
        path,
        HashMap::new(),
        headers,
        body,
        HashMap::new(),
    )))
}

fn parse_request_line(request: &str) -> io::Result<(Method, String)> {
    let line = request
        .lines()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid request"))?;

    let mut parts = line.split_whitespace();

    let method_str = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing method"))?;

    let path = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing path"))?;

    let method = Method::from_bytes(method_str.as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unsupported HTTP method"))?;

    Ok((method, path.to_string()))
}
