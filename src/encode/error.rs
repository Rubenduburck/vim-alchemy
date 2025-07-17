#![allow(clippy::uninlined_format_args)]
use crate::value::Value;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("base64 error: {0}")]
    FromBase64Error(#[from] base64::DecodeError),
    #[error("rug parsing error {0}")]
    RugParseIntegerError(#[from] rug::integer::ParseIntegerError),
    #[error("bs58 parsing error {0}")]
    Bs58Error(#[from] bs58::decode::Error),
    #[error("Unsupported base: {0}")]
    UnsupportedBase(i32),
    #[error("Unsupported Encoding")]
    UnsupportedEncoding,

    #[error("Missing input")]
    MissingInput,
    #[error("Missing encoding")]
    MissingEncoding,

    #[error("Unsupported hash")]
    UnsupportedHash,
}

impl From<Error> for Value {
    fn from(e: Error) -> Self {
        Value::from(format!("{}", e))
    }
}
