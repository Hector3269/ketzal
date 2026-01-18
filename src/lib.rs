pub mod  cli;
pub mod kernel;
pub mod infrastructure;

pub mod routing {
    pub use crate::infrastructure::http::routes::routes::Route;
}