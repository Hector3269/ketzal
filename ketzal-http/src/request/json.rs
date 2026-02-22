use super::Request;
use http::{header::CONTENT_TYPE, StatusCode};
use serde_json::Value;
use std::collections::HashMap;

use crate::response::Response;
impl Request {
    pub fn is_json(&self) -> bool {
        self.headers
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.starts_with("application/json"))
            .unwrap_or(false)
    }

    #[allow(clippy::result_large_err)]
    pub fn json(&self) -> Result<Value, Response> {
        if !self.is_json() {
            return Err(Response::json_error(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Unsupported Media Type. Expected application/json",
            ));
        }

        if self.body.is_empty() {
            return Err(Response::json_error(StatusCode::BAD_REQUEST, "Request body is empty"));
        }

        serde_json::from_slice::<Value>(&self.body)
            .map_err(|_| Response::json_error(StatusCode::BAD_REQUEST, "Invalid JSON body"))
    }

    #[allow(clippy::result_large_err)]
    pub fn json_map(&self) -> Result<HashMap<String, String>, Response> {
        let json = self.json()?;

        let mut data = HashMap::new();

        if let Some(obj) = json.as_object() {
            for (k, v) in obj {
                let value = match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                };

                data.insert(k.clone(), value);
            }
        }

        Ok(data)
    }
}
