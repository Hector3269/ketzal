use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    BindError(std::io::Error),
    AcceptError(std::io::Error),
    RuntimeError(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::BindError(e) => write!(f, "Failed to bind: {}", e),
            ServerError::AcceptError(e) => write!(f, "Failed to accept connection: {}", e),
            ServerError::RuntimeError(s) => write!(f, "Runtime error: {}", s),
        }
    }
}

impl std::error::Error for ServerError {}

#[derive(Debug)]
pub enum HttpError {
    RegistryPoisoned,
    InternalError(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::RegistryPoisoned => write!(f, "Route registry is poisoned"),
            HttpError::InternalError(s) => write!(f, "Internal error: {}", s),
        }
    }
}

impl std::error::Error for HttpError {}
