use neovim_lib::{Neovim, NeovimApi, Session, Value};

use crate::{client::Client, encode::error::Error};

use tracing::{error, info};

pub enum Message {
    ClassifyAndConvert,
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

pub struct EventHandler {
    nvim: Neovim,
    client: Client,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            nvim: Neovim::new(Session::new_parent().unwrap_or_else(|e| {
                error!("Failed to create nvim session: {}", e);
                panic!();
            })),
            client: Client::new(),
        }
    }

    pub fn escape(message: &str) -> String {
        const SPECIAL_CHARS: &str = "^$*+?.()|{}[]";
        message
            .chars()
            .fold(vec![], |mut acc, c| {
                if c == '\n' {
                    acc.extend(c.escape_default())
                } else if SPECIAL_CHARS.contains(c) {
                    acc.extend(['\\', c])
                } else {
                    acc.push(c)
                };
                acc
            })
            .into_iter()
            .collect()
    }

    pub fn substitute(&mut self, from: &str, to: &str) -> Result<(), Error> {
        let from = Self::escape(from);
        let to = Self::escape(to);
        let cmd = format!("'<,'>s/{}/{}", from, to,);
        info!(
            "replacing {} with message {} with command {}",
            from, to, cmd
        );
        Ok(self.nvim.command(&cmd)?)
    }

    pub fn replace(&mut self, from: &str, to: &str) -> Result<(), Error> {
        info!("replacing {} with message {}", from, to);
        self.substitute(from, to)?;
        self.position_cursor_before_selection()
    }

    pub fn put_after_cursor(&mut self, _message: &str) -> Result<(), Error> {
        info!("putting message at cursor");
        Ok(self.nvim.command("normal a")?)
    }

    pub fn position_cursor_before_selection(&mut self) -> Result<(), Error> {
        info!("positioning cursor before selection");
        Ok(self.nvim.command("normal '<")?)
    }

    pub fn recv(&mut self) {
        info!("Starting event loop");
        let receiver = self.nvim.session.start_event_loop_channel();

        info!("Receiving events");
        for (event, values) in receiver {
            match Message::from(event) {
                Message::Stop => {
                    info!("Stopping");
                    break;
                }
                message => self.handle(message, values),
            }
        }
    }

    pub fn handle(&mut self, message: Message, values: Vec<Value>) {
        info!("message received");
        match message {
            Message::ClassifyAndConvert => self.handle_classify_and_convert(values),
            Message::FlattenArray => self.handle_flatten_array(values),
            Message::ChunkArray => self.handle_chunk_array(values),
            Message::Unknown(event) => error!("Unknown event: {}", event),
            Message::ReverseArray => self.handle_reverse_array(values),
            Message::RotateArray => self.handle_rotate_array(values),
            Message::Generate => self.handle_generate(values),
            Message::Random => self.handle_random(values),
            Message::PadLeft => self.handle_pad_left(values),
            Message::PadRight => self.handle_pad_right(values),
            Message::Hash => self.handle_hash(values),
            _ => {}
        }
    }

    /// Classify the given input
    /// Then convert the input to the provided encoding
    /// E.g. classify_and_convert "0x1234" "bytes" -> "[0x12, 0x34]"
    fn handle_classify_and_convert(&mut self, values: Vec<Value>) {
        info!("Classify and convert");
        let mut args = values.iter();
        let encoding = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        info!("encoding: {}", encoding);
        let input = match args.next() {
            Some(encoding) => encoding.as_str().expect("Error: Invalid encoding"),
            None => {
                error!("Error: No encoding provided");
                return;
            }
        };
        info!("input: {}", input);
        match self.client.classify_and_convert(encoding, input) {
            Ok(result) => {
                if let Err(e) = self.replace(input, &result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Flatten the given array
    /// E.g. flatten_array "[[1, 2], [3, 4]]" -> "[1, 2, 3, 4]"
    fn handle_flatten_array(&mut self, values: Vec<Value>) {
        info!("Flatten array");
        let mut args = values.iter();
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        info!("input: {}", input);
        match self.client.flatten_array(input) {
            Ok(result) => {
                if let Err(e) = self.replace(input, &result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Chunk the given array
    /// E.g. chunk_array 2 "[1, 2, 3, 4, 5, 6]" -> "[[1, 2, 3], [4, 5, 6]]"
    fn handle_chunk_array(&mut self, values: Vec<Value>) {
        info!("Chunk array");
        let mut args = values.iter();
        let chunk_count = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<u32>()
                .expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        info!("chunk_count: {}", chunk_count);
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        info!("input: {}", input);
        match self.client.chunk_array(chunk_count as usize, input) {
            Ok(result) => {
                if let Err(e) = self.replace(input, &result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Reverse the given array
    /// E.g. reverse_array 2 "[1, 2, 3, 4, 5, 6]" -> "[5, 4, 3, 2, 1]"
    fn handle_reverse_array(&mut self, values: Vec<Value>) {
        info!("Reverse array");
        let mut args = values.iter();
        let depth = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<u32>()
                .expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        info!("input: {}", input);
        match self.client.reverse_array(input, depth as usize) {
            Ok(result) => {
                if let Err(e) = self.replace(input, &result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Rotate the given array
    /// E.g. rotate_array 2 "[1, 2, 3, 4, 5, 6]" -> "[5, 6, 1, 2, 3, 4]"
    fn handle_rotate_array(&mut self, values: Vec<Value>) {
        info!("Rotate array");
        let mut args = values.iter();
        let rotation = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<isize>()
                .expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        info!("input: {}", input);
        match self.client.rotate_array(input, rotation) {
            Ok(result) => {
                if let Err(e) = self.replace(input, &result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Generate an empty in the given encoding for the given number of bytes
    /// E.g. generate "bytes" 4 -> "[0x00, 0x00, 0x00, 0x00]"
    /// E.g. generate "hex" 4 -> "0x00000000"
    /// E.g. generate "int" 4 -> "00000000"
    fn handle_generate(&mut self, values: Vec<Value>) {
        info!("New");
        let mut args = values.iter();
        let encoding = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        let number = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<usize>()
                .expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        match self.client.generate(encoding, number) {
            Ok(result) => {
                if let Err(e) = self.put_after_cursor(&result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Generate a random value in the given encoding for the given number of bytes
    /// E.g. random "bytes" 4 -> "[0x12, 0x34, 0x56, 0x78]"
    /// E.g. random "hex" 4 -> "0x12345678"
    fn handle_random(&mut self, values: Vec<Value>) {
        info!("Random");
        let mut args = values.iter();
        let encoding = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        let number = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<usize>()
                .expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        match self.client.random(encoding, number) {
            Ok(result) => {
                if let Err(e) = self.put_after_cursor(&result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Pad the given input to the left to the given bytes
    /// E.g. pad_left 4 "0x1234" -> "0x00001234"
    /// E.g. pad_left 4 "[1, 2]" -> "[0x00, 0x00, 0x01, 0x02]"
    fn handle_pad_left(&mut self, values: Vec<Value>) {
        info!("Pad");
        let mut args = values.iter();
        let padding = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<usize>()
                .expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        match self.client.pad_left(padding, input) {
            Ok(result) => {
                if let Err(e) = self.replace(input, &result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Pad the given input to the right to the given bytes
    /// E.g. pad_right 4 "0x1234" -> "0x12340000"
    /// E.g. pad_right 4 "[1, 2]" -> "[0x01, 0x02, 0x00, 0x00]"
    fn handle_pad_right(&mut self, values: Vec<Value>) {
        info!("Pad");
        let mut args = values.iter();
        let padding = match args.next() {
            Some(input) => input
                .as_str()
                .expect("Error: Invalid input")
                .parse::<usize>()
                .expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        match self.client.pad_right(padding, input) {
            Ok(result) => {
                if let Err(e) = self.replace(input, &result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    /// Hash the given input
    /// If the input is classified as some type, hash the bytes
    /// otherwise, hash as utf8
    fn handle_hash(&mut self, values: Vec<Value>) {
        info!("Hash");
        let mut args = values.iter();
        let algo = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        let input = match args.next() {
            Some(input) => input.as_str().expect("Error: Invalid input"),
            None => {
                error!("Error: No input provided");
                return;
            }
        };
        match self.client.hash(algo, input) {
            Ok(result) => {
                if let Err(e) = self.replace(input, &result) {
                    error!("Error: {}", e)
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::EventHandler;

    #[test]
    fn test_escape() {
        let test = "[1\n2\n3]";
        let escaped = EventHandler::escape(test);
        assert_eq!(escaped, "\\[1\\n2\\n3\\]");
    }
}
