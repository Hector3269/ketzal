pub mod macros;
pub mod registry;
pub mod route_data;
pub mod routes;
pub mod trie;

use registry::RouteRegistry;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref REGISTRY: Mutex<RouteRegistry> =
        Mutex::new(RouteRegistry::new());
}
