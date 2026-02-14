use super::Request;
use std::collections::HashMap;

impl Request {
    fn form(&self) -> HashMap<String, String> {
        serde_urlencoded::from_bytes::<HashMap<String, String>>(&self.body).unwrap_or_default()
    }

    pub fn form_value(&self, key: &str) -> String {
        self.form().get(key).cloned().unwrap_or_default()
    }
}
