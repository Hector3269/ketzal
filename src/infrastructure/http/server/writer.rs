use crate::infrastructure::http::response::response::Response;
use crate::infrastructure::http::response::sse::SSEEvent;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use crate::kernel::constants::{
    protocol::{HTTP_VERSION_1_1, CRLF, HEADER_SEPARATOR},
    headers::CONTENT_LENGTH,
    status_code::reason_phrase
};

pub async fn write_response<W>(stream: &mut W, response: &Response) -> Result<(), std::io::Error>
where
    W: AsyncWrite + Unpin,
{
    // Write status line
    let status_text = reason_phrase(response.status);
    stream
        .write_all(format!("{} {} {}{}", HTTP_VERSION_1_1, response.status, status_text, CRLF).as_bytes())
        .await?;

    // Write headers
    for (key, value) in &response.headers {
        stream
            .write_all(format!("{}{}{}{}", key, HEADER_SEPARATOR, value, CRLF).as_bytes())
            .await?;
    }

    // Write content-length and body
    stream
        .write_all(format!("{}{}{}{}{}", CONTENT_LENGTH, HEADER_SEPARATOR, response.body.len(), CRLF, CRLF).as_bytes())
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
