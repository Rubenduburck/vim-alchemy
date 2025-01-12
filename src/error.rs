use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("neovim_lib error: {0}")]
    NeovimError(#[from] neovim_lib::CallError),

    #[error("Encode error: {0}")]
    EncodeError(#[from] crate::encode::error::Error),

    #[error("Value error: {0}")]
    ValueError(String),

    #[error("Var error: {0}")]
    VarError(#[from] std::env::VarError),
}
