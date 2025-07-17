use crate::types::CliResult;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ReverseCommand {
    /// Depth of reversal
    #[arg(short, long, default_value = "1")]
    pub depth: u64,
}

impl SubCommand for ReverseCommand {
    fn run(&self, _list_mode: bool, input: Option<&str>) -> CliResult {
        match input {
            Some(input) => {
                let client = Client::new();
                client.reverse_array(input, self.depth as usize).into()
            }
            None => Error::MissingArgs("No input provided".to_string()).into()
        }
    }
}