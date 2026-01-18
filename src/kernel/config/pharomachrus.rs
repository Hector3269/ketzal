use crate::infrastructure::http::server::tcp_server;
use crate::kernel::config::server::ServerConfig;

/// Kernel principal de la aplicación.
///
/// Responsabilidades:
/// - Inicializar configuración del servidor
/// - Arrancar el servidor TCP
///
/// NO:
/// - Registra rutas
/// - Conoce middlewares
/// - Carga archivos manualmente
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
    pub fn create(self) {
        tcp_server::run(self.server_config);
    }
}
