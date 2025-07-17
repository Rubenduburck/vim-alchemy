use crate::types::{CliResult, Response, ToResponse};
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
}

impl SubCommand for ClassifyAndConvertCommand {
    fn run(&self, list_mode: bool, input: Option<&str>) -> CliResult {
        let input = match input {
            Some(i) => i,
            None => return Error::MissingArgs("input".to_string()).into(),
        };
        let client = Client::new();

        if !list_mode && self.output_encoding.len() == 1 {
            // Single output encoding in non-list mode: return just the string
            let output_enc = &self.output_encoding[0];
            let output_encodings = vec![Encoding::from(output_enc)];
            client
                .classify_and_convert(output_encodings, input)
                .map(|result| {
                    result
                        .get(output_enc)
                        .map(|value| Response::from(value.clone()))
                        .unwrap_or_else(|| Response::from("Unsupported encoding"))
                })
                .into()
        } else {
            let output_encodings = self.output_encoding.iter().map(Encoding::from).collect();
            client
                .classify_and_convert(output_encodings, input)
                .map(|output| output.to_response())
                .into()
        }
    }
}
