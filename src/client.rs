use std::collections::HashMap;

use neovim_lib::Value;

use crate::{
    classify::{classifier::Classifier, types::Classification},
    encode::{
        decoding::Decoded,
        encoding::{BaseEncoding, Encoding},
        hashing::Hasher,
    },
    error::Error,
    get_param,
};

pub struct Client {
    classifier: Classifier,
}

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub(crate) classifier: crate::classify::classifier::Config,
}

impl From<Value> for Config {
    fn from(value: Value) -> Self {
        if let Value::Map(map) = value {
            Self {
                classifier: get_param(&map, "classifier").unwrap_or_default(),
            }
        } else {
            Self::default()
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            classifier: Classifier::default(),
        }
    }

    pub fn setup(&mut self, cfg: Config) {
        self.classifier.setup(cfg.classifier);
    }

    pub fn classify<'a>(&'a self, input: &'a str) -> Vec<Classification<'a>> {
        let results = self.classifier.classify(input);
        tracing::debug!("Results: {:?}", results);
        results
    }

    pub fn classify_best_match<'a>(&'a self, input: &'a str) -> Classification<'a> {
        self.classify(input).into_iter().min().unwrap_or_default()
    }

    pub fn classify_with<'a>(&'a self, encoding: &Encoding, input: &'a str) -> Classification<'a> {
        self.classifier.classify_with(encoding, input)
    }

    pub fn decode(&self, encoding: &Encoding, input: &str) -> Result<Decoded, Error> {
        let classification = self.classify_with(encoding, input);
        tracing::debug!("Classification: {:?}", classification);
        let decoded = Decoded::from(&classification);
        Ok(decoded)
    }

    pub fn encode(&self, encoding: &Encoding, input: &Decoded) -> Result<String, Error> {
        let encoded = encoding.encode(input, None)?;
        Ok(encoded)
    }

    pub fn convert(
        &self,
        input_encoding: &Encoding,
        mut output_encoding: Encoding,
        input: &str,
    ) -> Result<String, Error> {
        let pad = Some(false);
        let classification = self.classify_with(input_encoding, input);
        if matches!(&classification, Classification::Array(arr) if arr.is_lines()) {
            output_encoding = output_encoding.to_lines();
        }
        let decoded = Decoded::from(&classification);
        if matches!(&classification, Classification::Array(arr) if arr.is_lines()) {
            return Ok(output_encoding.to_lines().encode(&decoded, pad)?);
        }
        Ok(output_encoding.encode(&decoded, pad)?)
    }

    pub fn classify_and_convert(
        &self,
        mut output_encoding: Vec<Encoding>,
        input: &str,
    ) -> Result<HashMap<String, String>, Error> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        if matches!(&best, Classification::Array(arr) if arr.is_lines()) {
            output_encoding = output_encoding.into_iter().map(|e| e.to_lines()).collect();
        }

        let decoded = Decoded::from(&best);
        Ok(output_encoding
            .into_iter()
            .flat_map(|e| {
                e.encode(&decoded, pad)
                    .map(|result| (e.to_string(), result))
            })
            .collect())
    }

    pub fn flatten_array(&self, input: &str) -> Result<String, Error> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        tracing::debug!("Decoded: {:?}", decoded);
        let flattened = decoded.flatten();
        Ok(best.encoding().flatten().encode(&flattened, pad)?)
    }

    pub fn chunk_array(&self, chunk_count: usize, input: &str) -> Result<String, Error> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let chunked = decoded.chunk(chunk_count);
        Ok(best.encoding().encode(&chunked, pad)?)
    }

    pub fn reverse_array(&self, input: &str, depth: usize) -> Result<String, Error> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let reversed = decoded.reverse(depth);
        Ok(best.encoding().encode(&reversed, pad)?)
    }

    pub fn rotate_array(&self, input: &str, rotation: isize) -> Result<String, Error> {
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let rotated = decoded.rotate(rotation);
        Ok(best.encoding().encode(&rotated, pad)?)
    }

    pub fn generate(&self, encoding: &str, length: usize) -> Result<String, Error> {
        let encoding = Encoding::from(encoding);
        let generated = encoding.generate(length)?;
        Ok(generated)
    }

    pub fn random(&self, encoding: &str, length: usize) -> Result<String, Error> {
        let encoding = Encoding::from(encoding);
        let randomized = encoding.random(length)?;
        Ok(randomized)
    }

    pub fn pad_left(&self, length: usize, input: &str) -> Result<String, Error> {
        let pad = Some(true);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        let padded = decoded.right_pad(length);
        let encoded = best.encoding().encode(&padded, pad)?;
        Ok(encoded)
    }

    pub fn pad_right(&self, length: usize, input: &str) -> Result<String, Error> {
        let pad = Some(true);
        let best = self.classify_best_match(input);
        let decoded: Decoded = (&best).into();
        let padded = decoded.left_pad(length);
        let encoded = best.encoding().encode(&padded, pad)?;
        Ok(encoded)
    }

    pub fn classify_and_hash(
        &self,
        algorithm: Vec<String>,
        input: &str,
    ) -> Result<HashMap<String, String>, Error> {
        const DEFAULT_ENCODING: Encoding = Encoding::Base(BaseEncoding::new(16));
        let pad = Some(false);
        let best = self.classify_best_match(input);
        let decoded = Decoded::from(&best);
        Ok(algorithm
            .iter()
            .flat_map(|alg| {
                Hasher::try_from(alg)
                    .and_then(|hasher| hasher.hash(&decoded))
                    .and_then(|hash| DEFAULT_ENCODING.encode(&hash, pad))
                    .map(|hash| (alg.clone(), hash))
            })
            .collect())
    }

    pub fn hash(
        &self,
        algorithm: &str,
        input: &str,
        input_encoding: Encoding,
    ) -> Result<String, Error> {
        const DEFAULT_ENCODING: Encoding = Encoding::Base(BaseEncoding::new(16));
        let classification = self.classify_with(&input_encoding, input);
        let hash_encoding = Hasher::try_from(algorithm)?;
        let decoded = Decoded::from(&classification);
        let hash = hash_encoding.hash(&decoded)?;
        let encoded = DEFAULT_ENCODING.encode(&hash, Some(true))?;
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

    // FIXME: Test no longer works after adding textencoding
    //#[test]
    //fn test_client_hex_on_lines() {
    //    let client = Client::new();
    //    let lines = "123\n456\n789";
    //    let encoding = vec![Encoding::from("hex")];
    //    let converted = client
    //        .classify_and_convert(encoding, lines)
    //        .expect("Failed to convert");
    //    assert_eq!(converted.values().next().unwrap(), "0x7b\n0x1c8\n0x315");
    //}

    #[test]
    fn test_client_hex_from_bytes() {
        let client = Client::new();
        let test_set = [
            "[\n0x0,\n 0x0,\n 0x90,\n 0x78,\n 0x56,\n 0x34,\n 0x12\n]",
            "[0x0, 0x0, 0x90, 0x78, 0x56, 0x34, 0x12]",
"[0x21, 0x08, 0x77, 0x3a, 0x2d, 0x8f, 0x6e, 0x89, 0x24, 0xf0, 0x6c, 0x1f, 0x58, 0x81, 0x16, 0xfd, 0x98, 0x7a, 0x49, 0x54, 0xc9, 0xf5, 0x6f, 0xfd, 0xd4, 0x5e, 0x03, 0x93, 0x3d, 0x23, 0x60, 0x3e]"

        ];
        let expected = vec![
            "0x12345678900000",
            "0x12345678900000",
            "0x3e60233d93035ed4fd6ff5c954497a98fd1681581f6cf024896e8f2d3a770821",
        ];

        for (test, expect) in test_set.iter().zip(expected.into_iter().cycle()) {
            println!("Test: {}", test);
            let encoding = vec![Encoding::from("hex")];
            let converted = client
                .classify_and_convert(encoding, test)
                .expect("Failed to convert");
            assert_eq!(expect, converted.values().next().unwrap());
        }
    }

    #[test]
    fn test_client_bytes() {
        let client = Client::new();
        let test_set = ["92488e1e3eeecdf99f3ed2ce59233efb4b4fb612d5655c0ce9ea52b5a502e655"];
        let expected = vec![
            "[0x55, 0xe6, 0x2, 0xa5, 0xb5, 0x52, 0xea, 0xe9, 0xc, 0x5c, 0x65, 0xd5, 0x12, 0xb6, 0x4f, 0x4b, 0xfb, 0x3e, 0x23, 0x59, 0xce, 0xd2, 0x3e, 0x9f, 0xf9, 0xcd, 0xee, 0x3e, 0x1e, 0x8e, 0x48, 0x92]"
        ];

        for (test, expect) in test_set.iter().zip(expected) {
            let encoding = vec![Encoding::from("bytes")];
            let converted = client
                .classify_and_convert(encoding, test)
                .expect("Failed to convert");
            assert_eq!(expect, converted.values().next().unwrap());
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
            let encoding = vec![Encoding::from("int")];
            let converted = client
                .classify_and_convert(encoding, test)
                .expect("Failed to convert");
            assert_eq!(test, converted.values().next().unwrap());
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
            let encoding = vec![Encoding::from("hex")];
            let converted = client
                .classify_and_convert(encoding, test)
                .expect("Failed to convert");
            assert_eq!(test, converted.values().next().unwrap());
        }
    }

    #[test]
    fn test_client_base_64() {
        let client = Client::new();
        let test_set = vec!["aGVsbG8", "b3BlbmNhc3Q"];

        for test in test_set {
            let encoding = vec![Encoding::from("base64")];
            let converted = client
                .classify_and_convert(encoding, test)
                .expect("Failed to convert");
            assert_eq!(test, converted.values().next().unwrap());
        }
    }

    #[test]
    fn test_flatten_array() {
        let client = Client::new();
        let test: &str = "[1,2,3,[4,5,6,[7,8,9]]]";

        let converted = client.flatten_array(test).expect("Failed to convert");
        println!("{}", converted);
        assert_eq!(converted, "[1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let test: &str = "[[123]]";
        let converted = client.flatten_array(test).expect("Failed to convert");
        println!("{}", converted);
        assert_eq!(converted, "[123]");
    }

    #[test]
    fn test_chunk_array() {
        let client = Client::new();
        let test: &str = "[0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]";

        let converted = client.chunk_array(3, test).expect("Failed to convert");
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
    #[tracing_test::traced_test]
    fn test_hash() {
        let client = Client::new();

        let hashed = client
            .hash("keccak256", "getBalance()", Encoding::from("ascii"))
            .expect("Failed to convert");
        assert_eq!(
            hashed,
            "0x12065fe058ec54d32b956897063181d660e7d27f9bb883d28d5cc5ab3423e23c"
        );

        let hashed = client
            .hash("keccak256", "test_key", Encoding::from("ascii"))
            .expect("Failed to convert");
        assert_eq!(
            hashed,
            "0xad62e20f6955fd04f45eef123e61f3c74ce24e1ce4f6ab270b886cd860fd65ac"
        );

        let hashed = client
            .hash("sha256", "0x1234", Encoding::from("ascii"))
            .expect("Failed to convert");
        assert_eq!(
            hashed,
            "0x5a0737e8cbcfa24dcc118b0ab1e6d98bee17c57daa8a1686024159aae707ed6f"
        );
    }
}
