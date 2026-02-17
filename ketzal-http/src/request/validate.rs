use std::collections::HashMap;
use std::ops::ControlFlow;

use http::{header::CONTENT_TYPE, StatusCode};
use serde_json::Value;

use ketzal_validation::Validator;

use crate::request::validated_data::ValidatedData;
use crate::request::Request;
use crate::response::Response;

impl Request {
    pub fn validate<const N: usize>(
        &self,
        rules: [(&'static str, &'static str); N],
    ) -> ControlFlow<Response, ValidatedData> {
        let data = match self.json_body() {
            Ok(data) => data,
            Err(response) => return ControlFlow::Break(response),
        };

        let rules_map: HashMap<&str, &str> = rules.iter().cloned().collect();

        let mut validator = Validator::make(data, rules_map);

        match validator.validate() {
            Ok(()) => ControlFlow::Continue(ValidatedData::new(validator.validated_data())),
            Err(errors) => ControlFlow::Break(Response::json_validation_error(errors)),
        }
    }

    fn json_body(&self) -> Result<HashMap<String, Value>, Response> {
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
}
