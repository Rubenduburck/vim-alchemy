#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("neovim_lib error: {0}")]
    Neovim(#[from] neovim_lib::CallError),

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

impl From<Error> for neovim_lib::Value {
    fn from(e: Error) -> Self {
        neovim_lib::Value::from(format!("{}", e))
    }
}
