#[cfg(feature = "web")]
#[macro_export]
macro_rules! routes_web {
    ($($route:expr);* $(;)?) => {
        #[ctor::ctor]
        fn register_web_routes() {
            use $crate::infrastructure::http::routes::REGISTRY;

            let mut registry = REGISTRY
                .lock()
                .expect("Route registry poisoned");

            $(
                registry.register($route.into_data());
            )*
        }
    };
}
#[cfg(feature = "api")]
#[macro_export]
macro_rules! routes_api {
    ($($route:expr);* $(;)?) => {
        #[ctor::ctor]
        fn register_api_routes() {
            use $crate::infrastructure::http::routes::REGISTRY;

            let mut registry = REGISTRY
                .lock()
                .expect("Route registry poisoned");

            $(
                registry.register($route.into_data());
            )*
        }
    };
}
