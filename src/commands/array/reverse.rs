use crate::types::CliResult;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;
use std::io::{self, Read};

#[derive(Args)]
pub struct ReverseCommand {
    /// Depth of reversal
    #[arg(short, long, default_value = "1")]
    pub depth: u64,
    /// Input data to reverse
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    input: Vec<String>,
}

impl SubCommand for ReverseCommand {
    fn run(&self, _list_mode: bool) -> CliResult {
        let input = if self.input.is_empty() {
            // Read from stdin if no arguments provided
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => buffer.trim().to_string(),
                Err(e) => return Error::Generic(format!("Failed to read from stdin: {}", e)).into(),
            }
        } else {
            self.input.join(" ")
        };
        
        if input.is_empty() {
            return Error::MissingArgs("No input provided".to_string()).into();
        }
        let client = Client::new();
        client.reverse_array(&input, self.depth as usize).into()
    }
}