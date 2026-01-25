use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::response::sse::SSEEvent;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub async fn write_response<W>(stream: &mut W, response: &Response) -> Result<(), std::io::Error>
where
    W: AsyncWrite + Unpin,
{
    // Write status line
    let status_text = Response::status_text(response.status);
    stream
        .write_all(format!("HTTP/1.1 {} {}\r\n", response.status, status_text).as_bytes())
        .await?;

    // Write headers
    for (key, value) in &response.headers {
        stream
            .write_all(format!("{}: {}\r\n", key, value).as_bytes())
            .await?;
    }

    // Write content-length and body
    stream
        .write_all(format!("Content-Length: {}\r\n\r\n", response.body.len()).as_bytes())
        .await?;
    stream.write_all(response.body.as_bytes()).await?;

    stream.flush().await
}

pub async fn write_sse_event<W>(stream: &mut W, event: &SSEEvent) -> Result<(), std::io::Error>
where
    W: AsyncWrite + Unpin,
{
    stream.write_all(event.to_string().as_bytes()).await?;
    stream.flush().await
}
