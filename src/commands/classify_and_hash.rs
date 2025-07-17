use crate::types::{CliResult, Response, ToResponse};
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ClassifyAndHashCommand {
    /// Hash algorithm(s)
    #[arg(short, long, value_delimiter = ',')]
    pub algo: Vec<String>,
}

impl SubCommand for ClassifyAndHashCommand {
    fn run(&self, list_mode: bool, input: Option<&str>) -> CliResult {
        let input = match input {
            Some(i) => i,
            None => return Error::MissingArgs("input".to_string()).into(),
        };
        let client = Client::new();
        
        client.classify_and_hash(self.algo.clone(), input)
            .map(|output| {
                if !list_mode && self.algo.len() == 1 {
                    // Single algorithm in non-list mode: return just the hash
                    output.get(&self.algo[0])
                        .map(|hash| Response::from(hash.clone()))
                        .unwrap_or_else(|| Response::from("Unsupported hash algorithm"))
                } else {
                    // Multiple algorithms or list mode: return full structure
                    output.to_response()
                }
            })
            .into()
    }
}
