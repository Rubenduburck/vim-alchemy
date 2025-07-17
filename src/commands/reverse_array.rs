use crate::cli::Response;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ReverseArrayCommand {
    /// Depth of reversal
    #[arg(short, long, default_value = "1")]
    pub depth: u64,
    /// The array to reverse
    pub input: String,
}

impl SubCommand for ReverseArrayCommand {
    fn run(&self, _list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        match client.reverse_array(&self.input, self.depth as usize) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        }
    }
}