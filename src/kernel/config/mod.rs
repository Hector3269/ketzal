use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::kernel::error::KernelResult;

#[derive(Debug, Clone)]
pub struct Config {
    data: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> KernelResult<()> {
        let content = fs::read_to_string(path)?;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                self.data
                    .insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Ok(())
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn get_or_default(&self, key: &str, default: &str) -> String {
        self.get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    pub fn load_from_env(&mut self, prefix: &str) {
        for (key, value) in std::env::vars() {
            if key.starts_with(prefix) {
                let config_key = key.strip_prefix(prefix).unwrap_or(&key).to_lowercase();
                self.data.insert(config_key, value);
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
