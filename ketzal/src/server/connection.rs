use crate::routes::registry;
use ketzal_http::protocol::h1;
use ketzal_http::Response;
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn handle(&mut self) -> io::Result<()> {
        let request = match h1::decode(&mut self.stream).await? {
            Some(req) => req,
            None => return Ok(()),
        };

        let router = registry::get_router();

        let method = request.method.clone();
        let path = request.path.clone();

        let response = if let Some(handler) = router.handle(method, &path, request) {
            handler.await
        } else {
            Response::not_found()
        };

        let bytes = h1::encode(&response);

        self.stream.write_all(&bytes).await?;
        self.stream.flush().await?;

        Ok(())
    }
}
