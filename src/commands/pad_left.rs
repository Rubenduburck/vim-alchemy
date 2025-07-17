use crate::cli::Response;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct PadLeftCommand {
    /// Padding size in bytes
    #[arg(short, long)]
    pub padding: u64,
    /// The input to pad
    pub input: String,
}

impl SubCommand for PadLeftCommand {
    fn run(&self, _list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        match client.pad_left(self.padding as usize, &self.input) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        }
    }
}