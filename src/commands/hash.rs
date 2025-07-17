use crate::types::{CliResult, HashResponse, HashResult, Response};
use crate::client::Client;
use crate::commands::SubCommand;
use crate::encode::encoding::Encoding;
use crate::error::Error;
use std::collections::HashMap;
use clap::Args;

#[derive(Args)]
pub struct HashCommand {
    /// Hash algorithm(s)
    #[arg(short, long, value_delimiter = ',')]
    pub algo: Vec<String>,
    /// Input encoding(s)
    #[arg(short, long, value_delimiter = ',')]
    pub input_encoding: Vec<String>,
}

impl SubCommand for HashCommand {
    fn run(&self, list_mode: bool, input: Option<&str>) -> CliResult {
        let input = match input {
            Some(i) => i,
            None => return Error::MissingArgs("input".to_string()).into(),
        };
        let client = Client::new();
        
        if !list_mode && self.algo.len() == 1 && self.input_encoding.len() == 1 {
            // Single algorithm, single encoding, non-list mode: return just the hash
            let algo_name = &self.algo[0];
            let encoding = &self.input_encoding[0];
            client.hash(algo_name, input, Encoding::from(encoding))
                .map(|hash| hash.to_string())
                .into()
        } else {
            // Multiple algorithms/encodings or list mode: return full structure
            let results = self.input_encoding
                .iter()
                .flat_map(|encoding| {
                    let values = self.algo
                        .iter()
                        .flat_map(|algo| {
                            client
                                .hash(algo, input, Encoding::from(encoding))
                                .ok()
                                .map(|output| {
                                    (
                                        algo.clone(),
                                        HashResult {
                                            output: output.to_string(),
                                        },
                                    )
                                })
                        })
                        .collect::<HashMap<String, HashResult>>();
                    if values.is_empty() {
                        None
                    } else {
                        Some((encoding.clone(), values))
                    }
                })
                .collect::<HashMap<String, HashMap<String, HashResult>>>();
            Response::Hash(HashResponse::Multiple(results)).into()
        }
    }
}