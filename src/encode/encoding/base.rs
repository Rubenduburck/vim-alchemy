use super::{super::decoding::Decoded, super::error::Error};
use base64::Engine;
use rug::Integer;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BaseEncoding {
    pub(crate) base: i32,
}

impl std::fmt::Display for BaseEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add special display for some cases
        write!(f, "base{}", self.base)
    }
}

impl BaseEncoding {
    const BASE_64_ENGINE: base64::engine::general_purpose::GeneralPurpose =
        base64::engine::general_purpose::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::general_purpose::NO_PAD,
        );

    pub const fn new(base: i32) -> Self {
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
