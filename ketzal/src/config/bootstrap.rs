use ketzal_http::config::ServerConfig;
use crate::server::server::Server;

pub struct Bootstrap {
    server_config: ServerConfig,
}

impl Bootstrap {
    pub fn new() -> Self {
        Self {
            server_config: ServerConfig::default(),
        }
    }

    pub fn with_server(mut self, config: ServerConfig) -> Self {
        self.server_config = config;
        self
    }

    pub async fn create(self) -> std::io::Result<()> {
        println!("Starting server with config: {:?}", self.server_config);

        let server = Server::new(self.server_config).await?;
        server.run().await
    }
}

impl Default for Bootstrap {
    fn default() -> Self {
        Self::new()
    }
}
