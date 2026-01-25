use super::super::method::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncRead;

#[derive(Debug)]
pub enum RequestError {
    InvalidJson(serde_json::Error),
    InvalidUtf8(std::string::FromUtf8Error),
    InvalidFormData(String),
    MissingField(String),
    ValidationError(super::form_request::ValidationErrors),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

impl UploadedFile {
    pub fn new(filename: String, content_type: String, data: Vec<u8>) -> Self {
        Self {
            filename,
            content_type,
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

pub enum RequestBody {
    Buffered(Vec<u8>),
    Streaming(Arc<Mutex<Option<Box<dyn AsyncRead + Unpin + Send + Sync>>>>),
}

impl fmt::Debug for RequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestBody::Buffered(b) => f.debug_tuple("Buffered").field(b).finish(),
            RequestBody::Streaming(_) => f.debug_tuple("Streaming").finish(),
        }
    }
}

impl Clone for RequestBody {
    fn clone(&self) -> Self {
        match self {
            RequestBody::Buffered(b) => RequestBody::Buffered(b.clone()),
            RequestBody::Streaming(s) => RequestBody::Streaming(s.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: RequestBody,
    pub params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub query_string: Option<String>,
    pub files: HashMap<String, UploadedFile>,
    cached_json: Arc<Mutex<Option<Value>>>,
    cached_form_data: Arc<Mutex<Option<HashMap<String, String>>>>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            method,
            path,
            headers: HashMap::new(),
            body: RequestBody::Buffered(Vec::new()),
            params: HashMap::new(),
            query_params: HashMap::new(),
            query_string: None,
            files: HashMap::new(),
            cached_json: Arc::new(Mutex::new(None)),
            cached_form_data: Arc::new(Mutex::new(None)),
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

    pub async fn body_bytes(&self) -> Result<Vec<u8>, RequestError> {
        match &self.body {
            RequestBody::Buffered(b) => Ok(b.clone()),
            RequestBody::Streaming(s) => {
                let mut guard = s.lock().unwrap();
                if let Some(mut stream) = guard.take() {
                    use tokio::io::AsyncReadExt;
                    let mut buf = Vec::new();
                    stream
                        .read_to_end(&mut buf)
                        .await
                        .map_err(|e| RequestError::InvalidFormData(e.to_string()))?;
                    Ok(buf)
                } else {
                    Err(RequestError::InvalidFormData(
                        "Body already consumed".to_string(),
                    ))
                }
            }
        }
    }

    pub async fn body_as_string(&self) -> Result<String, RequestError> {
        let bytes = self.body_bytes().await?;
        String::from_utf8(bytes).map_err(RequestError::InvalidUtf8)
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

    pub fn is_multipart(&self) -> bool {
        self.header("Content-Type")
            .map(|ct| ct.contains("multipart/form-data"))
            .unwrap_or(false)
    }

    pub fn file(&self, key: &str) -> Option<&UploadedFile> {
        self.files.get(key)
    }

    pub fn all_files(&self) -> &HashMap<String, UploadedFile> {
        &self.files
    }

    pub async fn json<T: DeserializeOwned>(&self) -> Result<T, RequestError> {
        let bytes = self.body_bytes().await?;
        serde_json::from_slice(&bytes).map_err(RequestError::InvalidJson)
    }

    pub async fn json_value(&self) -> Result<Value, RequestError> {
        let mut cache = self.cached_json.lock().unwrap();

        if cache.is_none() {
            let bytes = self.body_bytes().await?;
            let value = serde_json::from_slice(&bytes).map_err(RequestError::InvalidJson)?;
            *cache = Some(value);
        }

        Ok(cache.as_ref().unwrap().clone())
    }

    pub async fn input(&self, key: &str) -> Option<String> {
        if let Some(val) = self.query(key) {
            return Some(val.clone());
        }

        if self.is_json() {
            if let Ok(json) = self.json_value().await {
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
            if let Ok(form) = self.form_data().await {
                return form.get(key).cloned();
            }
        }

        None
    }

    pub async fn input_or(&self, key: &str, default: &str) -> String {
        self.input(key).await.unwrap_or_else(|| default.to_string())
    }

    pub async fn all_inputs(&self) -> HashMap<String, Value> {
        let mut inputs = HashMap::new();

        for (key, val) in &self.query_params {
            inputs.insert(key.clone(), Value::String(val.clone()));
        }

        if self.is_json() {
            if let Ok(json) = self.json_value().await {
                if let Value::Object(map) = json {
                    for (key, val) in map {
                        inputs.insert(key, val);
                    }
                }
            }
        }

        if self.is_form() {
            if let Ok(form) = self.form_data().await {
                for (key, val) in form {
                    inputs.insert(key, Value::String(val));
                }
            }
        }

        inputs
    }

    pub async fn only(&self, keys: &[&str]) -> HashMap<String, Value> {
        let all = self.all_inputs().await;
        keys.iter()
            .filter_map(|&key| all.get(key).map(|v| (key.to_string(), v.clone())))
            .collect()
    }

    pub async fn except(&self, keys: &[&str]) -> HashMap<String, Value> {
        let all = self.all_inputs().await;
        all.into_iter()
            .filter(|(key, _)| !keys.contains(&key.as_str()))
            .collect()
    }

    pub async fn form_data(&self) -> Result<HashMap<String, String>, RequestError> {
        let mut cache = self.cached_form_data.lock().unwrap();

        if cache.is_none() {
            let body_str = self.body_as_string().await?;
            let data: HashMap<String, String> = url::form_urlencoded::parse(body_str.as_bytes())
                .into_owned()
                .collect();
            *cache = Some(data);
        }

        Ok(cache.as_ref().unwrap().clone())
    }

    pub async fn validate<F: super::form_request::traits::FormRequest + Default>(
        &self,
        _form_request: &F,
    ) -> Result<HashMap<String, Value>, RequestError> {
        let data = self.all_inputs().await;
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
