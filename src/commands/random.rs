use crate::cli::Response;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct RandomCommand {
    /// Encoding type
    #[arg(short, long)]
    pub encoding: String,
    /// Number of bytes
    #[arg(short, long)]
    pub bytes: u64,
}

impl SubCommand for RandomCommand {
    fn run(&self, _list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        match client.random(&self.encoding, self.bytes as usize) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        }
    }
}