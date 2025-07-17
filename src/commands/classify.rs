use crate::types::{ClassificationResult, CliResult};
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ClassifyCommand {
}

impl SubCommand for ClassifyCommand {
    fn run(&self, list_mode: bool, input: Option<&str>) -> CliResult {
        let input = match input {
            Some(i) => i,
            None => return Error::MissingArgs("input".to_string()).into(),
        };
        let client = Client::new();
        let mut classifications = client.classify(input);
        classifications.retain(|c| !c.is_empty());
        classifications.sort(); // Sorts by score (ascending) then by encoding

        if list_mode {
            let results: Vec<ClassificationResult> = classifications
                .iter()
                .map(|c| ClassificationResult {
                    encoding: c.encoding().to_string(),
                    score: c.score(),
                })
                .collect();
            results.into()
        } else if let Some(best) = classifications.first() {
            best.encoding().to_string().into()
        } else {
            Error::Generic("No classifications found".to_string()).into()
        }
    }
}

