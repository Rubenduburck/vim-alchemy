use crate::types::CliResult;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ChunkCommand {
    /// Number of chunks to create
    #[arg(short, long)]
    pub chunks: u64,
}

impl SubCommand for ChunkCommand {
    fn run(&self, _list_mode: bool, input: Option<&str>) -> CliResult {
        match input {
            Some(input) => {
                let client = Client::new();
                client.chunk_array(self.chunks as usize, input).into()
            }
            None => Error::MissingArgs("No input provided".to_string()).into()
        }
    }
}