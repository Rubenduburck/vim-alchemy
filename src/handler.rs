use neovim_lib::{RequestHandler, Value};

use crate::client::Client;
use crate::error::Error;

pub enum Message {
    ClassifyAndConvert,
    Classify,
    FlattenArray,
    ChunkArray,
    ReverseArray,
    RotateArray,
    Generate,
    Random,
    PadLeft,
    PadRight,
    Stop,
    Hash,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match event.as_str() {
            "classify_and_convert" => Message::ClassifyAndConvert,
            "classify" => Message::Classify,
            "chunk_array" => Message::ChunkArray,
            "flatten_array" => Message::FlattenArray,
            "reverse_array" => Message::ReverseArray,
            "rotate_array" => Message::RotateArray,
            "generate" => Message::Generate,
            "random" => Message::Random,
            "pad_left" => Message::PadLeft,
            "pad_right" => Message::PadRight,
            "stop" => Message::Stop,
            "hash" => Message::Hash,
            _ => Message::Unknown(event),
        }
    }
}

pub struct Handler {
    client: Client,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            client: Client::new(),
        }
    }

    /// Classify the given input
    /// Then convert the input to the provided encoding
    /// E.g. classify_and_convert "0x1234" "bytes" -> "[0x12, 0x34]"
    fn handle_classify_and_convert(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Classify and convert");
        let mut args = values.iter();
        let encoding = match args.next() {
            Some(encoding) => encoding.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("encoding".to_string()));
            }
        };
        tracing::info!("encoding: {}", encoding);
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid encoding"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        tracing::info!("input: {}", input);
        self.client
            .classify_and_convert(encoding, input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Classify the given input
    /// E.g. classify "0x1234" -> "hex"
    fn handle_classify(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Classify");
        let mut args = values.iter();
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        tracing::info!("input: {}", input);
        Ok(Value::from(
            self.client
                .classify(input)
                .iter()
                .map(Value::from)
                .collect::<Vec<Value>>(),
        ))
    }

    /// Flatten the given array
    /// E.g. flatten_array "[[1, 2], [3, 4]]" -> "[1, 2, 3, 4]"
    fn handle_flatten_array(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Flatten array");
        let mut args = values.iter();
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        tracing::info!("input: {}", input);
        self.client
            .flatten_array(input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Chunk the given array
    /// E.g. chunk_array 2 "[1, 2, 3, 4, 5, 6]" -> "[[1, 2, 3], [4, 5, 6]]"
    fn handle_chunk_array(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Chunk array");
        let mut args = values.iter();
        let chunk_count = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<u32>()
                .expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("chunk_count".to_string()));
            }
        };
        tracing::info!("chunk_count: {}", chunk_count);
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        tracing::info!("input: {}", input);
        self.client
            .chunk_array(chunk_count as usize, input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Reverse the given array
    /// E.g. reverse_array 2 "[1, 2, 3, 4, 5, 6]" -> "[5, 4, 3, 2, 1]"
    fn handle_reverse_array(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Reverse array");
        let mut args = values.iter();
        let depth = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<u32>()
                .expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("depth".to_string()));
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        tracing::info!("input: {}", input);
        self.client
            .reverse_array(input, depth as usize)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Rotate the given array
    /// E.g. rotate_array 2 "[1, 2, 3, 4, 5, 6]" -> "[5, 6, 1, 2, 3, 4]"
    fn handle_rotate_array(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Rotate array");
        let mut args = values.iter();
        let rotation = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<isize>()
                .expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("rotation".to_string()));
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        tracing::info!("input: {}", input);
        self.client
            .rotate_array(input, rotation)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Generate an empty in the given encoding for the given number of bytes
    /// E.g. generate "bytes" 4 -> "[0x00, 0x00, 0x00, 0x00]"
    /// E.g. generate "hex" 4 -> "0x00000000"
    /// E.g. generate "int" 4 -> "00000000"
    fn handle_generate(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("New");
        let mut args = values.iter();
        let encoding = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("encoding".to_string()));
            }
        };
        let number = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<usize>()
                .expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("number".to_string()));
            }
        };
        self.client
            .generate(encoding, number)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Generate a random value in the given encoding for the given number of bytes
    /// E.g. random "bytes" 4 -> "[0x12, 0x34, 0x56, 0x78]"
    /// E.g. random "hex" 4 -> "0x12345678"
    fn handle_random(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Random");
        let mut args = values.iter();
        let encoding = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("encoding".to_string()));
            }
        };
        let number = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<usize>()
                .expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("number".to_string()));
            }
        };
        self.client
            .random(encoding, number)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Pad the given input to the left to the given bytes
    /// E.g. pad_left 4 "0x1234" -> "0x00001234"
    /// E.g. pad_left 4 "[1, 2]" -> "[0x00, 0x00, 0x01, 0x02]"
    fn handle_pad_left(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Pad");
        let mut args = values.iter();
        let padding = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<usize>()
                .expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("padding".to_string()));
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        self.client
            .pad_left(padding, input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Pad the given input to the right to the given bytes
    /// E.g. pad_right 4 "0x1234" -> "0x12340000"
    /// E.g. pad_right 4 "[1, 2]" -> "[0x01, 0x02, 0x00, 0x00]"
    fn handle_pad_right(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Pad");
        let mut args = values.iter();
        let padding = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<usize>()
                .expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("padding".to_string()));
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        self.client
            .pad_right(padding, input)
            .map(Value::from)
            .map_err(Error::from)
    }

    /// Hash the given input
    /// If the input is classified as some type, hash the bytes
    /// otherwise, hash as utf8
    fn handle_hash(&mut self, values: Vec<Value>) -> Result<Value, Error> {
        tracing::info!("Hash");
        let mut args = values.iter();
        let algo = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("algo".to_string()));
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                return Err(Error::MissingArgs("input".to_string()));
            }
        };
        tracing::debug!("algo: {}", algo);
        tracing::debug!("input: {}", input);
        self.client
            .hash(algo, input)
            .map(Value::from)
            .map_err(Error::from)
    }
}

impl Default for Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestHandler for Handler {
    fn handle_request(&mut self, name: &str, args: Vec<Value>) -> Result<Value, Value> {
        tracing::debug!("Handling request: {}", name);
        let result = match name {
            "classify_and_convert" => self.handle_classify_and_convert(args.to_vec()),
            "classify" => self.handle_classify(args.to_vec()),
            "flatten_array" => self.handle_flatten_array(args.to_vec()),
            "chunk_array" => self.handle_chunk_array(args.to_vec()),
            "reverse_array" => self.handle_reverse_array(args.to_vec()),
            "rotate_array" => self.handle_rotate_array(args.to_vec()),
            "generate" => self.handle_generate(args.to_vec()),
            "random" => self.handle_random(args.to_vec()),
            "pad_left" => self.handle_pad_left(args.to_vec()),
            "pad_right" => self.handle_pad_right(args.to_vec()),
            "hash" => self.handle_hash(args.to_vec()),
            _ => Err(Error::UnknownRequest(name.to_string())),
        }
        .map_err(Value::from);
        tracing::debug!("Result: {:?}", result);
        result
    }
}

impl neovim_lib::Handler for Handler {
    fn handle_notify(&mut self, name: &str, args: Vec<Value>) {
        tracing::debug!("Handling notify {}: {:?}", name, args);
    }
}
