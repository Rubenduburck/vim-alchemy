use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use crate::commands::{
    classify::ClassifyCommand,
    convert::ConvertCommand,
    classify_and_convert::ClassifyAndConvertCommand,
    flatten_array::FlattenArrayCommand,
    chunk_array::ChunkArrayCommand,
    reverse_array::ReverseArrayCommand,
    rotate_array::RotateArrayCommand,
    generate::GenerateCommand,
    random::RandomCommand,
    pad_left::PadLeftCommand,
    pad_right::PadRightCommand,
    hash::HashCommand,
    classify_and_hash::ClassifyAndHashCommand,
};

#[derive(Parser)]
#[command(name = "alchemy")]
#[command(about = "A CLI tool for encoding, decoding, and data transformation")]
#[command(version)]
pub struct Cli {
    /// Return all results in JSON format with scores
    #[arg(short, long, global = true)]
    pub list: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Classify(ClassifyCommand),
    Convert(ConvertCommand),
    ClassifyAndConvert(ClassifyAndConvertCommand),
    FlattenArray(FlattenArrayCommand),
    ChunkArray(ChunkArrayCommand),
    ReverseArray(ReverseArrayCommand),
    RotateArray(RotateArrayCommand),
    Generate(GenerateCommand),
    Random(RandomCommand),
    PadLeft(PadLeftCommand),
    PadRight(PadRightCommand),
    Hash(HashCommand),
    ClassifyAndHash(ClassifyAndHashCommand),
}

// Response types for JSON output
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    String(String),
    Classifications(Vec<ClassificationResult>),
    Conversions(ConversionResponse),
    Hash(HashResponse),
    Json(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub encoding: String,
    pub score: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConversionResponse {
    Full {
        encodings: Vec<EncodingWithDecodings>,
    },
    Regular(std::collections::HashMap<String, std::collections::HashMap<String, ConversionResult>>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodingWithDecodings {
    pub encoding: String,
    pub score: usize,
    pub decodings: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HashResponse {
    Single(String),
    Multiple(std::collections::HashMap<String, std::collections::HashMap<String, HashResult>>),
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
