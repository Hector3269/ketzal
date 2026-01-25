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

        impl $crate::infrastructure::http::request::form_request::FormRequest for $name {
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
