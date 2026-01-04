use std::fmt;

#[derive(Debug)]
pub enum KernelError {
    Config(String),
    Container(String),
    Execution(String),
    Io(std::io::Error),
    Other(String),
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::Config(msg) => write!(f, "Configuration error: {}", msg),
            KernelError::Container(msg) => write!(f, "Container error: {}", msg),
            KernelError::Execution(msg) => write!(f, "Execution error: {}", msg),
            KernelError::Io(err) => write!(f, "IO error: {}", err),
            KernelError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for KernelError {}

impl From<std::io::Error> for KernelError {
    fn from(err: std::io::Error) -> Self {
        KernelError::Io(err)
    }
}

pub type KernelResult<T> = Result<T, KernelError>;
