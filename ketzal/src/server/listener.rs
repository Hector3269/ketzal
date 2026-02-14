use std::io;
use tokio::net::{TcpListener, TcpStream};

pub struct Listener {
    inner: TcpListener,
}

impl Listener {
    pub async fn bind(addr: &str) -> io::Result<Self> {
        let inner = TcpListener::bind(addr).await?;
        Ok(Self { inner })
    }

    pub async fn accept(&self) -> io::Result<TcpStream> {
        let (stream, addr) = self.inner.accept().await?;
        println!("📡 New connection from {addr}");
        Ok(stream)
    }
}
