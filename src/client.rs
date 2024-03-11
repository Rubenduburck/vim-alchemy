use crate::{
    classify::{classifier::Classifier, types::Classification},
    encode::decoding::Decoded,
    encode::{encoding::{Encoding, BaseEncoding}, hashing::Hasher},
    error::ConvertError,
};

pub struct Client {
    classifier: Classifier,
}

impl Client {
    pub fn new() -> Self {
        Self {
            classifier: Classifier::new(),
        }
    }

    pub fn classify<'a>(&'a self, input: &'a str) -> Vec<Classification<'a>> {
        self.classifier.classify(input)
    }

    pub fn classify_best_match<'a>(&'a self, input: &'a str) -> Classification<'a> {
        self.classify(input).into_iter().min().unwrap_or_default()
    }

    pub fn classify_and_convert(
        &self,
        encoding: &str,
        input: &str,
    ) -> Result<String, ConvertError> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let mut encoding = Encoding::from(encoding);
        match &best {
            Classification::Array(arr) if arr.is_lines() => {
                encoding = encoding.to_lines();
            }
            _ => (),
        }

        let decoded = Decoded::from(&best);
        Ok(encoding.encode(&decoded, pad)?)
    }

    pub fn classify_and_convert_all(
        &self,
        encoding: &str,
        input: &str,
    ) -> Result<Vec<(String, String)>, ConvertError> {
        let mut classifications = self.classify(input);
        classifications.sort();
        let decodings = classifications
            .iter()
            .map(|c| (c.to_string(), Decoded::from(c)))
            .collect::<Vec<_>>();
        let encoding = Encoding::from(encoding);
        let mut encodeds = Vec::new();
        for decoding in decodings {
            encodeds.push((decoding.0, encoding.encode(&decoding.1, Some(false))?));
        }
        Ok(encodeds)
    }

    pub fn flatten_array(&self, input: &str) -> Result<String, ConvertError> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let flattened = decoded.flatten();
        Ok(best.encoding().flatten().encode(&flattened, pad)?)
    }

    pub fn chunk_array(&self, chunk_count: usize, input: &str) -> Result<String, ConvertError> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let chunked = decoded.chunk(chunk_count);
        Ok(best.encoding().encode(&chunked, pad)?)
    }

    pub fn reverse_array(&self, input: &str, depth: usize) -> Result<String, ConvertError> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let reversed = decoded.reverse(depth);
        Ok(best.encoding().encode(&reversed, pad)?)
    }

    pub fn rotate_array(&self, input: &str, rotation: isize) -> Result<String, ConvertError> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let rotated = decoded.rotate(rotation);
        Ok(best.encoding().encode(&rotated, pad)?)
    }

    pub fn generate(&self, encoding: &str, length: usize) -> Result<String, ConvertError> {
        let encoding = Encoding::from(encoding);
        let generated = encoding.generate(length)?;
        Ok(generated)
    }

    pub fn random(&self, encoding: &str, length: usize) -> Result<String, ConvertError> {
        let encoding = Encoding::from(encoding);
        let randomized = encoding.random(length)?;
        Ok(randomized)
    }

    pub fn pad_left(&self, length: usize, input: &str) -> Result<String, ConvertError> {
        let pad = Some(true);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let padded = decoded.right_pad(length);
        let encoded = best.encoding().encode(&padded, pad)?;
        Ok(encoded)
    }

    pub fn pad_right(&self, length: usize, input: &str) -> Result<String, ConvertError> {
        let pad = Some(true);
        let best = self.classify_best_match(input);
        let decoded: Decoded = (&best).into();
        let padded = decoded.left_pad(length);
        let encoded = best.encoding().encode(&padded, pad)?;
        Ok(encoded)
    }

    pub fn hash(&self, algorithm: &str, input: &str) -> Result<String, ConvertError> {
        let best = self.classify_best_match(input);
        let hash_encoding = Hasher::from(algorithm);
        let encoded = if best.error() > 0 {
            let decoded = Decoded::from_be_bytes(input.as_bytes());
            let hash = hash_encoding.hash(&decoded)?;
            Encoding::Base(BaseEncoding::new(16)).encode(&hash, Some(true))?
        } else {
            let decoded = Decoded::from(&best);
            let hash = hash_encoding.hash(&decoded)?;
            best.encoding().encode(&hash, Some(true))?
        };

        Ok(encoded)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_hex_on_lines() {
        let client = Client::new();
        let lines = "123\n456\n789";
        let converted = client.classify_and_convert("hex", lines).expect("Failed to convert");
        assert_eq!(converted, "0x7b\n0x1c8\n0x315");
    }

    #[test]
    fn test_client_hex_from_bytes() {
        let client = Client::new();
        let test_set = [
            "[\n0x0,\n 0x0,\n 0x90,\n 0x78,\n 0x56,\n 0x34,\n 0x12\n]",
            "[0x0, 0x0, 0x90, 0x78, 0x56, 0x34, 0x12]",
        ];
        let expected = vec!["0x12345678900000"];

        for (test, expect) in test_set.iter().zip(expected.into_iter().cycle()) {
            let converted = client
                .classify_and_convert("hex", test)
                .expect("Failed to convert");
            assert_eq!(expect, converted);
        }
    }

    #[test]
    fn test_client_bytes() {
        let client = Client::new();
        let test_set = ["92488e1e3eeecdf99f3ed2ce59233efb4b4fb612d5655c0ce9ea52b5a502e655"];
        let expected = vec!["[0x55, 0xe6, 0x2, 0xa5, 0xb5, 0x52, 0xea, 0xe9, 0xc, 0x5c, 0x65, 0xd5, 0x12, 0xb6, 0x4f, 0x4b, 0xfb, 0x3e, 0x23, 0x59, 0xce, 0xd2, 0x3e, 0x9f, 0xf9, 0xcd, 0xee, 0x3e, 0x1e, 0x8e, 0x48, 0x92]"];

        for (test, expect) in test_set.iter().zip(expected) {
            let converted = client
                .classify_and_convert("bytes", test)
                .expect("Failed to convert");
            assert_eq!(expect, converted);
        }
    }

    #[test]
    fn test_client_int() {
        let client = Client::new();
        let test_set = vec![
            "1234",
            "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789",
        ];

        for test in test_set {
            let converted = client
                .classify_and_convert("int", test)
                .expect("Failed to convert");
            assert_eq!(test, converted);
        }
    }

    #[test]
    fn test_client_hex() {
        let client = Client::new();
        let test_set = vec![
            "0x1234",
            "0x123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789",
        ];

        for test in test_set {
            let converted = client
                .classify_and_convert("hex", test)
                .expect("Failed to convert");
            assert_eq!(test, converted);
        }
    }

    #[test]
    fn test_client_base_64() {
        let client = Client::new();
        let test_set = vec!["aGVsbG8", "b3BlbmNhc3Q"];

        for test in test_set {
            let converted = client
                .classify_and_convert("base64", test)
                .expect("Failed to convert");
            assert_eq!(test, converted);
        }
    }

    #[test]
    fn test_flatten_array() {
        let client = Client::new();
        const TEST: &str = "[1,2,3,[4,5,6,[7,8,9]]]";

        let converted = client.flatten_array(TEST).expect("Failed to convert");
        println!("{}", converted);
        assert_eq!(converted, "[1, 2, 3, 4, 5, 6, 7, 8, 9]");
    }

    #[test]
    fn test_chunk_array() {
        let client = Client::new();
        const TEST: &str = "[0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]";

        let converted = client.chunk_array(3, TEST).expect("Failed to convert");
        println!("{}", converted);
        assert_eq!(converted, "[0x70809, 0x40506, 0x10203]");
    }

    #[test]
    fn test_generate() {
        let client = Client::new();
        let generated = client.generate("hex", 32).expect("Failed to convert");
        assert_eq!(
            generated,
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        let generate = client.generate("base64", 32).expect("Failed to convert");
        assert_eq!(generate, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        let generate = client.generate("int", 32).expect("Failed to convert");
        assert_eq!(
            generate,
            "000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );
        let generate = client.generate("bytes", 32).expect("Failed to convert");
        assert_eq!(generate, "[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]");
    }

    #[test]
    fn test_pad_right() {
        let client = Client::new();
        let padded = client.pad_right(32, "0x1234").expect("Failed to convert");
        assert_eq!(
            padded,
            "0x1234000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn test_pad_left() {
        let client = Client::new();
        let padded = client.pad_left(32, "0x1234").expect("Failed to convert");
        assert_eq!(
            padded,
            "0x0000000000000000000000000000000000000000000000000000000000001234"
        );
    }

    #[test]
    fn test_hash() {
        let client = Client::new();
        let hashed = client.hash("keccak256", "test_key").expect("Failed to convert");
        assert_eq!(hashed, "0xad62e20f6955fd04f45eef123e61f3c74ce24e1ce4f6ab270b886cd860fd65ac");
        let hashed = client.hash("keccak256", "0x1234").expect("Failed to convert");
        assert_eq!(hashed, "0x56570de287d73cd1cb6092bb8fdee6173974955fdef345ae579ee9f475ea7432");
    }
}
