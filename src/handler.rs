use std::collections::HashMap;

use neovim_lib::{RequestHandler, Value};

use crate::client::Client;
use crate::error::Error;
use crate::{get_array, get_param};

#[derive(Debug, Clone)]
pub struct Selection {
    pub text: String,
    pub start_line: u64,
    pub start_col: u64,
    pub end_line: u64,
    pub end_col: u64,
}

impl TryFrom<Value> for Selection {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let map = value
            .as_map()
            .ok_or(Error::InvalidArgs("selection".to_string()))?;
        Ok(Selection {
            text: get_param(map, "text")?,
            start_line: get_param(map, "start_line")?,
            start_col: get_param(map, "start_col")?,
            end_line: get_param(map, "end_line")?,
            end_col: get_param(map, "end_col")?,
        })
    }
}

#[derive(Debug)]
pub enum Request {
    ClassifyAndConvert {
        output_encoding: String,
        selection: Selection,
    },
    Classify {
        selection: Selection,
    },
    Convert {
        input_encoding: Vec<String>,
        output_encoding: Vec<String>,
        selection: Selection,
    },
    FlattenArray {
        selection: Selection,
    },
    ChunkArray {
        chunk_count: u64,
        selection: Selection,
    },
    ReverseArray {
        depth: u64,
        selection: Selection,
    },
    RotateArray {
        rotation: i64,
        selection: Selection,
    },
    Generate {
        encoding: String,
        bytes: u64,
    },
    Random {
        encoding: String,
        bytes: u64,
    },
    PadLeft {
        padding: u64,
        selection: Selection,
    },
    PadRight {
        padding: u64,
        selection: Selection,
    },
    Stop,
    Hash {
        algo: Vec<String>,
        selection: Selection,
        input_encoding: Vec<String>,
    },
    Unknown(Vec<Value>),
    Setup(crate::client::Config),
}

impl Request {
    const CLASSIFY_AND_CONVERT: &'static str = "classify_and_convert";
    const CLASSIFY: &'static str = "classify";
    const CONVERT: &'static str = "convert";
    const FLATTEN_ARRAY: &'static str = "flatten_array";
    const CHUNK_ARRAY: &'static str = "chunk_array";
    const REVERSE_ARRAY: &'static str = "reverse_array";
    const ROTATE_ARRAY: &'static str = "rotate_array";
    const GENERATE: &'static str = "generate";
    const RANDOM: &'static str = "random";
    const PAD_LEFT: &'static str = "pad_left";
    const PAD_RIGHT: &'static str = "pad_right";
    const STOP: &'static str = "stop";
    const HASH: &'static str = "hash";
    const SETUP: &'static str = "setup";
}

impl TryFrom<(&str, Vec<Value>)> for Request {
    type Error = Error;

    fn try_from(value: (&str, Vec<Value>)) -> Result<Self, Self::Error> {
        let (method, params) = value;
        let params = params
            .first()
            .and_then(Value::as_map)
            .ok_or(Error::InvalidArgs("params".to_string()))?;
        match method {
            Self::CONVERT => Ok(Request::Convert {
                input_encoding: get_array(params, "input_encoding")?,
                output_encoding: get_array(params, "output_encoding")?,
                selection: get_param(params, "selection")?,
            }),
            Self::CLASSIFY => Ok(Request::Classify {
                selection: get_param(params, "selection")?,
            }),
            Self::CLASSIFY_AND_CONVERT => Ok(Request::ClassifyAndConvert {
                output_encoding: get_param(params, "output_encoding")?,
                selection: get_param(params, "selection")?,
            }),
            Self::FLATTEN_ARRAY => Ok(Request::FlattenArray {
                selection: get_param(params, "selection")?,
            }),
            Self::CHUNK_ARRAY => Ok(Request::ChunkArray {
                chunk_count: get_param(params, "chunk_count")?,
                selection: get_param(params, "selection")?,
            }),
            Self::REVERSE_ARRAY => Ok(Request::ReverseArray {
                depth: get_param(params, "depth")?,
                selection: get_param(params, "selection")?,
            }),
            Self::ROTATE_ARRAY => Ok(Request::RotateArray {
                rotation: get_param(params, "rotation")?,
                selection: get_param(params, "selection")?,
            }),
            Self::GENERATE => Ok(Request::Generate {
                encoding: get_param(params, "encoding")?,
                bytes: get_param(params, "bytes")?,
            }),
            Self::RANDOM => Ok(Request::Random {
                encoding: get_param(params, "encoding")?,
                bytes: get_param(params, "bytes")?,
            }),
            Self::PAD_LEFT => Ok(Request::PadLeft {
                padding: get_param(params, "padding")?,
                selection: get_param(params, "selection")?,
            }),
            Self::PAD_RIGHT => Ok(Request::PadRight {
                padding: get_param(params, "padding")?,
                selection: get_param(params, "selection")?,
            }),
            Self::STOP => Ok(Request::Stop),
            Self::HASH => Ok(Request::Hash {
                algo: get_array(params, "algo")?,
                selection: get_param(params, "selection")?,
                input_encoding: get_array(params, "input_encoding")?,
            }),
            Self::SETUP => Ok(Request::Setup(get_param(params, "config")?)),
            _ => Err(Error::UnknownRequest(method.to_string())),
        }
    }
}

pub struct Handler {
    client: Client,
}

#[derive(Debug)]
struct Conversions(HashMap<String, HashMap<String, Conversion>>);

#[derive(Debug)]
pub struct Conversion {
    pub input: String,
    pub output: String,
}

impl From<Conversion> for Value {
    fn from(value: Conversion) -> Self {
        Value::Map(
            vec![
                (Value::from("input"), Value::from(value.input)),
                (Value::from("output"), Value::from(value.output)),
            ]
            .into_iter()
            .collect(),
        )
    }
}

impl From<Conversions> for Value {
    fn from(value: Conversions) -> Self {
        Value::Map(
            value
                .0
                .into_iter()
                .map(|(k, v)| {
                    (
                        Value::from(k),
                        Value::Map(
                            v.into_iter()
                                .map(|(k, v)| (Value::from(k), Value::from(v)))
                                .collect::<Vec<(Value, Value)>>(),
                        ),
                    )
                })
                .collect::<Vec<(Value, Value)>>(),
        )
    }
}

#[derive(Debug)]
pub struct Hashing {
    pub output: String,
}

impl From<Hashing> for Value {
    fn from(value: Hashing) -> Self {
        Value::Map(
            vec![(Value::from("output"), Value::from(value.output))]
                .into_iter()
                .collect(),
        )
    }
}

#[derive(Debug)]
struct Hashings(HashMap<String, HashMap<String, Hashing>>);

impl From<Hashings> for Value {
    fn from(value: Hashings) -> Self {
        Value::Map(
            value
                .0
                .into_iter()
                .map(|(k, v)| {
                    (
                        Value::from(k),
                        Value::Map(
                            v.into_iter()
                                .map(|(k, v)| (Value::from(k), Value::from(v)))
                                .collect::<Vec<(Value, Value)>>(),
                        ),
                    )
                })
                .collect::<Vec<(Value, Value)>>(),
        )
    }
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            client: Client::new(),
        }
    }

    /// Classify the given input
    /// E.g. classify "0x1234" -> "hex"
    fn handle_classify(&mut self, input: &str) -> Result<Value, Error> {
        tracing::info!("Classify");
        let mut classifications = self.client.classify(input);
        classifications.retain(|c| !c.is_empty());
        classifications.sort();
        Ok(Value::from(
            classifications
                .iter()
                .map(Value::from)
                .collect::<Vec<Value>>(),
        ))
    }

    /// Convert given classification
    /// E.g. convert "0x1234" "base64" -> "MTIzNA=="
    /// Given an array of output_encodings, return a map of the results
    fn handle_convert(
        &mut self,
        input_encoding: Vec<String>,
        output_encoding: Vec<String>,
        input: &str,
    ) -> Result<Conversions, Error> {
        tracing::info!("Convert");
        tracing::debug!("input_encoding: {:?}", input_encoding);
        tracing::debug!("output_encoding: {:?}", output_encoding);
        let result = input_encoding
            .iter()
            .flat_map(|encoding| {
                self.client
                    .decode(encoding, input)
                    .map(|decoded| {
                        output_encoding
                            .iter()
                            .flat_map(|output_encoding| {
                                self.client
                                    .encode(output_encoding, &decoded)
                                    .map(|encoded| {
                                        (
                                            output_encoding.clone(),
                                            Conversion {
                                                input: decoded.to_string(),
                                                output: encoded,
                                            },
                                        )
                                    })
                            })
                            .collect::<HashMap<String, Conversion>>()
                    })
                    .map(|results| (encoding.clone(), results))
            })
            .collect::<HashMap<String, HashMap<String, Conversion>>>();
        Ok(Conversions(result))
    }

    /// Classify the given input
    /// Then convert the input to the provided encoding
    /// E.g. classify_and_convert "0x1234" "bytes" -> "[0x12, 0x34]"
    fn handle_classify_and_convert(&mut self, encoding: &str, input: &str) -> Result<Value, Error> {
        tracing::info!("Classify and convert");
        self.client
            .classify_and_convert(encoding, input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Flatten the given array
    /// E.g. flatten_array "[[1, 2], [3, 4]]" -> "[1, 2, 3, 4]"
    fn handle_flatten_array(&mut self, input: &str) -> Result<Value, Error> {
        tracing::info!("Flatten array");
        self.client
            .flatten_array(input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Chunk the given array
    /// E.g. chunk_array 2 "[1, 2, 3, 4, 5, 6]" -> "[[1, 2, 3], [4, 5, 6]]"
    fn handle_chunk_array(&mut self, chunk_count: u64, input: &str) -> Result<Value, Error> {
        tracing::info!("Chunk array");
        self.client
            .chunk_array(chunk_count as usize, input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Reverse the given array
    /// E.g. reverse_array 2 "[1, 2, 3, 4, 5, 6]" -> "[5, 4, 3, 2, 1]"
    fn handle_reverse_array(&mut self, depth: u64, input: &str) -> Result<Value, Error> {
        tracing::info!("Reverse array");
        self.client
            .reverse_array(input, depth as usize)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Rotate the given array
    /// E.g. rotate_array 2 "[1, 2, 3, 4, 5, 6]" -> "[5, 6, 1, 2, 3, 4]"
    fn handle_rotate_array(&mut self, rotation: i64, input: &str) -> Result<Value, Error> {
        tracing::info!("Rotate array");
        self.client
            .rotate_array(input, rotation as isize)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Generate an empty in the given encoding for the given number of bytes
    /// E.g. generate "bytes" 4 -> "[0x00, 0x00, 0x00, 0x00]"
    /// E.g. generate "hex" 4 -> "0x00000000"
    /// E.g. generate "int" 4 -> "00000000"
    fn handle_generate(&mut self, encoding: &str, number: u64) -> Result<Value, Error> {
        tracing::info!("New");
        self.client
            .generate(encoding, number as usize)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Generate a random value in the given encoding for the given number of bytes
    /// E.g. random "bytes" 4 -> "[0x12, 0x34, 0x56, 0x78]"
    /// E.g. random "hex" 4 -> "0x12345678"
    fn handle_random(&mut self, encoding: &str, number: u64) -> Result<Value, Error> {
        tracing::info!("Random");
        self.client
            .random(encoding, number as usize)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Pad the given input to the left to the given bytes
    /// E.g. pad_left 4 "0x1234" -> "0x00001234"
    /// E.g. pad_left 4 "[1, 2]" -> "[0x00, 0x00, 0x01, 0x02]"
    fn handle_pad_left(&mut self, padding: u64, input: &str) -> Result<Value, Error> {
        tracing::info!("Pad");
        self.client
            .pad_left(padding as usize, input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Pad the given input to the right to the given bytes
    /// E.g. pad_right 4 "0x1234" -> "0x12340000"
    /// E.g. pad_right 4 "[1, 2]" -> "[0x01, 0x02, 0x00, 0x00]"
    fn handle_pad_right(&mut self, padding: u64, input: &str) -> Result<Value, Error> {
        tracing::info!("Pad");
        self.client
            .pad_right(padding as usize, input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Hash the given input
    /// If the input is classified as some type, hash the bytes
    /// otherwise, hash as utf8
    fn handle_hash(
        &mut self,
        algo: Vec<String>,
        input: &str,
        input_encoding: Vec<String>,
    ) -> Result<Value, Error> {
        tracing::info!("Hash");
        let results = input_encoding
            .iter()
            .flat_map(|encoding| {
                let values = algo
                    .iter()
                    .flat_map(|algo| {
                        self.client.hash(algo, input, encoding).map(|output| {
                            (
                                algo.clone(),
                                Hashing {
                                    output: output.to_string(),
                                },
                            )
                        })
                    })
                    .collect::<HashMap<String, Hashing>>();
                if values.is_empty() {
                    None
                } else {
                    Some((encoding.clone(), values))
                }
            })
            .collect::<HashMap<String, HashMap<String, Hashing>>>();
        Ok(Hashings(results).into())
    }
}

impl Default for Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestHandler for Handler {
    fn handle_request(&mut self, name: &str, args: Vec<Value>) -> Result<Value, Value> {
        tracing::debug!("Handling request {}: {:?}", name, args);
        let request = Request::try_from((name, args))?;
        tracing::debug!("Parsed request: {:?}", request);
        let result = match request {
            Request::Classify { selection } => self.handle_classify(&selection.text),
            Request::Convert {
                input_encoding,
                output_encoding,
                selection,
            } => self
                .handle_convert(input_encoding, output_encoding, &selection.text)
                .map(Value::from),
            Request::ClassifyAndConvert {
                ref output_encoding,
                selection,
            } => self.handle_classify_and_convert(output_encoding, &selection.text),
            Request::FlattenArray { selection } => self.handle_flatten_array(&selection.text),
            Request::ChunkArray {
                chunk_count,
                selection,
            } => self.handle_chunk_array(chunk_count, &selection.text),
            Request::ReverseArray { depth, selection } => {
                self.handle_reverse_array(depth, &selection.text)
            }
            Request::RotateArray {
                rotation,
                selection,
            } => self.handle_rotate_array(rotation, &selection.text),
            Request::Generate {
                ref encoding,
                bytes,
            } => self.handle_generate(encoding, bytes),
            Request::Random {
                ref encoding,
                bytes,
            } => self.handle_random(encoding, bytes),
            Request::PadLeft { padding, selection } => {
                self.handle_pad_left(padding, &selection.text)
            }
            Request::PadRight { padding, selection } => {
                self.handle_pad_right(padding, &selection.text)
            }
            Request::Stop => {
                tracing::info!("Stopping");
                std::process::exit(0);
            }
            Request::Hash {
                algo,
                selection,
                input_encoding,
            } => self.handle_hash(algo, &selection.text, input_encoding),
            Request::Unknown(values) => {
                Err(Error::UnknownRequest(format!("{}, {:?}", name, values)))
            }
            Request::Setup(config) => {
                tracing::info!("Setting up client");
                self.client.setup(config);
                Ok(Value::Nil)
            }
        };
        tracing::debug!("Result: {:?}", result);
        result.map_err(|e| e.into())
    }
}

impl neovim_lib::Handler for Handler {
    fn handle_notify(&mut self, name: &str, args: Vec<Value>) {
        tracing::debug!("Handling notify {}: {:?}", name, args);
    }
}
