use http::{HeaderMap, Method};
use std::collections::HashMap;


#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
  
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn new(
        method: Method,
        path: String,
        query: HashMap<String, String>,
        headers: HeaderMap,
        body: Vec<u8>,
        params: HashMap<String, String>,
    ) -> Self {
        
        Self {
            method,
            path,
            query,
            headers,
            body,
            params,
        }
    }

   
}
