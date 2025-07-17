use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "alchemy")]
#[command(about = "A CLI tool for encoding, decoding, and data transformation")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Classify the input encoding
    Classify {
        /// The input text to classify
        input: String,
    },
    /// Convert between encodings (auto-classifies if input encoding not specified)
    Convert {
        /// Input encoding(s) - if not specified, will auto-classify
        #[arg(short, long, value_delimiter = ',')]
        input_encoding: Option<Vec<String>>,
        /// Output encoding(s) 
        #[arg(short, long, value_delimiter = ',')]
        output_encoding: Vec<String>,
        /// The input text
        input: String,
    },
    /// Classify input and convert to specified encodings
    ClassifyAndConvert {
        /// Output encoding(s)
        #[arg(short, long, value_delimiter = ',')]
        output_encoding: Vec<String>,
        /// The input text
        input: String,
    },
    /// Flatten a nested array
    FlattenArray {
        /// The array to flatten
        input: String,
    },
    /// Chunk an array into groups
    ChunkArray {
        /// Number of chunks to create
        #[arg(short, long)]
        chunks: u64,
        /// The array to chunk
        input: String,
    },
    /// Reverse an array
    ReverseArray {
        /// Depth of reversal
        #[arg(short, long, default_value = "1")]
        depth: u64,
        /// The array to reverse
        input: String,
    },
    /// Rotate an array
    RotateArray {
        /// Rotation amount (negative for left, positive for right)
        #[arg(short, long)]
        rotation: i64,
        /// The array to rotate
        input: String,
    },
    /// Generate empty data in specified encoding
    Generate {
        /// Encoding type
        #[arg(short, long)]
        encoding: String,
        /// Number of bytes
        #[arg(short, long)]
        bytes: u64,
    },
    /// Generate random data in specified encoding
    Random {
        /// Encoding type
        #[arg(short, long)]
        encoding: String,
        /// Number of bytes
        #[arg(short, long)]
        bytes: u64,
    },
    /// Pad data to the left
    PadLeft {
        /// Padding size in bytes
        #[arg(short, long)]
        padding: u64,
        /// The input to pad
        input: String,
    },
    /// Pad data to the right
    PadRight {
        /// Padding size in bytes
        #[arg(short, long)]
        padding: u64,
        /// The input to pad
        input: String,
    },
    /// Hash the input using specified algorithm(s)
    Hash {
        /// Hash algorithm(s)
        #[arg(short, long, value_delimiter = ',')]
        algo: Vec<String>,
        /// Input encoding(s)
        #[arg(short, long, value_delimiter = ',')]
        input_encoding: Vec<String>,
        /// The input to hash
        input: String,
    },
    /// Classify input and hash using specified algorithm(s)
    ClassifyAndHash {
        /// Hash algorithm(s)
        #[arg(short, long, value_delimiter = ',')]
        algo: Vec<String>,
        /// The input to hash
        input: String,
    },
}

// Response types for JSON output
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Classifications(Vec<String>),
    Conversions(std::collections::HashMap<String, std::collections::HashMap<String, ConversionResult>>),
    ClassifyAndConvert(std::collections::HashMap<String, String>),
    String(String),
    Hash(std::collections::HashMap<String, std::collections::HashMap<String, HashResult>>),
    ClassifyAndHash(std::collections::HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashResult {
    pub output: String,
}