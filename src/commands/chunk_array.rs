use crate::cli::Response;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ChunkArrayCommand {
    /// Number of chunks to create
    #[arg(short, long)]
    pub chunks: u64,
    /// The array to chunk
    pub input: String,
}

impl SubCommand for ChunkArrayCommand {
    fn run(&self, _list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        match client.chunk_array(self.chunks as usize, &self.input) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        }
    }
}