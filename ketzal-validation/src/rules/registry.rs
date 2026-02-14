use std::collections::HashMap;
use std::sync::OnceLock;

use serde_json::Value;

use crate::validator::Validator;

use super::basic;
use super::conditional;

pub type Rule = Box<
    dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
        + Send
        + Sync,
>;
static RULES_REGISTRY: OnceLock<HashMap<&'static str, Rule>> = OnceLock::new();

pub fn rules_registry() -> &'static HashMap<&'static str, Rule> {
    RULES_REGISTRY.get_or_init(|| {
        let mut map = HashMap::new();

        map.insert(
            "required",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, _: Option<&str>| {
                    basic::required::required(f, n, v, val, None)
                },
            ) as Rule,
        );

        map.insert(
            "nullable",
            Box::new(
                |_f: &str, _n: &str, _v: Option<&Value>, _val: &Validator, _: Option<&str>| Ok(()),
            ) as Rule,
        );

        map.insert(
            "string",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, _: Option<&str>| {
                    basic::string::string(f, n, v, val, None)
                },
            ) as Rule,
        );

        map.insert(
            "numeric",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, _: Option<&str>| {
                    basic::numeric::numeric(f, n, v, val, None)
                },
            ) as Rule,
        );

        map.insert(
            "email",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, _: Option<&str>| {
                    basic::email::email(f, n, v, val, None)
                },
            ) as Rule,
        );

        map.insert(
            "min",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                    basic::min::min(f, n, v, val, p)
                },
            ) as Rule,
        );

        map.insert(
            "max",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                    basic::max::max(f, n, v, val, p)
                },
            ) as Rule,
        );

        map.insert(
            "in",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                    basic::in_::in_val(f, n, v, val, p)
                },
            ) as Rule,
        );

        map.insert(
            "confirmed",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                    basic::confirmed::confirmed(f, n, v, val, p)
                },
            ) as Rule,
        );

        map.insert(
            "required_if",
            Box::new(
                |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                    conditional::required_if(f, n, v, val, p)
                },
            ) as Rule,
        );

        map
    })
}

pub fn split_rule(rule: &str) -> (&str, Option<&str>) {
    let mut parts = rule.splitn(2, ':');
    (parts.next().unwrap(), parts.next())
}
