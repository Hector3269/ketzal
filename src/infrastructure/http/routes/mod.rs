pub mod routes;
pub mod route_data;
pub mod registry;
pub mod macros;

use std::sync::Mutex;
use registry::RouteRegistry;

lazy_static::lazy_static! {
    pub static ref REGISTRY: Mutex<RouteRegistry> =
        Mutex::new(RouteRegistry::new());
}
