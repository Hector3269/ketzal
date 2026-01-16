use crate::infrastructure::http::server::tcp_server;
use crate::kernel::config::server::ServerConfig;

pub struct Pharomachrus {
    pub web_routes: Vec<String>,
    pub api_routes: Vec<String>,
}

pub struct RoutesConfig {
    pub web: Vec<String>,
    pub api: Vec<String>,
}

impl Pharomachrus {
    pub fn new() -> Self {
        Self {
            web_routes: Vec::new(),
            api_routes: Vec::new(),
        }
    }

    pub fn with_routing(mut self, routes: RoutesConfig) -> Self {
        self.web_routes = routes.web;
        self.api_routes = routes.api;
        self
    }

    pub fn create(self) {
        println!("=== Web Routes ===");
        for r in &self.web_routes {
            println!("{}", r);
        }

        println!("=== API Routes ===");
        for r in &self.api_routes {
            println!("{}", r);
        }

        println!("Application created!");
        let config = ServerConfig::default();
        tcp_server::run(config);
    }
}
