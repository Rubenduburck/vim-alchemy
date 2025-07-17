#![allow(clippy::uninlined_format_args)]
use sha3::{digest::DynDigest, Digest};

use super::decoding::Decoded;
use super::encoding::BaseEncoding;
use super::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hasher {
    Sha2(usize),
    Sha3(usize),
    Keccak(usize),
    Blake2(usize),
    Md5,
}

impl Hasher {
    pub fn sha2(bits: usize) -> Self {
        if [224, 256, 384, 512].contains(&bits) {
            Self::Sha2(bits)
        } else {
            Self::Sha2(256)
        }
    }

    pub fn sha3(bits: usize) -> Self {
        if [224, 256, 384, 512].contains(&bits) {
            Self::Sha3(bits)
        } else {
            Self::Sha3(256)
        }
    }

    pub fn keccak(bits: usize) -> Self {
        if [224, 256, 384, 512].contains(&bits) {
            Self::Keccak(bits)
        } else {
            Self::Keccak(256)
        }
    }

    pub fn blake2(bits: usize) -> Self {
        if [256, 512].contains(&bits) {
            Self::Blake2(bits)
        } else {
            Self::Blake2(256)
        }
    }

    pub fn md5() -> Self {
        Self::Md5
    }
}

impl std::fmt::Display for Hasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hasher::Sha2(bits) => write!(f, "sha2-{}", bits),
            Hasher::Sha3(bits) => write!(f, "sha3-{}", bits),
            Hasher::Keccak(bits) => write!(f, "keccak-{}", bits),
            Hasher::Blake2(bits) => write!(f, "blake2-{}", bits),
            Hasher::Md5 => write!(f, "md5"),
        }
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Self::Keccak(256)
    }
}

impl TryFrom<&String> for Hasher {
    type Error = Error;
    fn try_from(s: &String) -> Result<Self, Error> {
        Hasher::try_from(s.as_str())
    }
}

impl TryFrom<&str> for Hasher {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Error> {
        fn extract_bits(s: &str) -> usize {
            s.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or_default()
        }
        let s = s.trim().to_lowercase();

        if let Some(rest) = s.strip_prefix(Self::BLAKE2) {
            Ok(Self::blake2(extract_bits(rest)))
        } else if let Some(rest) = s.strip_prefix(Self::KECCAK) {
            Ok(Self::keccak(extract_bits(rest)))
        } else if let Some(rest) = s.strip_prefix(Self::SHA3) {
            Ok(Self::sha3(extract_bits(rest)))
        } else if let Some(rest) = s.strip_prefix(Self::SHA) {
            Ok(Self::sha2(extract_bits(rest)))
        } else {
            Err(Error::UnsupportedHash)
        }
    }
}

impl Hasher {
    const SHA: &'static str = "sha";
    const SHA3: &'static str = "sha3";
    const KECCAK: &'static str = "keccak";
    const BLAKE2: &'static str = "blake2";

    pub fn hasher(&self) -> Result<Box<dyn DynDigest>, Error> {
        match self {
            Self::Sha2(bits) => match bits {
                224 => Ok(Box::new(sha2::Sha224::new())),
                256 => Ok(Box::new(sha2::Sha256::new())),
                384 => Ok(Box::new(sha2::Sha384::new())),
                512 => Ok(Box::new(sha2::Sha512::new())),
                _ => Err(Error::UnsupportedHash),
            },
            Self::Sha3(bits) => match bits {
                224 => Ok(Box::new(sha3::Sha3_224::new())),
                256 => Ok(Box::new(sha3::Sha3_256::new())),
                384 => Ok(Box::new(sha3::Sha3_384::new())),
                512 => Ok(Box::new(sha3::Sha3_512::new())),
                _ => Err(Error::UnsupportedHash),
            },
            Self::Keccak(bits) => match bits {
                224 => Ok(Box::new(sha3::Keccak224::new())),
                256 => Ok(Box::new(sha3::Keccak256::new())),
                384 => Ok(Box::new(sha3::Keccak384::new())),
                512 => Ok(Box::new(sha3::Keccak512::new())),
                _ => Err(Error::UnsupportedHash),
            },
            Self::Blake2(bits) => match bits {
                256 => Ok(Box::new(blake2::Blake2s256::new())),
                512 => Ok(Box::new(blake2::Blake2b512::new())),
                _ => Err(Error::UnsupportedHash),
            },
            Self::Md5 => todo!(),
        }
    }

    pub fn hash(&self, decoded: &Decoded) -> Result<Decoded, Error> {
        let mut hasher = self.hasher()?;
        match decoded {
            Decoded::Array(a) => a.iter().for_each(|item| hasher.update(&item.to_be_bytes())),
            Decoded::Bytes(_) => decoded
                .to_be_bytes()
                .chunks(32)
                .for_each(|chunk| hasher.update(chunk)),
        };
        Ok(Decoded::from_be_bytes(&hasher.finalize()))
    }

    pub fn encode(&self, decoded: &Decoded, pad: Option<bool>) -> Result<String, Error> {
        BaseEncoding::new(16).encode(&self.hash(decoded)?, pad)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hasher_from_str() {
        use super::Hasher;

        // Test keccak variants
        assert_eq!(Hasher::try_from("keccak").unwrap(), Hasher::Keccak(256));
        assert_eq!(Hasher::try_from("keccak-256").unwrap(), Hasher::Keccak(256));
        assert_eq!(Hasher::try_from("keccak-512").unwrap(), Hasher::Keccak(512));
        assert_eq!(Hasher::try_from("keccak-384").unwrap(), Hasher::Keccak(384));

        // Test sha3 variants
        assert_eq!(Hasher::try_from("sha3").unwrap(), Hasher::Sha3(256));
        assert_eq!(Hasher::try_from("sha3-256").unwrap(), Hasher::Sha3(256));
        assert_eq!(Hasher::try_from("sha3-512").unwrap(), Hasher::Sha3(512));

        // Test sha2 variants
        assert_eq!(Hasher::try_from("sha").unwrap(), Hasher::Sha2(256));
        assert_eq!(Hasher::try_from("sha-256").unwrap(), Hasher::Sha2(256));
        assert_eq!(Hasher::try_from("sha-512").unwrap(), Hasher::Sha2(512));

        // Test blake2 variants
        assert_eq!(Hasher::try_from("blake2").unwrap(), Hasher::Blake2(256));
        assert_eq!(Hasher::try_from("blake2-256").unwrap(), Hasher::Blake2(256));
        assert_eq!(Hasher::try_from("blake2-512").unwrap(), Hasher::Blake2(512));
    }

    use super::*;

    #[test]
    fn test_keccak_256() {
        let input = "test_key";
        let expected = vec![
            0xad, 0x62, 0xe2, 0xf, 0x69, 0x55, 0xfd, 0x4, 0xf4, 0x5e, 0xef, 0x12, 0x3e, 0x61, 0xf3,
            0xc7, 0x4c, 0xe2, 0x4e, 0x1c, 0xe4, 0xf6, 0xab, 0x27, 0xb, 0x88, 0x6c, 0xd8, 0x60,
            0xfd, 0x65, 0xac,
        ];
        let hasher = Hasher::Keccak(256);
        let decoded = Decoded::from_be_bytes(input.as_bytes());
        println!("decoded {:?}", decoded);
        let hashed = hasher.hash(&decoded).unwrap();
        println!("hashed {:?}", hashed);
        assert_eq!(hashed.to_be_bytes(), expected);
    }
}
