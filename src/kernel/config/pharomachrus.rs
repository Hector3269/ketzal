use crate::infrastructure::http::server::tcp_server;
use crate::kernel::config::server::ServerConfig;

pub struct Pharomachrus {
    server_config: ServerConfig,
}

impl Pharomachrus {
    pub fn new() -> Self {
        Self {
            server_config: ServerConfig::default(),
        }
    }
    pub fn with_server(mut self, config: ServerConfig) -> Self {
        self.server_config = config;
        self
    }
    pub async fn create(self) {
        let _ = tcp_server::run(self.server_config).await;
    }
}
