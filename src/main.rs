use clap::Parser;
use std::collections::HashMap;
use alchemy::cli::{Cli, Commands, ConversionResult, HashResult, Response};
use alchemy::client::Client;
use alchemy::encode::encoding::Encoding;
use alchemy::error::Error;

fn main() {
    let cli = Cli::parse();
    let client = Client::new();
    
    let result = match cli.command {
        Commands::Classify { input } => {
            let mut classifications = client.classify(&input);
            classifications.retain(|c| !c.is_empty());
            classifications.sort();
            let classification_strings: Vec<String> = classifications
                .iter()
                .map(|c| c.to_string())
                .collect();
            Ok(Response::Classifications(classification_strings))
        }
        Commands::Convert {
            input_encoding,
            output_encoding,
            input,
        } => {
            // If no input encoding specified, classify first
            let encodings = match input_encoding {
                Some(encodings) => encodings,
                None => {
                    let mut classifications = client.classify(&input);
                    classifications.retain(|c| !c.is_empty());
                    classifications.into_iter().map(|c| c.to_string()).collect()
                }
            };
            
            // For simple case: single input, single output encoding
            if encodings.len() == 1 && output_encoding.len() == 1 {
                let input_enc = &encodings[0];
                let output_enc = &output_encoding[0];
                
                match client.decode(&Encoding::from(input_enc), &input) {
                    Ok(decoded) => {
                        match client.encode(&Encoding::from(output_enc), &decoded) {
                            Ok(encoded) => Ok(Response::String(encoded)),
                            Err(e) => Err(Error::from(e)),
                        }
                    }
                    Err(e) => Err(Error::from(e)),
                }
            } else {
                // Multiple encodings: return full JSON structure
                let result = encodings
                    .iter()
                    .flat_map(|encoding| {
                        client
                            .decode(&Encoding::from(encoding), &input)
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
                Ok(Response::Conversions(result))
            }
        }
        Commands::ClassifyAndConvert {
            output_encoding,
            input,
        } => {
            // If single output encoding, return just the string
            if output_encoding.len() == 1 {
                let output_enc = &output_encoding[0];
                let output_encodings = vec![Encoding::from(output_enc)];
                match client.classify_and_convert(output_encodings, &input) {
                    Ok(result) => {
                        // Get the first (and only) value from the hashmap
                        if let Some(value) = result.get(output_enc) {
                            Ok(Response::String(value.clone()))
                        } else {
                            Err(Error::Encode(crate::encode::error::Error::InvalidEncoding("No conversion result found".to_string())))
                        }
                    }
                    Err(e) => Err(Error::from(e)),
                }
            } else {
                // Multiple output encodings: return full structure
                let output_encodings = output_encoding.iter().map(Encoding::from).collect();
                match client.classify_and_convert(output_encodings, &input) {
                    Ok(output) => Ok(Response::ClassifyAndConvert(output)),
                    Err(e) => Err(Error::from(e)),
                }
            }
        }
        Commands::FlattenArray { input } => match client.flatten_array(&input) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        },
        Commands::ChunkArray { chunks, input } => {
            match client.chunk_array(chunks as usize, &input) {
                Ok(output) => Ok(Response::String(output)),
                Err(e) => Err(Error::from(e)),
            }
        }
        Commands::ReverseArray { depth, input } => {
            match client.reverse_array(&input, depth as usize) {
                Ok(output) => Ok(Response::String(output)),
                Err(e) => Err(Error::from(e)),
            }
        }
        Commands::RotateArray { rotation, input } => {
            match client.rotate_array(&input, rotation as isize) {
                Ok(output) => Ok(Response::String(output)),
                Err(e) => Err(Error::from(e)),
            }
        }
        Commands::Generate { encoding, bytes } => {
            match client.generate(&encoding, bytes as usize) {
                Ok(output) => Ok(Response::String(output)),
                Err(e) => Err(Error::from(e)),
            }
        }
        Commands::Random { encoding, bytes } => match client.random(&encoding, bytes as usize) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        },
        Commands::PadLeft { padding, input } => match client.pad_left(padding as usize, &input) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        },
        Commands::PadRight { padding, input } => match client.pad_right(padding as usize, &input) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        },
        Commands::Hash {
            algo,
            input_encoding,
            input,
        } => {
            let results = input_encoding
                .iter()
                .flat_map(|encoding| {
                    let values = algo
                        .iter()
                        .flat_map(|algo| {
                            client
                                .hash(algo, &input, Encoding::from(encoding))
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
            Ok(Response::Hash(results))
        }
        Commands::ClassifyAndHash { algo, input } => {
            match client.classify_and_hash(algo, &input) {
                Ok(output) => Ok(Response::ClassifyAndHash(output)),
                Err(e) => Err(e),
            }
        }
    };

    match result {
        Ok(response) => {
            match response {
                Response::String(s) => println!("{}", s),
                _ => println!("{}", serde_json::to_string(&response).unwrap()),
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}