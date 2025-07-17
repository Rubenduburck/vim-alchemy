#![allow(clippy::uninlined_format_args)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Encode error: {0}")]
    Encode(#[from] crate::encode::error::Error),

    #[error("Value error: {0}")]
    Value(String),

    #[error("Var error: {0}")]
    Var(#[from] std::env::VarError),

    #[error("Missing args {0}")]
    MissingArgs(String),

    #[error("Unknown request {0}")]
    UnknownRequest(String),

    #[error("Invalid args {0}")]
    InvalidArgs(String),
}

impl From<Error> for crate::value::Value {
    fn from(e: Error) -> Self {
        crate::value::Value::from(format!("{}", e))
    }
}
