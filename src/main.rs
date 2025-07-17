use clap::Parser;
use std::collections::HashMap;
use alchemy::cli::{Cli, Commands, ConversionResult, HashResult, Response, ClassificationResult, ConversionResponse, HashResponse, EncodingWithDecodings};
use alchemy::client::Client;
use alchemy::encode::encoding::Encoding;
use alchemy::error::Error;

fn main() {
    let cli = Cli::parse();
    let client = Client::new();
    let list_mode = cli.list;
    
    let result = match cli.command {
        Commands::Classify { input } => {
            let mut classifications = client.classify(&input);
            classifications.retain(|c| !c.is_empty());
            classifications.sort(); // Sorts by score (ascending) then by encoding
            
            if list_mode {
                // Return all classifications with scores
                let results: Vec<ClassificationResult> = classifications
                    .iter()
                    .map(|c| ClassificationResult {
                        encoding: c.encoding().to_string(),
                        score: c.score(),
                    })
                    .collect();
                Ok(Response::Classifications(results))
            } else {
                // Return only the best match (lowest score)
                if let Some(best) = classifications.first() {
                    Ok(Response::String(best.encoding().to_string()))
                } else {
                    Ok(Response::String("Empty".to_string()))
                }
            }
        }
        Commands::Convert {
            input_encoding,
            output_encoding,
            input,
        } => {
            let output_encoding = output_encoding.unwrap_or_default();
            let was_input_encoding_none = input_encoding.is_none();
            
            // Determine input encodings
            let (encodings, _classifications) = match input_encoding {
                Some(encodings) => (encodings, None),
                None => {
                    let mut classifications = client.classify(&input);
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
                    if let Ok(decoded) = client.decode(&Encoding::from(encoding_str), &input) {
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
                    if let Ok(decoded) = client.decode(&Encoding::from(encoding), &input) {
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
                        if let Ok(decoded) = client.decode(&Encoding::from(encoding), &input) {
                            if let Ok(encoded) = client.encode(&Encoding::from(output_enc), &decoded) {
                                result_string = Some(encoded);
                                break;
                            }
                        }
                    }
                    
                    match result_string {
                        Some(encoded) => Ok(Response::String(encoded)),
                        None => Err(Error::Encode(alchemy::encode::error::Error::UnsupportedEncoding)),
                    }
                } else {
                    // Multiple encodings or list mode: return full JSON structure
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
                    Ok(Response::Conversions(ConversionResponse::Regular(result)))
                }
            }
        }
        Commands::ClassifyAndConvert {
            output_encoding,
            input,
        } => {
            if !list_mode && output_encoding.len() == 1 {
                // Single output encoding in non-list mode: return just the string
                let output_enc = &output_encoding[0];
                let output_encodings = vec![Encoding::from(output_enc)];
                match client.classify_and_convert(output_encodings, &input) {
                    Ok(result) => {
                        // Get the first (and only) value from the hashmap
                        if let Some(value) = result.get(output_enc) {
                            Ok(Response::String(value.clone()))
                        } else {
                            Err(Error::Encode(alchemy::encode::error::Error::UnsupportedEncoding))
                        }
                    }
                    Err(e) => Err(Error::from(e)),
                }
            } else {
                // Multiple output encodings or list mode: return full structure
                let output_encodings = output_encoding.iter().map(Encoding::from).collect();
                match client.classify_and_convert(output_encodings, &input) {
                    Ok(output) => Ok(Response::Json(serde_json::to_value(output).unwrap())),
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
            if !list_mode && algo.len() == 1 && input_encoding.len() == 1 {
                // Single algorithm, single encoding, non-list mode: return just the hash
                let algo_name = &algo[0];
                let encoding = &input_encoding[0];
                match client.hash(algo_name, &input, Encoding::from(encoding)) {
                    Ok(hash) => Ok(Response::String(hash.to_string())),
                    Err(e) => Err(Error::from(e)),
                }
            } else {
                // Multiple algorithms/encodings or list mode: return full structure
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
                Ok(Response::Hash(HashResponse::Multiple(results)))
            }
        }
        Commands::ClassifyAndHash { algo, input } => {
            match client.classify_and_hash(algo.clone(), &input) {
                Ok(output) => {
                    if !list_mode && algo.len() == 1 {
                        // Single algorithm in non-list mode: return just the hash
                        if let Some(hash) = output.get(&algo[0]) {
                            Ok(Response::String(hash.clone()))
                        } else {
                            Err(Error::Encode(alchemy::encode::error::Error::UnsupportedHash))
                        }
                    } else {
                        // Multiple algorithms or list mode: return full structure
                        Ok(Response::Json(serde_json::to_value(output).unwrap()))
                    }
                }
                Err(e) => Err(e),
            }
        }
    };

    match result {
        Ok(response) => {
            match response {
                Response::String(s) => println!("{}", s),
                Response::Classifications(classifications) => {
                    println!("{}", serde_json::to_string(&classifications).unwrap())
                }
                Response::Conversions(conversions) => {
                    println!("{}", serde_json::to_string(&conversions).unwrap())
                }
                Response::Hash(hash) => {
                    println!("{}", serde_json::to_string(&hash).unwrap())
                }
                Response::Json(json) => {
                    println!("{}", serde_json::to_string(&json).unwrap())
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}