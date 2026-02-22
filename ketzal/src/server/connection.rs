use crate::routes::registry;
use ketzal_http::protocol::h1;
use ketzal_http::Response;
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
    kind: RouterKind,
}

#[derive(Clone, Copy)]
pub enum RouterKind {
    Web,
    Api,
}

impl Connection {
    pub fn new(stream: TcpStream, kind: RouterKind) -> Self {
        Self { stream, kind }
    }

    pub async fn handle(&mut self) -> io::Result<()> {
        let request = match h1::decode(&mut self.stream).await? {
            Some(req) => req,
            None => return Ok(()),
        };

        let router = match self.kind {
            RouterKind::Web => registry::get_web_router(),
            RouterKind::Api => registry::get_api_router(),
        };

        let response = match router.handle(&request.method.clone(), &request.path.clone(), request)
        {
            Some(future) => future.await,
            None => Response::not_found(),
        };

        let bytes = h1::encode(&response);
        self.stream.write_all(&bytes).await?;
        self.stream.flush().await?;

        Ok(())
    }
}
