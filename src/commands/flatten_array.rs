use crate::cli::Response;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct FlattenArrayCommand {
    /// The array to flatten
    pub input: String,
}

impl SubCommand for FlattenArrayCommand {
    fn run(&self, _list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        match client.flatten_array(&self.input) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        }
    }
}