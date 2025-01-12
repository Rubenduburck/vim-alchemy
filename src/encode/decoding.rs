use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};
use rug::Integer as RugInteger;

use crate::{
    classify::types::{Array, Classification, Integer},
    encode::error::Error,
};

#[derive(Debug, Clone)]
pub enum Decoded {
    Array(Vec<Decoded>),
    Bytes(Vec<u8>),
}

impl Default for Decoded {
    fn default() -> Self {
        Decoded::Bytes(vec![])
    }
}

const BASE_64_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::NO_PAD);

impl<'a> TryFrom<&Integer<'a>> for Decoded {
    type Error = Error;

    fn try_from(classification: &Integer<'a>) -> Result<Self, Self::Error> {
        match classification.base {
            2..=36 => Ok(Decoded::from_le_bytes(
                &RugInteger::from_str_radix(classification.value, classification.base)?
                    .to_digits::<u8>(rug::integer::Order::LsfBe),
            )),
            58 => Ok(Decoded::from_be_bytes(
                &bs58::decode(classification.value).into_vec()?,
            )),
            64 => Ok(Decoded::from_be_bytes(
                &BASE_64_ENGINE.decode(classification.value.as_bytes())?,
            )),
            _ => Err(Error::UnsupportedBase(classification.base)),
        }
    }
}

impl<'a> From<Array<'a>> for Decoded {
    fn from(classification: Array<'a>) -> Self {
        Decoded::Array(
            classification
                .collapse()
                .into_iter()
                .map(Decoded::from)
                .collect(),
        )
    }
}

impl<'a> From<&Array<'a>> for Decoded {
    fn from(classification: &Array<'a>) -> Self {
        Decoded::Array(
            classification
                .collapse()
                .into_iter()
                .map(Decoded::from)
                .collect(),
        )
    }
}

impl<'a> From<&Classification<'a>> for Decoded {
    fn from(classification: &Classification<'a>) -> Self {
        match classification {
            Classification::Array(a) => Decoded::from(a),
            Classification::Integer(i) => Decoded::try_from(i).unwrap_or_default(),
            _ => Decoded::default(),
        }
    }
}

#[allow(dead_code)]
trait Hasher {
    fn update(&mut self, input: &[u8]);
    fn finalize(self) -> [u8; 32];
}

impl Decoded {
    pub fn len(&self) -> usize {
        match self {
            Decoded::Array(a) => a.len(),
            Decoded::Bytes(b) => b.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Decoded::Bytes(vec![0]);
        }
        Decoded::Bytes(bytes.iter().rev().cloned().collect())
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Decoded::Bytes(vec![0]);
        }
        Decoded::Bytes(bytes.to_vec())
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        match self {
            Decoded::Array(a) => a.iter().rev().flat_map(|x| x.to_be_bytes()).collect(),
            Decoded::Bytes(b) => b.iter().rev().cloned().collect(),
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        match self {
            Decoded::Array(a) => a.iter().flat_map(|x| x.to_le_bytes()).collect(),
            Decoded::Bytes(b) => b.clone(),
        }
    }

    pub fn leading_zero_bytes(&self) -> usize {
        match self {
            Decoded::Array(_) => 0,
            Decoded::Bytes(b) => b.iter().take_while(|x| **x == 0).count(),
        }
    }

    pub fn trailing_zero_bytes(&self) -> usize {
        match self {
            Decoded::Array(_) => 0,
            Decoded::Bytes(b) => b.iter().rev().take_while(|x| **x == 0).count(),
        }
    }
}

pub struct DecodedIter<'a> {
    inner: &'a Decoded,
    index: usize,
}

impl<'a> Iterator for DecodedIter<'a> {
    type Item = &'a Decoded;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner {
            Decoded::Array(a) => {
                if self.index < a.len() {
                    let item = &a[self.index];
                    self.index += 1;
                    Some(item)
                } else {
                    None
                }
            }
            _ => Some(self.inner),
        }
    }
}

impl Decoded {
    pub fn to_vec(&self) -> Vec<Decoded> {
        match self {
            Decoded::Array(a) => a.clone(),
            Decoded::Bytes(b) => b.iter().map(|x| Decoded::Bytes(vec![*x])).collect(),
        }
    }

    pub fn left_truncate(&self, length: usize) -> Self {
        match self {
            Self::Array(a) => Self::Array(a.iter().take(length).cloned().collect()),
            Self::Bytes(b) => Self::Bytes(b.iter().take(length).cloned().collect()),
        }
    }

    pub fn right_truncate(&self, length: usize) -> Self {
        match self {
            Self::Array(a) => Self::Array(a.iter().rev().take(length).rev().cloned().collect()),
            Self::Bytes(b) => Self::Bytes(b.iter().rev().take(length).rev().cloned().collect()),
        }
    }

    pub fn left_pad(&self, padding: usize) -> Self {
        match self {
            Self::Array(a) => Self::Array(
                std::iter::repeat(Decoded::Bytes(vec![0]))
                    .take(padding.saturating_sub(a.len()))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .chain(a.iter().cloned())
                    .collect(),
            ),
            Self::Bytes(b) => Self::Bytes(
                std::iter::repeat(0)
                    .take(padding.saturating_sub(b.len()))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .chain(b.iter().cloned())
                    .collect(),
            ),
        }
    }

    pub fn right_pad(&self, padding: usize) -> Self {
        match self {
            Self::Array(a) => Self::Array(
                a.iter()
                    .cloned()
                    .chain(
                        std::iter::repeat(Decoded::Bytes(vec![0]))
                            .take(padding.saturating_sub(a.len()))
                            .collect::<Vec<_>>(),
                    )
                    .collect(),
            ),
            Self::Bytes(b) => Self::Bytes(
                b.iter()
                    .cloned()
                    .chain(
                        std::iter::repeat(0)
                            .take(padding.saturating_sub(b.len()))
                            .collect::<Vec<_>>(),
                    )
                    .collect(),
            ),
        }
    }

    pub fn flatten_values(&self) -> Vec<Decoded> {
        match self {
            Self::Array(a) => a
                .iter()
                .flat_map(|x| match x {
                    Self::Array(a) => a.to_vec(),
                    _ => vec![x.clone()],
                })
                .collect(),
            _ => vec![self.clone()],
        }
    }

    pub fn flatten(&self) -> Self {
        match self {
            Self::Array(a) => Self::Array(a.iter().flat_map(|x| x.flatten_values()).collect()),
            _ => self.clone(),
        }
    }

    pub fn chunk(&self, chunk_count: usize) -> Self {
        match self {
            Self::Array(a) => Self::Array(
                a.chunks(a.len() / chunk_count)
                    .map(|x| Self::Array(x.to_vec()))
                    .collect(),
            ),
            Self::Bytes(b) => Self::Array(
                b.chunks(b.len() / chunk_count)
                    .map(Self::from_le_bytes)
                    .collect(),
            ),
        }
    }

    pub fn reverse(&self, depth: usize) -> Self {
        if depth == 0 {
            return self.clone();
        }
        match self {
            Self::Array(a) => Self::Array(a.iter().rev().map(|x| x.reverse(depth - 1)).collect()),
            Self::Bytes(b) => Self::Bytes(b.iter().rev().cloned().collect()),
        }
    }

    pub fn rotate(&self, rotation: isize) -> Self {
        match self {
            Self::Array(a) => {
                let len = a.len() as isize;
                let rotation = (rotation % len + len) % len;
                let (left, right) = a.split_at(rotation as usize);
                Self::Array(right.iter().chain(left).cloned().collect())
            }
            Self::Bytes(b) => {
                let len = b.len() as isize;
                let rotation = (rotation % len + len) % len;
                let (left, right) = b.split_at(rotation as usize);
                Self::Bytes(right.iter().chain(left).cloned().collect())
            }
        }
    }
}
