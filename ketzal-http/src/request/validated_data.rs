use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatedData {
    data: HashMap<String, Value>,
}

impl ValidatedData {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Self { data }
    }

    pub fn all(self) -> HashMap<String, Value> {
        self.data
    }

    pub fn only<const N: usize>(self, keys: [&str; N]) -> Self {
        let filtered = self
            .data
            .into_iter()
            .filter(|(k, _)| keys.contains(&k.as_str()))
            .collect();

        Self { data: filtered }
    }

    pub fn except<const N: usize>(self, keys: [&str; N]) -> Self {
        let filtered = self
            .data
            .into_iter()
            .filter(|(k, _)| !keys.contains(&k.as_str()))
            .collect();

        Self { data: filtered }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}
