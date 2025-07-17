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
    /// Input data to chunk
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    input: Vec<String>,
}

impl SubCommand for ChunkCommand {
    fn run(&self, _list_mode: bool) -> CliResult {
        if self.input.is_empty() {
            return Error::MissingArgs("No input provided".to_string()).into();
        }
        let input = self.input.join(" ");
        let client = Client::new();
        client.chunk_array(self.chunks as usize, &input).into()
    }
}