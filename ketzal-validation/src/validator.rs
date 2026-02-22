use serde_json::Value;
use std::collections::HashMap;

use crate::errors::ValidationErrors;
use crate::rules::{rules_registry, split_rule};

pub struct Validator {
    pub data: HashMap<String, Value>,
    rules: HashMap<String, Vec<String>>,
    pub custom_messages: HashMap<String, String>,
    pub custom_attributes: HashMap<String, String>,
    errors: ValidationErrors,
}

impl Validator {
    pub fn make(data: HashMap<String, Value>, rules: HashMap<&'static str, &'static str>) -> Self {
        let parsed_rules = rules
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.split('|').map(|s| s.trim().to_string()).collect()))
            .collect();

        Self {
            data,
            rules: parsed_rules,
            custom_messages: HashMap::new(),
            custom_attributes: HashMap::new(),
            errors: HashMap::new(),
        }
    }

    pub fn set_custom_messages(&mut self, messages: HashMap<&'static str, &'static str>) {
        self.custom_messages =
            messages.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    }

    pub fn set_custom_attributes(&mut self, attributes: HashMap<&'static str, &'static str>) {
        self.custom_attributes =
            attributes.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    }

    pub fn validate(&mut self) -> Result<(), ValidationErrors> {
        self.errors.clear();
        let registry = rules_registry();

        for (field, rules) in &self.rules {
            let value = self.data.get(field);
            let field_name = self.custom_attributes.get(field).unwrap_or(field);

            // evitar allocation innecesaria
            if value.is_none() && rules.iter().any(|r| r == "nullable") {
                continue;
            }

            for rule in rules {
                let (rule_name, param) = split_rule(rule);

                if rule_name == "nullable" {
                    continue;
                }

                if let Some(handler) = registry.get(rule_name) {
                    if let Err(msg) = handler(field, field_name, value, self, param) {
                        self.errors.entry(field.clone()).or_default().push(msg);
                    }
                }
            }
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    pub fn validated_data(self) -> HashMap<String, Value> {
        self.data.into_iter().filter(|(k, _)| self.rules.contains_key(k)).collect()
    }
}
