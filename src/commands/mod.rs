pub mod classify;
pub mod convert;
pub mod classify_and_convert;
pub mod flatten_array;
pub mod chunk_array;
pub mod reverse_array;
pub mod rotate_array;
pub mod generate;
pub mod random;
pub mod pad_left;
pub mod pad_right;
pub mod hash;
pub mod classify_and_hash;

use crate::cli::Response;
use crate::error::Error;

pub trait SubCommand {
    fn run(&self, list_mode: bool) -> Result<Response, Error>;
}