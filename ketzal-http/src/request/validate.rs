use std::collections::HashMap;
use std::ops::ControlFlow;

use http::{header::CONTENT_TYPE, StatusCode};
use serde_json::Value;

use ketzal_validation::Validator;

use crate::request::validated_data::ValidatedData;
use crate::request::Request;
use crate::response::Response;

impl Request {
    pub fn validate_json<const N: usize>(
        &self,
        rules: [(&'static str, &'static str); N],
    ) -> ControlFlow<Response, ValidatedData> {
        let data = match self.parse_json() {
            Ok(data) => data,
            Err(resp) => return ControlFlow::Break(resp),
        };

        self.run_validation(data, rules)
    }

    pub fn validate_form<const N: usize>(
        &self,
        rules: [(&'static str, &'static str); N],
    ) -> ControlFlow<Response, ValidatedData> {
        let data = match self.parse_form() {
            Ok(data) => data,
            Err(resp) => return ControlFlow::Break(resp),
        };

        self.run_validation(data, rules)
    }

    fn run_validation<const N: usize>(
        &self,
        data: HashMap<String, Value>,
        rules: [(&'static str, &'static str); N],
    ) -> ControlFlow<Response, ValidatedData> {
        let rules_map: HashMap<&str, &str> = rules.iter().cloned().collect();

        let mut validator = Validator::make(data, rules_map);

        match validator.validate() {
            Ok(()) => ControlFlow::Continue(ValidatedData::new(validator.validated_data())),
            Err(errors) => ControlFlow::Break(Response::json_validation_error(errors)),
        }
    }

    fn parse_json(&self) -> Result<HashMap<String, Value>, Response> {
        let content_type = self
            .headers
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                Response::json_error(
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    "Content-Type must be application/json",
                )
            })?;

        if !content_type.starts_with("application/json") {
            return Err(Response::json_error(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Content-Type must be application/json",
            ));
        }

        if self.body.is_empty() {
            return Ok(HashMap::new());
        }

        let parsed: Value = serde_json::from_slice(&self.body)
            .map_err(|_| Response::json_error(StatusCode::BAD_REQUEST, "Invalid JSON body"))?;

        let obj = parsed.as_object().ok_or_else(|| {
            Response::json_error(StatusCode::BAD_REQUEST, "JSON body must be an object")
        })?;

        Ok(obj.clone().into_iter().collect())
    }

    fn parse_form(&self) -> Result<HashMap<String, Value>, Response> {
        let content_type = self
            .headers
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                Response::json_error(
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    "Content-Type must be application/x-www-form-urlencoded",
                )
            })?;

        if !content_type.starts_with("application/x-www-form-urlencoded") {
            return Err(Response::json_error(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Content-Type must be application/x-www-form-urlencoded",
            ));
        }

        if self.body.is_empty() {
            return Ok(HashMap::new());
        }

        let parsed: HashMap<String, String> = serde_urlencoded::from_bytes(&self.body)
            .map_err(|_| Response::json_error(StatusCode::BAD_REQUEST, "Invalid form body"))?;

        let mut map = HashMap::new();

        for (k, v) in parsed {
            map.insert(k, Value::String(v));
        }

        Ok(map)
    }
}
