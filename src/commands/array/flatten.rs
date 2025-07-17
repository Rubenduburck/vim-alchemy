use crate::types::CliResult;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct FlattenCommand {
}

impl SubCommand for FlattenCommand {
    fn run(&self, _list_mode: bool, input: Option<&str>) -> CliResult {
        match input {
            Some(input) => {
                let client = Client::new();
                client.flatten_array(input).into()
            }
            None => Error::MissingArgs("No input provided".to_string()).into()
        }
    }
}