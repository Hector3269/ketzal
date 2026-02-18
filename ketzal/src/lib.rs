pub mod config;
pub mod routes;
pub mod server;
pub use ketzal_http::{Request, Response};
pub use ketzal_router::{Route, Router};

// macro validator
#[macro_export]
macro_rules! form_request {
    (
        $name:ident {
            rules: {
                $($field:expr => $rule:expr),* $(,)?
            }
            $(, messages: {
                $($msg_key:expr => $msg_val:expr),* $(,)?
            })?
            $(, attributes: {
                $($attr_key:expr => $attr_val:expr),* $(,)?
            })?
        }
    ) => {
        #[derive(Default)]
        pub struct $name;

        impl ketzal_validation::FormRequest for $name {
            fn rules(&self) -> std::collections::HashMap<&'static str, &'static str> {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert($field, $rule);
                )*
                map
            }

            $(
                fn messages(&self) -> std::collections::HashMap<&'static str, &'static str> {
                    let mut map = std::collections::HashMap::new();
                    $(
                        map.insert($msg_key, $msg_val);
                    )*
                    map
                }
            )?

            $(
                fn attributes(&self) -> std::collections::HashMap<&'static str, &'static str> {
                    let mut map = std::collections::HashMap::new();
                    $(
                        map.insert($attr_key, $attr_val);
                    )*
                    map
                }
            )?
        }
    };
}

// validate macros

/// application/json
#[macro_export]
macro_rules! validate_json {
    ($req:expr => {
        $($field:literal => $rule:literal),* $(,)?
    }) => {{
        let __req = &$req;

        match __req.validate_json([
            $(
                ($field, $rule),
            )*
        ]) {
            ::std::ops::ControlFlow::Continue(val) => val,
            ::std::ops::ControlFlow::Break(resp) => return resp,
        }
    }};
}
// application/x-www-form-urlencoded
#[macro_export]
macro_rules! validate_form {
    ($req:expr => {
        $($field:literal => $rule:literal),* $(,)?
    }) => {{
        let __req = &$req;

        match __req.validate_form([
            $(
                ($field, $rule),
            )*
        ]) {
            ::std::ops::ControlFlow::Continue(val) => val,
            ::std::ops::ControlFlow::Break(resp) => return resp,
        }
    }};
}

// routes

// Route registration macro for web routes
#[cfg(feature = "web")]
#[macro_export]
macro_rules! routes_web {
    ($($route:expr);* $(;)?) => {
        #[ctor::ctor]
        fn register_web_routes() {
            use $crate::routes::registry::register;

            $(
                register($route);
            )*
        }
    };
}

// Route registration macro for API routes
#[cfg(feature = "api")]
#[macro_export]
macro_rules! routes_api {
    ($($route:expr);* $(;)?) => {
        #[ctor::ctor]
        fn register_api_routes() {
            use $crate::routes::registry::register;

            $(
                register($route);
            )*
        }
    };
}
