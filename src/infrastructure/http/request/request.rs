use super::super::method::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum RequestError {
    InvalidJson(serde_json::Error),
    InvalidUtf8(std::string::FromUtf8Error),
    InvalidFormData(String),
    MissingField(String),
    ValidationError(super::form_request::ValidationErrors),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::InvalidJson(e) => write!(f, "JSON inválido: {}", e),
            RequestError::InvalidUtf8(e) => write!(f, "UTF-8 inválido: {}", e),
            RequestError::InvalidFormData(e) => write!(f, "Datos de formulario inválidos: {}", e),
            RequestError::MissingField(field) => write!(f, "Campo requerido: {}", field),
            RequestError::ValidationError(errors) => {
                write!(f, "Errores de validación: {:?}", errors)
            }
        }
    }
}

impl std::error::Error for RequestError {}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub query_string: Option<String>,
    cached_json: std::cell::RefCell<Option<Value>>,
    cached_form_data: std::cell::RefCell<Option<HashMap<String, String>>>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            method,
            path,
            headers: HashMap::new(),
            body: Vec::new(),
            params: HashMap::new(),
            query_params: HashMap::new(),
            query_string: None,
            cached_json: std::cell::RefCell::new(None),
            cached_form_data: std::cell::RefCell::new(None),
        }
    }

    pub fn param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    pub fn query(&self, key: &str) -> Option<&String> {
        self.query_params.get(key)
    }

    pub fn query_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.query_params
            .get(key)
            .map(|s| s.as_str())
            .unwrap_or(default)
    }

    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v)
    }

    pub fn bearer_token(&self) -> Option<String> {
        self.header("Authorization").and_then(|auth| {
            if auth.starts_with("Bearer ") {
                Some(auth[7..].to_string())
            } else {
                None
            }
        })
    }

    pub fn body_as_string(&self) -> Result<String, RequestError> {
        String::from_utf8(self.body.clone()).map_err(RequestError::InvalidUtf8)
    }

    pub fn is_json(&self) -> bool {
        self.header("Content-Type")
            .map(|ct| ct.contains("application/json"))
            .unwrap_or(false)
    }

    pub fn is_form(&self) -> bool {
        self.header("Content-Type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false)
    }

    pub fn json<T: DeserializeOwned>(&self) -> Result<T, RequestError> {
        serde_json::from_slice(&self.body).map_err(RequestError::InvalidJson)
    }

    pub fn json_value(&self) -> Result<Value, RequestError> {
        let mut cache = self.cached_json.borrow_mut();

        if cache.is_none() {
            let value = serde_json::from_slice(&self.body).map_err(RequestError::InvalidJson)?;
            *cache = Some(value);
        }

        Ok(cache.as_ref().unwrap().clone())
    }

    pub fn input(&self, key: &str) -> Option<String> {
        if let Some(val) = self.query(key) {
            return Some(val.clone());
        }

        if self.is_json() {
            if let Ok(json) = self.json_value() {
                if let Some(val) = json.get(key) {
                    return match val {
                        Value::String(s) => Some(s.clone()),
                        Value::Number(n) => Some(n.to_string()),
                        Value::Bool(b) => Some(b.to_string()),
                        _ => None,
                    };
                }
            }
        }

        if self.is_form() {
            if let Ok(form) = self.form_data() {
                return form.get(key).cloned();
            }
        }

        None
    }

    pub fn input_or(&self, key: &str, default: &str) -> String {
        self.input(key).unwrap_or_else(|| default.to_string())
    }

    pub fn all_inputs(&self) -> HashMap<String, Value> {
        let mut inputs = HashMap::new();

        for (key, val) in &self.query_params {
            inputs.insert(key.clone(), Value::String(val.clone()));
        }

        if self.is_json() {
            if let Ok(json) = self.json_value() {
                if let Value::Object(map) = json {
                    for (key, val) in map {
                        inputs.insert(key, val);
                    }
                }
            }
        }

        if self.is_form() {
            if let Ok(form) = self.form_data() {
                for (key, val) in form {
                    inputs.insert(key, Value::String(val));
                }
            }
        }

        inputs
    }

    pub fn only(&self, keys: &[&str]) -> HashMap<String, Value> {
        let all = self.all_inputs();
        keys.iter()
            .filter_map(|&key| all.get(key).map(|v| (key.to_string(), v.clone())))
            .collect()
    }

    pub fn except(&self, keys: &[&str]) -> HashMap<String, Value> {
        let all = self.all_inputs();
        all.into_iter()
            .filter(|(key, _)| !keys.contains(&key.as_str()))
            .collect()
    }

    pub fn form_data(&self) -> Result<HashMap<String, String>, RequestError> {
        let mut cache = self.cached_form_data.borrow_mut();

        if cache.is_none() {
            let body_str = self.body_as_string()?;
            let data: HashMap<String, String> = url::form_urlencoded::parse(body_str.as_bytes())
                .into_owned()
                .collect();
            *cache = Some(data);
        }

        Ok(cache.as_ref().unwrap().clone())
    }

    pub fn validate<F: super::form_request::FormRequest + Default>(
        &self,
        _form_request: &F,
    ) -> Result<HashMap<String, Value>, RequestError> {
        let data = self.all_inputs();
        F::validate_data(data).map_err(RequestError::ValidationError)
    }

    pub fn full_url(&self) -> String {
        if let Some(qs) = &self.query_string {
            format!("{}?{}", self.path, qs)
        } else {
            self.path.clone()
        }
    }
}
