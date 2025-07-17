use crate::encode::decoding::Decoded;
use crate::error::Error;
use std::fmt::Display;

/// Core trait for all encoders
pub trait Encoder: Display + Send + Sync {
    /// Encode the decoded input into a string representation
    fn encode(&self, input: &Decoded) -> Result<String, Error>;
    
    /// Get the name of this encoding
    fn name(&self) -> &str;
}

/// Trait for encoders that support padding
pub trait PaddableEncoder: Encoder {
    /// Encode with padding support
    fn encode_with_padding(&self, input: &Decoded, pad: bool) -> Result<String, Error>;
}

/// Core trait for all decoders  
pub trait Decoder: Send + Sync {
    /// Decode a string input into the Decoded type
    fn decode(&self, input: &str) -> Result<Decoded, Error>;
    
    /// Check if the input can be decoded by this decoder
    fn can_decode(&self, input: &str) -> bool;
}

/// Trait for types that can be created from a specification string
pub trait EncodingSpec: Sized {
    /// Parse a specification string into an encoding instance
    fn from_spec(spec: &str) -> Result<Self, Error>;
    
    /// Convert this encoding to a specification string
    fn to_spec(&self) -> String;
}

/// Trait for encodings that can be composed with others
pub trait ComposableEncoding: Encoder {
    /// Compose this encoding with another
    fn compose(&self, other: Box<dyn Encoder>) -> Box<dyn Encoder>;
}

/// Trait for encodings that can be transformed
pub trait TransformableEncoding {
    /// Flatten nested structures
    fn flatten(&self) -> Self;
    
    /// Convert to a line-based representation
    fn to_lines(&self) -> Self;
}

/// Combined trait for types that can both encode and decode
pub trait Codec: Encoder + Decoder {
    /// Round-trip encode and decode for validation
    fn validate_round_trip(&self, input: &Decoded) -> Result<bool, Error> {
        let encoded = self.encode(input)?;
        let decoded = self.decode(&encoded)?;
        Ok(input == &decoded)
    }
}