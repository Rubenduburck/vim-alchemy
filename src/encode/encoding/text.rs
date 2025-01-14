use super::{super::decoding::Decoded, super::error::Error};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_ascii() {
        let text_encoding = TextEncoding::Ascii;
        let decoded = Decoded::Bytes(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]);
        assert_eq!(text_encoding.encode(&decoded).unwrap(), "Hello");
    }

    #[test]
    fn test_encode_utf8() {
        let text_encoding = TextEncoding::Utf(8);
        let decoded = Decoded::Bytes(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]);
        assert_eq!(text_encoding.encode(&decoded).unwrap(), "Hello");
    }

    #[test]
    fn test_encode_utf16() {
        let text_encoding = TextEncoding::Utf(16);
        let decoded = Decoded::Bytes(vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00]);
        assert_eq!(text_encoding.encode(&decoded).unwrap(), "Hello");
    }
}
