pub mod array;
pub mod classify;
pub mod classify_and_convert;
pub mod classify_and_hash;
pub mod convert;
pub mod generate;
pub mod hash;
pub mod pad;
pub mod random;

use crate::types::CliResult;

pub trait SubCommand {
    fn run(&self, list_mode: bool, input: Option<&str>) -> CliResult;
}

