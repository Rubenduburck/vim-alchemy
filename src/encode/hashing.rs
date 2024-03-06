use sha3::{digest::DynDigest, Digest};

use super::decoding::Decoded;
use super::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hasher {
    Sha2(usize),
    Sha3(usize),
    Keccak(usize),
}

impl Default for Hasher {
    fn default() -> Self {
        Self::Keccak(256)
    }
}

impl From<&str> for Hasher {
    fn from(s: &str) -> Self {
        let s = s.trim().to_lowercase();
        if let Some(s) = s.strip_prefix(Self::KECCAK) {
            Self::Keccak(s.parse().unwrap_or(256))
        } else if let Some(s) = s.strip_prefix(Self::SHA2) {
            Self::Sha2(s.parse().unwrap_or(256))
        } else if let Some(s) = s.strip_prefix(Self::SHA3) {
            Self::Sha3(s.parse().unwrap_or(256))
        } else {
            Self::default()
        }
    }
}

impl Hasher {
    const SHA2: &'static str = "sha2";
    const SHA3: &'static str = "sha3";
    const KECCAK: &'static str = "keccak";

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
        }
    }

    pub fn hash(&self, decoded: &Decoded) -> Result<Decoded, Error> {
        let mut hasher = self.hasher()?;
        match decoded {
            Decoded::Array(a) =>  a.iter().for_each(|item|  hasher.update(&item.to_be_bytes())),
            Decoded::Bytes(_) =>  decoded.to_be_bytes().chunks(32).for_each(|chunk| hasher.update(chunk)),
        };
        Ok(Decoded::from_be_bytes(&hasher.finalize()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_keccak_256() {
        let input = "test_key";
        let expected = vec![0xad, 0x62, 0xe2, 0xf, 0x69, 0x55, 0xfd, 0x4, 0xf4, 0x5e, 0xef, 0x12, 0x3e, 0x61, 0xf3, 0xc7, 0x4c, 0xe2, 0x4e, 0x1c, 0xe4, 0xf6, 0xab, 0x27, 0xb, 0x88, 0x6c, 0xd8, 0x60, 0xfd, 0x65, 0xac];
        let hasher = Hasher::Keccak(256);
        let decoded = Decoded::from_be_bytes(input.as_bytes());
        println!("decoded {:?}", decoded);
        let hashed = hasher.hash(&decoded).unwrap();
        println!("hashed {:?}", hashed);
        assert_eq!(hashed.to_be_bytes(), expected);
    }
}
