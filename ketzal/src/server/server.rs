use crate::server::connection::{Connection, RouterKind};
use crate::server::listener::Listener;
use ketzal_http::config::ServerConfig;
use std::io;

pub struct Server {
    config: ServerConfig,
    listener: Listener,
    kind: RouterKind,
}

impl Server {
    pub async fn new(config: ServerConfig, kind: RouterKind) -> io::Result<Self> {
        let addr = config.socket_addr();
        let listener = Listener::bind(&addr.to_string()).await?;
        Ok(Self {
            config,
            listener,
            kind,
        })
    }

    pub async fn web(config: ServerConfig) -> io::Result<Self> {
        Self::new(config, RouterKind::Web).await
    }

    pub async fn api(config: ServerConfig) -> io::Result<Self> {
        Self::new(config, RouterKind::Api).await
    }

    pub async fn run(self) -> io::Result<()> {
        println!("🚀 Server running on {}", self.config.socket_addr());

        let kind = self.kind;
        loop {
            let stream = self.listener.accept().await?;

            tokio::spawn(async move {
                let mut conn = Connection::new(stream, kind);
                if let Err(e) = conn.handle().await {
                    eprintln!("❌ connection error: {e}");
                }
            });
        }
    }
}
