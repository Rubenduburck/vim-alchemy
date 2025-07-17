use crate::types::CliResult;
use crate::commands::SubCommand;
use clap::{Args, Subcommand};

pub mod flatten;
pub mod chunk;
pub mod reverse;
pub mod rotate;

use flatten::FlattenCommand;
use chunk::ChunkCommand;
use reverse::ReverseCommand;
use rotate::RotateCommand;

#[derive(Args)]
pub struct ArrayCommand {
    #[command(subcommand)]
    pub command: ArrayCommands,
}

#[derive(Subcommand)]
pub enum ArrayCommands {
    /// Flatten a nested array
    Flatten(FlattenCommand),
    /// Chunk an array into specified number of chunks
    Chunk(ChunkCommand),
    /// Reverse an array at specified depth
    Reverse(ReverseCommand),
    /// Rotate an array left or right
    Rotate(RotateCommand),
}

impl SubCommand for ArrayCommand {
    fn run(&self, list_mode: bool, input: Option<&str>) -> CliResult {
        match &self.command {
            ArrayCommands::Flatten(cmd) => cmd.run(list_mode, input),
            ArrayCommands::Chunk(cmd) => cmd.run(list_mode, input),
            ArrayCommands::Reverse(cmd) => cmd.run(list_mode, input),
            ArrayCommands::Rotate(cmd) => cmd.run(list_mode, input),
        }
    }
}