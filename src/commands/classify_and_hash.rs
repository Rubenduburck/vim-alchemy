use crate::cli::Response;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ClassifyAndHashCommand {
    /// Hash algorithm(s)
    #[arg(short, long, value_delimiter = ',')]
    pub algo: Vec<String>,
    /// The input to hash
    pub input: String,
}

impl SubCommand for ClassifyAndHashCommand {
    fn run(&self, list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        
        match client.classify_and_hash(self.algo.clone(), &self.input) {
            Ok(output) => {
                if !list_mode && self.algo.len() == 1 {
                    // Single algorithm in non-list mode: return just the hash
                    if let Some(hash) = output.get(&self.algo[0]) {
                        Ok(Response::String(hash.clone()))
                    } else {
                        Err(Error::Encode(crate::encode::error::Error::UnsupportedHash))
                    }
                } else {
                    // Multiple algorithms or list mode: return full structure
                    Ok(Response::Json(serde_json::to_value(output).unwrap()))
                }
            }
            Err(e) => Err(e),
        }
    }
}