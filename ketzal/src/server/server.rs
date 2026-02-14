use crate::server::connection::Connection;
use crate::server::listener::Listener;
use ketzal_http::config::ServerConfig;
use std::io;

pub struct Server {
    config: ServerConfig,
    listener: Listener,
}

impl Server {
    pub async fn new(config: ServerConfig) -> io::Result<Self> {
        let addr = config.socket_addr();
        let listener = Listener::bind(&addr.to_string()).await?;

        Ok(Self { config, listener })
    }

    pub async fn run(self) -> io::Result<()> {
        println!("🚀 Server running on {}", self.config.socket_addr());

        loop {
            let stream = self.listener.accept().await?;

            tokio::spawn(async move {
                let mut conn = Connection::new(stream);
                if let Err(e) = conn.handle().await {
                    eprintln!("❌ connection error: {e}");
                }
            });
        }
    }
}
