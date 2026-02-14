use crate::routes::registry;
use crate::server::server::Server;
use ketzal_http::config::ServerConfig;

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
        registry::init_router();

        let server = Server::new(self.server_config).await?;
        server.run().await
    }
}

impl Default for Bootstrap {
    fn default() -> Self {
        Self::new()
    }
}
