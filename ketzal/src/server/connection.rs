use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn handle(&mut self) -> io::Result<()> {
        let mut buffer = [0u8; 1024];

        let n = self.stream.read(&mut buffer).await?;
        if n == 0 {
            return Ok(());
        }

        // placeholder: luego aquí va el parser HTTP
        let response = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello";

        self.stream.write_all(response).await?;
        self.stream.flush().await?;

        Ok(())
    }
}
