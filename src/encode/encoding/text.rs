use super::{super::decoding::Decoded, super::error::Error};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextEncoding {
    Utf(u8),
    Ascii,
}

impl std::fmt::Display for TextEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextEncoding::Utf(n) => write!(f, "utf{}", n),
            TextEncoding::Ascii => write!(f, "ascii"),
        }
    }
}

impl TextEncoding {
    pub fn all() -> Vec<Self> {
        vec![Self::Utf(8), Self::Utf(16), Self::Ascii]
    }
    pub fn encode(&self, v: &Decoded) -> Result<String, Error> {
        match self {
            TextEncoding::Utf(8) | TextEncoding::Ascii => {
                Ok(String::from_utf8_lossy(&v.to_le_bytes()).to_string())
            }
            TextEncoding::Utf(16) => {
                let utf_16_bytes: Vec<u16> = v
                    .to_le_bytes()
                    .chunks(2)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .enumerate()
                            .map(|(i, b)| {
                                u16::from(*b) * if i == 1 { 1 } else { u16::from(u8::MAX) }
                            })
                            .sum()
                    })
                    .collect();
                Ok(String::from_utf16_lossy(&utf_16_bytes).to_string())
            }
            _ => Err(Error::UnsupportedEncoding),
        }
    }
}
