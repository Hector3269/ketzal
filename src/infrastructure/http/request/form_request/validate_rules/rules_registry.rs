use std::collections::HashMap;

use serde_json::Value;

use crate::infrastructure::http::request::form_request::validator::Validator;

use super::{basic, conditional};

type RuleFn<'a> = Box<
    dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
        + Send
        + Sync
        + 'a,
>;

pub fn rules_registry<'a>() -> HashMap<&'static str, RuleFn<'a>> {
    let mut map = HashMap::new();
    map.insert(
        "required",
        Box::new(
            |f: &str, n: &str, v: Option<&Value>, val: &Validator, _: Option<&str>| {
                basic::required::required(f, n, v, val, None)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map.insert(
        "string",
        Box::new(
            |f: &str, n: &str, v: Option<&Value>, val: &Validator, _: Option<&str>| {
                basic::string_val::string(f, n, v, val, None)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map.insert(
        "numeric",
        Box::new(
            |f: &str, n: &str, v: Option<&Value>, val: &Validator, _: Option<&str>| {
                basic::numeric_val::numeric(f, n, v, val, None)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map.insert(
        "email",
        Box::new(
            move |f: &str, n: &str, v: Option<&Value>, val: &Validator, _: Option<&str>| {
                basic::email_val::email(f, n, v, val, None)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map.insert(
        "min",
        Box::new(
            |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                basic::min_val::min(f, n, v, val, p)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map.insert(
        "max",
        Box::new(
            |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                basic::max_val::max(f, n, v, val, p)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map.insert(
        "in",
        Box::new(
            |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                basic::in_val::in_val(f, n, v, val, p)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map.insert(
        "confirmed",
        Box::new(
            |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                basic::confirmed_val::confirmed(f, n, v, val, p)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map.insert(
        "required_if",
        Box::new(
            |f: &str, n: &str, v: Option<&Value>, val: &Validator, p: Option<&str>| {
                conditional::required_if(f, n, v, val, p)
            },
        )
            as Box<
                dyn Fn(&str, &str, Option<&Value>, &Validator, Option<&str>) -> Result<(), String>
                    + Send
                    + Sync,
            >,
    );
    map
}

pub fn split_rule(rule: &str) -> (&str, Option<&str>) {
    let mut parts = rule.splitn(2, ':');
    (parts.next().unwrap(), parts.next())
}
