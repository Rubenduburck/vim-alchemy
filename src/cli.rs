#![allow(clippy::uninlined_format_args)]
use crate::commands::SubCommand as _;
use crate::commands::{
    array::ArrayCommand, classify::ClassifyCommand,
    convert::ConvertCommand, generate::GenerateCommand,
    hash::HashCommand, pad::Pad, random::Random,
};
use crate::types::CliResult;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "alchemy")]
#[command(about = "A CLI tool for encoding, decoding, and data transformation")]
#[command(version)]
pub struct Cli {
    /// Return all results in JSON format with scores
    #[arg(short, long, global = true)]
    pub list: bool,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn run(&self) -> CliResult {
        let list_mode = self.list;
        match &self.command {
            Commands::Array(cmd) => cmd.run(list_mode),
            Commands::Classify(cmd) => cmd.run(list_mode),
            Commands::Convert(cmd) => cmd.run(list_mode),
            Commands::Generate(cmd) => cmd.run(list_mode),
            Commands::Random(cmd) => cmd.run(list_mode),
            Commands::Pad(cmd) => cmd.run(list_mode),
            Commands::Hash(cmd) => cmd.run(list_mode),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Array manipulation commands
    Array(ArrayCommand),
    /// Classify input encoding
    Classify(ClassifyCommand),
    /// Convert between encodings (auto-classifies if input encoding not specified)
    Convert(ConvertCommand),
    /// Generate random data
    Generate(GenerateCommand),
    /// Generate random data
    Random(Random),
    /// Pad data
    Pad(Pad),
    /// Hash data (auto-classifies if input encoding not specified)
    Hash(HashCommand),
}

