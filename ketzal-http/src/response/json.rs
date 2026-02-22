use http::{HeaderValue, StatusCode};
use serde::Serialize;
use std::collections::HashMap;

use super::Response;

impl Response {
    pub fn json<T: Serialize>(value: T) -> Self {
        Self::json_with_status(StatusCode::OK, value)
    }

    pub fn json_with_status<T: Serialize>(status: StatusCode, value: T) -> Self {
        let body = serde_json::to_vec(&value).unwrap_or_else(|_| b"{}".to_vec());

        let mut response = Self::with_body(status, body);

        response
            .headers
            .insert(http::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

        response
    }

    pub fn json_error(status: StatusCode, message: impl Into<String>) -> Self {
        let mut map = HashMap::new();
        map.insert("error", message.into());

        Self::json_with_status(status, map)
    }

    pub fn json_validation_error<T: Serialize>(errors: T) -> Self {
        Self::json_with_status(StatusCode::UNPROCESSABLE_ENTITY, errors)
    }
}
