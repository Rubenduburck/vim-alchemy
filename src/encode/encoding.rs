use super::{
    decoding::Decoded,
    error::Error,
    hashing::Hasher,
    types::{Bracket, Separator},
};
use crate::encode::types::Brackets;
use base64::Engine;
use rug::Integer;

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

impl Encoding {
    const INTEGER: &'static str = "int";
    const BINARY: &'static str = "bin";
    const BYTES: &'static str = "bytes";
    const BASE: &'static str = "base";
    const UTF: &'static str = "utf";
    const HEX: &'static str = "hex";


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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BaseEncoding {
    base: i32,
}

impl BaseEncoding {

    const BASE_64_ENGINE: base64::engine::general_purpose::GeneralPurpose =
        base64::engine::general_purpose::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::general_purpose::NO_PAD,
        );

    pub fn new(base: i32) -> Self {
        Self { base }
    }

    pub fn base_n_zero(base: i32) -> String {
        match base {
            64 => "A",
            58 => "1",
            _ => "0",
        }
        .into()
    }

    pub fn base_n_prefix(base: i32) -> String {
        match base {
            16 => "0x",
            2 => "0b",
            _ => Default::default(),
        }
        .into()
    }

    fn format(base: i32) -> impl FnOnce(String) -> String {
        move |s| format!("{}{}", Self::base_n_prefix(base), s)
    }

    pub fn encode(&self, input: &Decoded, pad: Option<bool>) -> Result<String, Error> {
        match self.base {
            2..=36 => Self::encode_with_rug(input, self.base),
            58 => Self::encode_base_58(input),
            64 => Self::encode_base_64(input),
            _ => Err(Error::UnsupportedBase(self.base)),
        }
        .map(|x| {
            if pad.unwrap_or(false) {
                Self::base_n_left_pad(self.base, input.len())(x)
            } else {
                x
            }
        })
        .map(Self::format(self.base))
    }

    fn encode_with_rug(input: &Decoded, base: i32) -> Result<String, Error> {
        Ok(
            Integer::from_digits(&input.to_le_bytes(), rug::integer::Order::Lsf)
                .to_string_radix(base),
        )
    }

    fn encode_base_58(input: &Decoded) -> Result<String, Error> {
        Ok(bs58::encode(input.to_be_bytes().clone()).into_string())
    }

    fn encode_base_64(input: &Decoded) -> Result<String, Error> {
        Ok(Self::BASE_64_ENGINE.encode(input.to_be_bytes().clone()))
    }

    fn base_n_pad_count(base: i32, target_byte_count: usize) -> usize {
        if base < 2 {
            return 0;
        }
        (8.0 / f64::log2(base as f64) * (target_byte_count as f64)).ceil() as usize
    }

    pub fn base_n_left_pad(base: i32, target_byte_count: usize) -> impl FnOnce(String) -> String {
        let zero = Self::base_n_zero(base);
        let target_str_len = Self::base_n_pad_count(base, target_byte_count);
        move |s| {
            let padding_count = target_str_len.saturating_sub(s.len());
            let padding = zero.repeat(padding_count);
            format!("{}{}", padding, s)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextEncoding {
    Utf(u8),
    Ascii,
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

#[derive(Debug, Clone)]
pub struct ArrayEncoding {
    pub values: Vec<Encoding>,
    pub brackets: Brackets,
    pub separator: Separator,
}

impl ArrayEncoding {
    pub fn new(
        values: Vec<Encoding>,
        brackets: Option<Brackets>,
        separator: Option<Separator>,
    ) -> Self {
        Self {
            values,
            brackets: brackets.unwrap_or_default(),
            separator: separator.unwrap_or_default(),
        }
    }

    pub fn flattened_values(&self) -> Vec<Encoding> {
        self.values
            .iter()
            .flat_map(|v| match v {
                Encoding::Array(a) => a.flattened_values(),
                _ => vec![v.clone()],
            })
            .collect()
    }

    pub fn flatten(&self) -> Self {
        Self::new(
            self.flattened_values(),
            Some(self.brackets.clone()),
            Some(self.separator),
        )
    }

    pub fn brackets(&self) -> [String; 2] {
        [
            self.brackets
                .open()
                .map(|c| c.to_string())
                .unwrap_or_default(),
            self.brackets
                .close()
                .map(|c| c.to_string())
                .unwrap_or_default(),
        ]
    }

    pub fn inner(&self) -> &Vec<Encoding> {
        &self.values
    }

    pub fn newline(&self) -> bool {
        self.separator.newline
    }

    pub fn encode(&self, input: &Decoded, pad: Option<bool>) -> Result<String, Error> {
        Ok(self.brackets().join(
            &input
                .to_vec()
                .iter()
                .zip(self.values.iter().cycle())
                .map(|(x, y)| y.encode(x, pad))
                .collect::<Result<Vec<String>, Error>>()?
                .join(&self.separator.to_string()),
        ))
    }
}

impl From<Vec<Encoding>> for ArrayEncoding {
    fn from(values: Vec<Encoding>) -> Self {
        Self::new(
            values,
            Some(Brackets::new(
                Some(Bracket::default()),
                Some(Bracket::default()),
            )),
            Some(Separator::default()),
        )
    }
}

impl Eq for ArrayEncoding {}

impl PartialEq for ArrayEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl Ord for ArrayEncoding {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.values.cmp(&other.values)
    }
}

impl PartialOrd for ArrayEncoding {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
            (Encoding::Base(BaseEncoding{base: a}), Encoding::Base(BaseEncoding{base: b})) => match (*a, *b) {
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

impl From<&str> for Encoding {
    fn from(s: &str) -> Self {
        let s = s.trim().to_lowercase();

        // [hex, int, base12] -> Array([Base(16), Base(10), Base(12)])
        if s.starts_with('[') && s.ends_with(']') {
            Encoding::Array(
                s[1..s.len() - 1]
                    .split(',')
                    .map(|e| Encoding::from(e.trim()))
                    .collect::<Vec<_>>()
                    .into(),
            )

        // base64 -> Base(64)
        } else if let Some(stripped) = s.strip_prefix(Self::BASE) {
            Encoding::Base(BaseEncoding::new(stripped.parse::<i32>().unwrap_or(10)))

        // utf8 -> Utf(8)
        } else if let Some(stripped) = s.strip_prefix(Self::UTF) {
            Encoding::Text(TextEncoding::Utf(stripped.parse::<u8>().unwrap_or(8)))

        // hex -> Base(16)
        } else {
            match s.as_str() {
                Self::BINARY => Encoding::Base(BaseEncoding::new(2)),
                Self::HEX => Encoding::Base(BaseEncoding::new(16)),
                Self::INTEGER => Encoding::Base(BaseEncoding::new(10)),
                Self::BYTES => Encoding::Array(vec![Encoding::Base(BaseEncoding::new(16))].into()),
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
        ];
        let expected_output = vec![
            Encoding::Base(BaseEncoding::new(16)),
            Encoding::Base(BaseEncoding::new(64)),
            Encoding::Base(BaseEncoding::new(10)),
            Encoding::Array(vec![Encoding::Base(BaseEncoding::new(16))].into()),
            Encoding::Array(vec![Encoding::Base(BaseEncoding::new(16)), Encoding::Base(BaseEncoding::new(10))].into()),
            Encoding::Base(BaseEncoding::new(58)),
            Encoding::Base(BaseEncoding::new(3)),
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
                            vec![Encoding::Base(BaseEncoding::new(10)), Encoding::Base(BaseEncoding::new(10)), Encoding::Base(BaseEncoding::new(10))],
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
                            vec![Encoding::Base(BaseEncoding::new(10)), Encoding::Base(BaseEncoding::new(10)), Encoding::Base(BaseEncoding::new(10))],
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
                            vec![Encoding::Base(BaseEncoding::new(10)), Encoding::Base(BaseEncoding::new(10)), Encoding::Base(BaseEncoding::new(10))],
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
            vec![Encoding::Base(BaseEncoding::new(16)), Encoding::Base(BaseEncoding::new(10))],
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
            vec![Encoding::Base(BaseEncoding::new(16)), Encoding::Base(BaseEncoding::new(10))],
            Some(Bracket::Round.into()),
            Some(Separator::from(",")),
        ))
        .encode(&test_input, Some(false));
        assert_eq!(result.unwrap(), "(0x1234567890, 78187493520)");
    }

    #[test]
    fn test_encode_hash() {
        let test_input = "test_key";
        let hashed = Hasher::Keccak(256).encode(&Decoded::from_be_bytes(test_input.as_bytes()), Some(true)).unwrap();
        assert_eq!(&hashed, "0xad62e20f6955fd04f45eef123e61f3c74ce24e1ce4f6ab270b886cd860fd65ac");
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
