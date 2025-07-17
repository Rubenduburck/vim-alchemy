use crate::cli::{ClassificationResult, Response};
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ClassifyCommand {
    /// The input text to classify
    pub input: String,
}

impl SubCommand for ClassifyCommand {
    fn run(&self, list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        let mut classifications = client.classify(&self.input);
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
            Ok(Response::Classifications(results))
        } else if let Some(best) = classifications.first() {
            Ok(Response::String(best.encoding().to_string()))
        } else {
            Ok(Response::String("Empty".to_string()))
        }
    }
}
