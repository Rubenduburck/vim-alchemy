use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::Error;

pub struct CliResult(pub Result<Response, Error>);

impl std::ops::Deref for CliResult {
    type Target = Result<Response, Error>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Result<String, Error>> for CliResult {
    fn from(value: Result<String, Error>) -> Self {
        match value {
            Ok(s) => CliResult(Ok(Response::String(s))),
            Err(e) => CliResult(Err(e)),
        }
    }
}

impl From<Result<Response, Error>> for CliResult {
    fn from(value: Result<Response, Error>) -> Self {
        CliResult(value)
    }
}

impl From<Result<HashMap<String, String>, Error>> for CliResult {
    fn from(value: Result<HashMap<String, String>, Error>) -> Self {
        match value {
            Ok(map) => CliResult(Ok(Response::Json(serde_json::to_value(map).unwrap()))),
            Err(e) => CliResult(Err(e)),
        }
    }
}

// Specific From implementations for commonly used types
impl From<String> for CliResult {
    fn from(s: String) -> Self {
        CliResult(Ok(Response::String(s)))
    }
}

impl From<Response> for CliResult {
    fn from(r: Response) -> Self {
        CliResult(Ok(r))
    }
}

impl From<Vec<ClassificationResult>> for CliResult {
    fn from(classifications: Vec<ClassificationResult>) -> Self {
        CliResult(Ok(Response::Classifications(classifications)))
    }
}

impl From<Vec<EncodingWithDecodings>> for CliResult {
    fn from(encodings: Vec<EncodingWithDecodings>) -> Self {
        CliResult(Ok(Response::Conversions(ConversionResponse::Full { encodings })))
    }
}

impl From<HashMap<String, Vec<String>>> for CliResult {
    fn from(decodings: HashMap<String, Vec<String>>) -> Self {
        CliResult(Ok(Response::Json(serde_json::to_value(decodings).unwrap())))
    }
}

impl From<HashMap<String, HashMap<String, ConversionResult>>> for CliResult {
    fn from(conversions: HashMap<String, HashMap<String, ConversionResult>>) -> Self {
        CliResult(Ok(Response::Conversions(ConversionResponse::Regular(conversions))))
    }
}

impl From<Option<String>> for CliResult {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => CliResult(Ok(Response::String(s))),
            None => CliResult(Err(Error::Generic("No value provided".to_string()))),
        }
    }
}

impl From<Error> for CliResult {
    fn from(e: Error) -> Self {
        CliResult(Err(e))
    }
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
    Regular(HashMap<String, HashMap<String, ConversionResult>>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodingWithDecodings {
    pub encoding: String,
    pub score: usize,
    pub decodings: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HashResponse {
    Single(String),
    Multiple(HashMap<String, HashMap<String, HashResult>>),
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

impl From<HashMap<String, HashMap<String, ConversionResult>>> for Response {
    fn from(conversions: HashMap<String, HashMap<String, ConversionResult>>) -> Self {
        Response::Conversions(ConversionResponse::Regular(conversions))
    }
}

impl From<HashMap<String, Vec<String>>> for Response {
    fn from(decodings: HashMap<String, Vec<String>>) -> Self {
        Response::Json(serde_json::to_value(decodings).unwrap())
    }
}

impl From<Vec<EncodingWithDecodings>> for Response {
    fn from(encodings: Vec<EncodingWithDecodings>) -> Self {
        Response::Conversions(ConversionResponse::Full { encodings })
    }
}

impl From<Vec<ClassificationResult>> for Response {
    fn from(classifications: Vec<ClassificationResult>) -> Self {
        Response::Classifications(classifications)
    }
}

impl From<ConversionResponse> for Response {
    fn from(conversions: ConversionResponse) -> Self {
        Response::Conversions(conversions)
    }
}

impl From<HashResponse> for Response {
    fn from(hash: HashResponse) -> Self {
        Response::Hash(hash)
    }
}

impl From<String> for Response {
    fn from(s: String) -> Self {
        Response::String(s)
    }
}

impl From<&str> for Response {
    fn from(s: &str) -> Self {
        Response::String(s.to_string())
    }
}

impl From<serde_json::Value> for Response {
    fn from(value: serde_json::Value) -> Self {
        Response::Json(value)
    }
}


pub trait ToResponse {
    fn to_response(self) -> Response;
}

impl<T: Serialize> ToResponse for T {
    fn to_response(self) -> Response {
        Response::Json(serde_json::to_value(self).unwrap())
    }
}

impl TryFrom<Error> for Response {
    type Error = ();

    fn try_from(e: Error) -> Result<Self, Self::Error> {
        Ok(Response::String(format!("Error: {}", e)))
    }
}