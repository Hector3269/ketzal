pub mod cli;
pub mod domain;
pub mod infrastructure;
pub mod kernel;

pub mod http {
    pub use crate::infrastructure::http::error;
    pub use crate::infrastructure::http::method;
    pub mod request {
        pub use crate::infrastructure::http::request::multipart;
        pub use crate::infrastructure::http::request::request;
        pub use crate::infrastructure::http::request::utils;
        pub mod form_request {
            pub use crate::infrastructure::http::request::form_request::errors;
            pub use crate::infrastructure::http::request::form_request::form_request_macro;
            pub use crate::infrastructure::http::request::form_request::traits;
            pub use crate::infrastructure::http::request::form_request::validate_rules;
            pub use crate::infrastructure::http::request::form_request::validator;
        }
    }
    pub use crate::infrastructure::http::middleware;
    pub use crate::infrastructure::http::response;
    pub mod routing {
        pub use crate::infrastructure::http::routes::routes::Route;
    }
}
