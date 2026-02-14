use serde_json::Value;
use std::collections::HashMap;

pub fn make(data: Value) -> HashMap<String, Value> {
    data.as_object().unwrap().clone().into_iter().collect()
}
