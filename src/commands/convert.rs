use crate::cli::{ConversionResult, ConversionResponse, EncodingWithDecodings, Response};
use crate::client::Client;
use crate::commands::SubCommand;
use crate::encode::encoding::Encoding;
use crate::error::Error;
use std::collections::HashMap;
use clap::Args;

#[derive(Args)]
pub struct ConvertCommand {
    /// Input encoding(s) - if not specified, will auto-classify
    #[arg(short, long, value_delimiter = ',')]
    pub input_encoding: Option<Vec<String>>,
    /// Output encoding(s) - if not specified, will return all possible decodings
    #[arg(short, long, value_delimiter = ',')]
    pub output_encoding: Option<Vec<String>>,
    /// The input text
    pub input: String,
}

impl SubCommand for ConvertCommand {
    fn run(&self, list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        let output_encoding = self.output_encoding.clone().unwrap_or_default();
        let was_input_encoding_none = self.input_encoding.is_none();
        
        // Determine input encodings
        let (encodings, _classifications) = match &self.input_encoding {
            Some(encodings) => (encodings.clone(), None),
            None => {
                let mut classifications = client.classify(&self.input);
                classifications.retain(|c| !c.is_empty());
                classifications.sort();
                let encoding_strings: Vec<String> = classifications.iter().map(|c| c.encoding().to_string()).collect();
                (encoding_strings, Some(classifications))
            }
        };
        
        // Handle special cases
        if was_input_encoding_none && output_encoding.is_empty() {
            // No input and no output encoding provided: show encodings with scores and all their possible decodings
            let classifications = _classifications.unwrap();
            let mut encodings_with_decodings = Vec::new();
            
            // Get all available output encodings from the client
            let all_encodings = vec!["hex", "base64", "utf8", "int", "bin", "base58"];
            
            for (classification, encoding_str) in classifications.iter().zip(&encodings) {
                let mut decodings = HashMap::new();
                
                // Try to decode with this encoding
                if let Ok(decoded) = client.decode(&Encoding::from(encoding_str), &self.input) {
                    // Try to encode the decoded value to all possible output encodings
                    for output_enc in &all_encodings {
                        if let Ok(encoded) = client.encode(&Encoding::from(*output_enc), &decoded) {
                            decodings.insert(output_enc.to_string(), encoded);
                        }
                    }
                }
                
                encodings_with_decodings.push(EncodingWithDecodings {
                    encoding: classification.encoding().to_string(),
                    score: classification.score(),
                    decodings,
                });
            }
            
            Ok(Response::Conversions(ConversionResponse::Full {
                encodings: encodings_with_decodings,
            }))
        } else if output_encoding.is_empty() {
            // No output encoding provided: return list of possible decodings
            let mut decodings: HashMap<String, Vec<String>> = HashMap::new();
            for encoding in &encodings {
                if let Ok(decoded) = client.decode(&Encoding::from(encoding), &self.input) {
                    decodings.insert(encoding.clone(), vec![decoded.to_string()]);
                }
            }
            Ok(Response::Json(serde_json::to_value(decodings).unwrap()))
        } else {
            // Normal conversion with output encodings specified
            if !list_mode && output_encoding.len() == 1 {
                // Non-list mode with single output: use best input encoding
                let output_enc = &output_encoding[0];
                
                // Try each encoding in order (sorted by score) and use the first successful one
                let mut result_string = None;
                for encoding in &encodings {
                    if let Ok(decoded) = client.decode(&Encoding::from(encoding), &self.input) {
                        if let Ok(encoded) = client.encode(&Encoding::from(output_enc), &decoded) {
                            result_string = Some(encoded);
                            break;
                        }
                    }
                }
                
                match result_string {
                    Some(encoded) => Ok(Response::String(encoded)),
                    None => Err(Error::Encode(crate::encode::error::Error::UnsupportedEncoding)),
                }
            } else {
                // Multiple encodings or list mode: return full JSON structure
                let result = encodings
                    .iter()
                    .flat_map(|encoding| {
                        client
                            .decode(&Encoding::from(encoding), &self.input)
                            .ok()
                            .map(|decoded| {
                                let conversions = output_encoding
                                    .iter()
                                    .flat_map(|output_encoding| {
                                        client
                                            .encode(&Encoding::from(output_encoding), &decoded)
                                            .ok()
                                            .map(|encoded| {
                                                (
                                                    output_encoding.clone(),
                                                    ConversionResult {
                                                        input: decoded.to_string(),
                                                        output: encoded,
                                                    },
                                                )
                                            })
                                    })
                                    .collect::<HashMap<String, ConversionResult>>();
                                (encoding.clone(), conversions)
                            })
                    })
                    .collect::<HashMap<String, HashMap<String, ConversionResult>>>();
                Ok(Response::Conversions(ConversionResponse::Regular(result)))
            }
        }
    }
}