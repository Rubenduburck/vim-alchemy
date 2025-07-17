use crate::cli::Response;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::encode::encoding::Encoding;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct ClassifyAndConvertCommand {
    /// Output encoding(s)
    #[arg(short, long, value_delimiter = ',')]
    pub output_encoding: Vec<String>,
    /// The input text
    pub input: String,
}

impl SubCommand for ClassifyAndConvertCommand {
    fn run(&self, list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        
        if !list_mode && self.output_encoding.len() == 1 {
            // Single output encoding in non-list mode: return just the string
            let output_enc = &self.output_encoding[0];
            let output_encodings = vec![Encoding::from(output_enc)];
            client.classify_and_convert(output_encodings, &self.input)
                .map(|result| {
                    if let Some(value) = result.get(output_enc) {
                        Response::String(value.clone())
                    } else {
                        Response::String("Unsupported encoding".to_string())
                    }
                })
        } else {
            let output_encodings = self.output_encoding.iter().map(Encoding::from).collect();
            match client.classify_and_convert(output_encodings, &self.input) {
                Ok(output) => Ok(Response::Json(serde_json::to_value(output).unwrap())),
                Err(e) => Err(Error::from(e)),
            }
        }
    }
}
