use super::{decoding::Decoded, error::Error, hashing::Hasher, types::Separator};

pub mod base;
pub use base::BaseEncoding;

pub mod text;
use neovim_lib::Value;
pub use text::TextEncoding;

pub mod array;
pub use array::ArrayEncoding;

#[derive(Clone, Eq, PartialEq, Debug)]
/// Higher priority first, in case errors are equal.
/// e.g. 1234 can qualify as both a hex string and a decimal string
/// so we have to make a choice here, and the more reasonable choice
/// seems to me that this is a decimal string and not a hex string
pub enum Encoding {
    Text(TextEncoding),
    Base(BaseEncoding),
    Array(ArrayEncoding),
    Empty,
    Hash(Hasher),
}

impl std::fmt::Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Encoding::Text(t) => write!(f, "{}", t),
            Encoding::Base(b) => write!(f, "{}", b),
            Encoding::Array(a) => write!(f, "{}", a),
            Encoding::Empty => write!(f, "Empty"),
            Encoding::Hash(h) => write!(f, "{}", h),
        }
    }
}

impl From<&Encoding> for Value {
    fn from(encoding: &Encoding) -> Value {
        encoding.to_string().into()
    }
}

impl Encoding {
    pub(crate) const INTEGER: &'static str = "int";
    pub(crate) const BINARY: &'static str = "bin";
    pub(crate) const BYTES: &'static str = "bytes";
    pub(crate) const BASE: &'static str = "base";
    pub(crate) const UTF: &'static str = "utf";
    pub(crate) const HEX: &'static str = "hex";
    pub(crate) const ASCII: &'static str = "ascii";

    pub fn to_lines(&self) -> Encoding {
        Encoding::Array(ArrayEncoding::new(
            vec![self.clone()],
            None,
            Some(Separator::from('\n')),
        ))
    }

    pub fn flatten(&self) -> Encoding {
        match self {
            Encoding::Array(a) => Encoding::Array(a.flatten()),
            _ => self.clone(),
        }
    }

    pub fn encode(&self, input: &Decoded, pad: Option<bool>) -> Result<String, Error> {
        match self {
            Encoding::Array(v) => v.encode(input, pad),
            Encoding::Base(n) => n.encode(input, pad),
            Encoding::Text(t) => t.encode(input),
            Encoding::Hash(h) => h.encode(input, pad),
            Encoding::Empty => Ok("".into()),
        }
    }

    pub fn generate(&self, length: usize) -> Result<String, Error> {
        self.encode(&Decoded::from_le_bytes(&vec![0; length]), Some(true))
    }

    pub fn random(&self, length: usize) -> Result<String, Error> {
        self.encode(
            &Decoded::from_le_bytes(
                (0..length)
                    .map(|_| rand::random::<u8>())
                    .collect::<Vec<u8>>()
                    .as_ref(),
            ),
            Some(true),
        )
    }
}

// Some ugly stuff in here, hardcoded priorities
// should probably find a better solution for this
impl Ord for Encoding {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Encoding::Empty, _) => std::cmp::Ordering::Greater,
            (_, Encoding::Empty) => std::cmp::Ordering::Less,
            (Encoding::Array(a), Encoding::Array(b)) => a.cmp(b),
            (Encoding::Base(_), Encoding::Array(_)) => std::cmp::Ordering::Less,
            (Encoding::Array(_), Encoding::Base(_)) => std::cmp::Ordering::Greater,
            (
                Encoding::Base(BaseEncoding { base: a }),
                Encoding::Base(BaseEncoding { base: b }),
            ) => match (*a, *b) {
                (10, _) => std::cmp::Ordering::Less,
                (_, 10) => std::cmp::Ordering::Greater,
                (64, 58) => std::cmp::Ordering::Less,
                (58, 64) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            },
            (Encoding::Hash(_), _) => std::cmp::Ordering::Greater,
            (_, Encoding::Hash(_)) => std::cmp::Ordering::Less,
            (Encoding::Text(_), _) => std::cmp::Ordering::Greater,
            (_, Encoding::Text(_)) => std::cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for Encoding {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&String> for Encoding {
    fn from(s: &String) -> Self {
        Encoding::from(s.as_str())
    }
}

impl From<&str> for Encoding {
    fn from(s: &str) -> Self {
        let s = s.trim().to_lowercase();

        // [hex, int, base12] -> Array([Base(16), Base(10), Base(12)])
        // [hex; 3] -> Array([Base(16), Base(16), Base(16)])
        if s.starts_with('[') && s.ends_with(']') {
            // TODO: make this smoother
            let inner = s[1..s.len() - 1].trim();
            let split = inner.split(';').collect::<Vec<&str>>();
            let values = if split.len() == 2 {
                let count = split[1].trim().parse::<usize>().unwrap_or(1);
                let inner = split[0]
                    .split(',')
                    .map(|e| Encoding::from(e.trim()))
                    .collect::<Vec<_>>();
                let values = inner
                    .iter()
                    .cycle()
                    .take(count)
                    .cloned()
                    .collect::<Vec<Encoding>>();
                values.into()
            } else {
                inner
                    .split(',')
                    .map(|e| Encoding::from(e.trim()))
                    .collect::<Vec<_>>()
                    .into()
            };
            Encoding::Array(values)

        // base64, base-64, base_64, etc -> Base(64)
        } else if let Some(stripped) = s.strip_prefix(Self::BASE) {
            let num_str = stripped
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            Encoding::Base(BaseEncoding::new(num_str.parse::<i32>().unwrap_or(10)))

        // utf8 -> Utf(8)
        } else if let Some(stripped) = s.strip_prefix(Self::UTF) {
            let num_str = stripped
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            Encoding::Text(TextEncoding::Utf(num_str.parse::<u8>().unwrap_or(8)))

        // hex -> Base(16)
        } else {
            match s.as_str() {
                Self::BINARY => Encoding::Base(BaseEncoding::new(2)),
                Self::HEX => Encoding::Base(BaseEncoding::new(16)),
                Self::INTEGER => Encoding::Base(BaseEncoding::new(10)),
                Self::BYTES => Encoding::Array(vec![Encoding::Base(BaseEncoding::new(16))].into()),
                Self::ASCII => Encoding::Text(TextEncoding::Ascii),
                _ => Encoding::Base(BaseEncoding::new(10)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::encode::types::Bracket;

    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Encoding::Base(BaseEncoding::new(2)).to_string(), "bin");
        assert_eq!(Encoding::Base(BaseEncoding::new(10)).to_string(), "int");
        assert_eq!(Encoding::Base(BaseEncoding::new(16)).to_string(), "hex");
        assert_eq!(Encoding::Base(BaseEncoding::new(58)).to_string(), "base58");
        assert_eq!(Encoding::Base(BaseEncoding::new(64)).to_string(), "base64");
        assert_eq!(Encoding::Text(TextEncoding::Utf(8)).to_string(), "utf8");
        assert_eq!(Encoding::Text(TextEncoding::Ascii).to_string(), "ascii");
        assert_eq!(
            Encoding::Array(vec![Encoding::Base(BaseEncoding::new(16))].into()).to_string(),
            "[hex]"
        );
        assert_eq!(
            Encoding::Array(
                vec![
                    Encoding::Base(BaseEncoding::new(16)),
                    Encoding::Base(BaseEncoding::new(10))
                ]
                .into()
            )
            .to_string(),
            "[hex, int]"
        );
    }

    #[test]
    fn test_ordering() {
        let left = Encoding::Base(BaseEncoding::new(10));
        let right = Encoding::Base(BaseEncoding::new(16));
        assert_eq!(left.cmp(&right), std::cmp::Ordering::Less);

        let left = Encoding::Base(BaseEncoding::new(2));
        let right = Encoding::Base(BaseEncoding::new(16));
        assert_eq!(left.cmp(&right), std::cmp::Ordering::Less);

        let left = Encoding::Base(BaseEncoding::new(16));
        let right = Encoding::Empty;
        assert_eq!(left.cmp(&right), std::cmp::Ordering::Less);

        let left = Encoding::Empty;
        let right = Encoding::Base(BaseEncoding::new(16));
        assert_eq!(left.cmp(&right), std::cmp::Ordering::Greater);

        let left = Encoding::Text(TextEncoding::Utf(8));
        let right = Encoding::Array(vec![Encoding::Base(BaseEncoding::new(16))].into());
        assert_eq!(left.cmp(&right), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_ordering_many() {
        let test_input = ["hex", "bin", "base64", "bytes", "int"];
        let expected_output = vec![
            Encoding::Base(BaseEncoding::new(10)),
            Encoding::Base(BaseEncoding::new(2)),
            Encoding::Base(BaseEncoding::new(16)),
            Encoding::Base(BaseEncoding::new(64)),
            Encoding::Array(vec![Encoding::Base(BaseEncoding::new(16))].into()),
        ];
        let mut test_output = test_input
            .iter()
            .map(|x| Encoding::from(*x))
            .collect::<Vec<Encoding>>();
        test_output.sort();
        assert_eq!(test_output, expected_output);
    }

    #[test]
    fn into_encoding() {
        let test_input = [
            "hex",
            "base64",
            "int",
            "bytes",
            "[hex, int]",
            "base58",
            "base3",
            "[hex; 2]",
        ];
        let expected_output = vec![
            Encoding::Base(BaseEncoding::new(16)),
            Encoding::Base(BaseEncoding::new(64)),
            Encoding::Base(BaseEncoding::new(10)),
            Encoding::Array(vec![Encoding::Base(BaseEncoding::new(16))].into()),
            Encoding::Array(
                vec![
                    Encoding::Base(BaseEncoding::new(16)),
                    Encoding::Base(BaseEncoding::new(10)),
                ]
                .into(),
            ),
            Encoding::Base(BaseEncoding::new(58)),
            Encoding::Base(BaseEncoding::new(3)),
            Encoding::Array(
                vec![
                    Encoding::Base(BaseEncoding::new(16)),
                    Encoding::Base(BaseEncoding::new(16)),
                ]
                .into(),
            ),
        ];
        for (i, e) in test_input.iter().zip(expected_output.iter()) {
            assert_eq!(Encoding::from(*i), *e);
        }
    }

    #[test]
    fn test_encode_binary() {
        let test_input = Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]);
        let result = Encoding::Base(BaseEncoding::new(2)).encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "0b1001000110100010101100111100010010000");
    }

    #[test]
    fn test_encode_hex() {
        let test_input = Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]);
        let result = Encoding::Base(BaseEncoding::new(16)).encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "0x1234567890");
    }

    #[test]
    fn test_encode_base_58() {
        let test_input = Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]);
        let result = Encoding::Base(BaseEncoding::new(58)).encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "348ALp7");
    }

    #[test]
    fn test_encode_base_64() {
        let test_input = Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]);
        let result = Encoding::Base(BaseEncoding::new(64)).encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "EjRWeJA");
    }

    #[test]
    fn test_encode_integer() {
        let test_input = Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]);
        let result = Encoding::Base(BaseEncoding::new(10)).encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "78187493520");
    }

    #[test]
    fn test_encode_utf8() {
        let test_input = Decoded::Bytes(vec![0x6f, 0x6b, 0x20, 0x6c, 0x6f, 0x6c]);
        let result = Encoding::Text(TextEncoding::Utf(8)).encode(&test_input, Some(false));
        println!("{:?}", result);
    }

    #[test]
    fn test_encode_utf16() {
        let test_input = Decoded::Bytes(vec![0x0, 0x88, 0x0, 0xc6]);
        let result = Encoding::Text(TextEncoding::Utf(16)).encode(&test_input, Some(false));
        println!("{:?}", result);
    }

    #[test]
    fn test_encode_array() {
        let test_input = Decoded::Array(vec![
            Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]),
            Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]),
        ]);
        let result = Encoding::Array(ArrayEncoding::from(vec![
            Encoding::Base(BaseEncoding::new(16)),
            Encoding::Base(BaseEncoding::new(10)),
        ]))
        .encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "[0x1234567890, 78187493520]");
    }

    #[test]
    fn test_flatten() {
        let encoding = Encoding::Array(ArrayEncoding::new(
            vec![
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Array(ArrayEncoding::new(
                    vec![
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Array(ArrayEncoding::new(
                            vec![
                                Encoding::Base(BaseEncoding::new(10)),
                                Encoding::Base(BaseEncoding::new(10)),
                                Encoding::Base(BaseEncoding::new(10)),
                            ],
                            None,
                            None,
                        )),
                    ],
                    None,
                    None,
                )),
            ],
            None,
            None,
        ));
        let flattened = encoding.flatten();
        println!("{:?}", flattened);
    }

    #[test]
    fn test_encode_nested_array() {
        let test_input = Decoded::Array(vec![
            Decoded::Bytes(vec![0x0]),
            Decoded::Bytes(vec![0x1]),
            Decoded::Bytes(vec![0x2]),
            Decoded::Array(vec![
                Decoded::Bytes(vec![0x3]),
                Decoded::Bytes(vec![0x4]),
                Decoded::Bytes(vec![0x5]),
                Decoded::Array(vec![
                    Decoded::Bytes(vec![0x6]),
                    Decoded::Bytes(vec![0x7]),
                    Decoded::Bytes(vec![0x8]),
                ]),
            ]),
        ]);
        let encoding = Encoding::Array(ArrayEncoding::new(
            vec![
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Array(ArrayEncoding::new(
                    vec![
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Array(ArrayEncoding::new(
                            vec![
                                Encoding::Base(BaseEncoding::new(10)),
                                Encoding::Base(BaseEncoding::new(10)),
                                Encoding::Base(BaseEncoding::new(10)),
                            ],
                            None,
                            None,
                        )),
                    ],
                    None,
                    None,
                )),
            ],
            None,
            None,
        ));
        let result = encoding.encode(&test_input, Some(false)).unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn test_wrong_encoding() {
        let test_input = Decoded::Array(vec![
            Decoded::Bytes(vec![0x1]),
            Decoded::Bytes(vec![0x2]),
            Decoded::Bytes(vec![0x3]),
            Decoded::Bytes(vec![0x4]),
            Decoded::Bytes(vec![0x5]),
            Decoded::Bytes(vec![0x6]),
            Decoded::Bytes(vec![0x7]),
            Decoded::Bytes(vec![0x8]),
            Decoded::Bytes(vec![0x9]),
        ]);
        let encoding = Encoding::Array(ArrayEncoding::new(
            vec![
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Array(ArrayEncoding::new(
                    vec![
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Base(BaseEncoding::new(10)),
                        Encoding::Array(ArrayEncoding::new(
                            vec![
                                Encoding::Base(BaseEncoding::new(10)),
                                Encoding::Base(BaseEncoding::new(10)),
                                Encoding::Base(BaseEncoding::new(10)),
                            ],
                            None,
                            None,
                        )),
                    ],
                    None,
                    None,
                )),
            ],
            None,
            None,
        ));
        let result = encoding.flatten().encode(&test_input, Some(false)).unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn test_encode_array_newline() {
        let test_input = Decoded::Array(vec![
            Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]),
            Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]),
        ]);
        let result = Encoding::Array(ArrayEncoding::new(
            vec![
                Encoding::Base(BaseEncoding::new(16)),
                Encoding::Base(BaseEncoding::new(10)),
            ],
            None,
            Some(Separator::from('\n')),
        ))
        .encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "0x1234567890\n78187493520");
    }

    #[test]
    fn test_encode_array_round_brackets() {
        let test_input = Decoded::Array(vec![
            Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]),
            Decoded::Bytes(vec![0x90, 0x78, 0x56, 0x34, 0x12]),
        ]);
        let result = Encoding::Array(ArrayEncoding::new(
            vec![
                Encoding::Base(BaseEncoding::new(16)),
                Encoding::Base(BaseEncoding::new(10)),
            ],
            Some(Bracket::Round.into()),
            Some(Separator::from(",")),
        ))
        .encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "(0x1234567890, 78187493520)");
    }

    #[test]
    fn test_encode_hash() {
        let test_input = "test_key";
        let hashed = Hasher::Keccak(256)
            .encode(&Decoded::from_be_bytes(test_input.as_bytes()), Some(true))
            .unwrap();
        assert_eq!(
            &hashed,
            "0xad62e20f6955fd04f45eef123e61f3c74ce24e1ce4f6ab270b886cd860fd65ac"
        );
        println!("{:?}", hashed);
    }

    #[test]
    fn test_left_pad_hex() {
        assert_eq!(
            BaseEncoding::base_n_left_pad(16, 5)("1234567890".to_string()),
            "1234567890"
        );
        assert_eq!(
            BaseEncoding::base_n_left_pad(16, 10)("1234567890".to_string()),
            "00000000001234567890"
        );
        assert_eq!(
            BaseEncoding::base_n_left_pad(64, 5)("1234567890".to_string()),
            "1234567890"
        );
        assert_eq!(
            BaseEncoding::base_n_left_pad(2, 5)("1001101001".to_string()),
            "0000000000000000000000000000001001101001"
        );
    }
}
