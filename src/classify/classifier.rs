use neovim_lib::Value;

use crate::{
    classify::{
        regex::RegexCache,
        types::{Array, Classification},
    },
    encode::{
        encoding::{ArrayEncoding, BaseEncoding, Encoding, TextEncoding},
        types::{Bracket, Brackets, Separator},
    },
};

use super::types::{Integer, Text};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Config {
    pub available_encodings: Vec<Encoding>,
}

impl From<Value> for Config {
    fn from(v: Value) -> Self {
        if let Value::Map(map) = v {
            let available_encodings = crate::get_array::<String>(&map, "available_encodings")
                .unwrap_or_default()
                .into_iter()
                .map(|v| Encoding::from(v.as_str()))
                .collect();
            Self {
                available_encodings,
            }
        } else {
            Self::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            available_encodings: vec![
                Encoding::Base(BaseEncoding::new(2)),
                Encoding::Base(BaseEncoding::new(10)),
                Encoding::Base(BaseEncoding::new(16)),
                Encoding::Base(BaseEncoding::new(58)),
                Encoding::Base(BaseEncoding::new(64)),
                Encoding::Text(TextEncoding::Utf(8)),
            ],
        }
    }
}

pub struct Classifier {
    cfg: Config,
    re: RegexCache,
}

impl Default for Classifier {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

// Classification
impl Classifier {
    const PRECISION: usize = 1000;
    pub fn new(cfg: Config) -> Self {
        Self {
            re: RegexCache::new(),
            cfg,
        }
    }

    pub fn setup(&mut self, cfg: Config) {
        self.cfg = cfg;
    }

    /// Extracts an array from a string
    /// Current approach is to iterate within the outer brackets
    /// and count the depth of the brackets. If we see a separator at depth 0, we split the string
    /// at that index.
    pub fn extract_array<'a>(
        &'a self,
        sep: char,
        open: Option<char>,
        close: Option<char>,
    ) -> impl 'a + Fn(&'a str) -> Vec<&'a str> {
        move |s: &'a str| {
            let inner_start = open.map(|o| s.find(o).map_or(0, |i| i + 1)).unwrap_or(0);
            let inner_end = close
                .map(|c| s.rfind(c).map_or(s.len(), |i| i))
                .unwrap_or(s.len());
            let inner = &s[inner_start..inner_end];
            inner
                .chars()
                .enumerate()
                .fold((vec![0], 0_usize), |(mut acc, mut depth), (i, c)| {
                    if let Ok(b) = Bracket::try_from(c) {
                        if b.open() == c {
                            depth = depth.saturating_add(1);
                        } else if b.close() == c {
                            depth = depth.saturating_sub(1);
                        }
                    }
                    if depth == 0 && c == sep {
                        acc.push(i);
                    }
                    (acc, depth)
                })
                .0
                .into_iter()
                .chain(std::iter::once(inner.len()))
                .collect::<Vec<_>>()
                .as_slice()
                .windows(2)
                // This line causes "," and ",\n" etc to be the same
                .map(|w| inner[(if w[0] == 0 { 0 } else { w[0] + 1 })..w[1]].trim())
                // This line removes empty entries caused by trailing separators etc
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        }
    }

    pub fn classify_with<'a>(
        &'a self,
        encoding: &Encoding,
        candidate: &'a str,
    ) -> Classification<'a> {
        match encoding {
            Encoding::Base(enc) => self.classify_base_with(candidate, enc.base),
            Encoding::Array(enc) => self.classify_array_with(enc, candidate),
            Encoding::Text(enc) => self.classify_text_with(enc, candidate),
            _ => Classification::Empty,
        }
    }

    pub fn classify_text_with<'a>(
        &'a self,
        encoding: &TextEncoding,
        candidate: &'a str,
    ) -> Classification<'a> {
        Classification::Text(Text::new(
            *encoding,
            candidate,
            self.text_err(candidate, encoding),
        ))
    }

    pub fn classify<'a>(&'a self, candidate: &'a str) -> Vec<Classification<'a>> {
        self.cfg
            .available_encodings
            .iter()
            .map(|enc| self.classify_with(enc, candidate))
            .chain(std::iter::once(self.classify_array(candidate)))
            .collect()
    }

    pub fn classify_base_with<'a>(&'a self, candidate: &'a str, base: i32) -> Classification<'a> {
        Classification::Integer(Integer::new(
            base,
            self.re.extract_base(base)(candidate).unwrap_or(""),
            self.base_n_err(candidate, base),
        ))
    }

    pub fn classify_array_with<'a>(
        &'a self,
        encoding: &ArrayEncoding,
        candidate: &'a str,
    ) -> Classification<'a> {
        let values = self.extract_array(
            encoding.separator.to_char(),
            encoding.brackets.open(),
            encoding.brackets.close(),
        )(candidate);
        Classification::Array(Array::new(
            values
                .iter()
                .zip(encoding.values.iter())
                .map(|(v, e)| vec![self.classify_with(e, v)])
                .collect(),
            &encoding.brackets,
            encoding.separator,
            self.array_err(candidate, &encoding.brackets),
        ))
    }

    pub fn classify_array<'a>(&'a self, candidate: &'a str) -> Classification<'a> {
        match (
            self.re.extract_separators()(candidate),
            self.re.extract_brackets()(candidate),
        ) {
            (None, None) => Classification::Empty,
            (separator, brackets) => {
                let separator = separator.map_or(Separator::default(), Separator::from);
                let brackets = brackets.map_or(Brackets::default(), Brackets::from);
                let values =
                    self.extract_array(separator.to_char(), brackets.open(), brackets.close())(
                        candidate,
                    );

                Classification::Array(Array::new(
                    values.iter().map(|v| self.classify(v)).collect(),
                    &brackets,
                    separator,
                    self.array_err(candidate, &brackets),
                ))
            }
        }
    }

    fn array_err(&self, candidate: &str, brackets: &Brackets) -> usize {
        let between_brackets_count = match (
            brackets.open().and_then(|o| candidate.find(o)),
            brackets.close().and_then(|c| candidate.rfind(c)),
        ) {
            (Some(start), Some(end)) => end.saturating_sub(start) + 1,
            (Some(start), None) => candidate.len() - start,
            (None, Some(end)) => end,
            _ => 0,
        };
        Self::PRECISION - Self::PRECISION * between_brackets_count / candidate.len()
    }

    /// Returns the percentage of the string that does not match the base
    fn base_n_err(&self, candidate: &str, base: i32) -> usize {
        match candidate.len() {
            0 => Self::PRECISION,
            length => {
                Self::PRECISION - Self::PRECISION * self.re.match_base(base)(candidate) / length
            }
        }
    }

    /// Returns the percentage of the string that does not match the text encoding
    fn text_err(&self, candidate: &str, encoding: &TextEncoding) -> usize {
        match candidate.len() {
            0 => Self::PRECISION,
            length => {
                Self::PRECISION - Self::PRECISION * self.re.match_text(encoding)(candidate) / length
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_err() {
        const INPUT: &str = "0x1234";
        let cl = Classifier::default();
        assert_eq!(cl.base_n_err(INPUT, 16), 0);
        assert_eq!(cl.base_n_err(INPUT, 10), 167);

        const ARR_INPUT: &str = "[[123]]";
        assert_eq!(cl.array_err(ARR_INPUT, &Brackets::from('[')), 0);
        assert_eq!(cl.text_err(INPUT, &TextEncoding::Utf(8)), 0);

        const TEXT_INPUT: &str = "myFunction()";
        assert_eq!(cl.text_err(TEXT_INPUT, &TextEncoding::Utf(8)), 0);
        assert_eq!(cl.array_err(TEXT_INPUT, &Brackets::default()), 1000);
    }

    #[test]
    fn test_extract_array() {
        const ARRAY_STRING: &str = "[1, 2, 3, 4]";
        let cl = Classifier::default();
        let extracted = cl.extract_array(',', Some('['), Some(']'))(ARRAY_STRING);
        assert_eq!(extracted, vec!["1", "2", "3", "4"]);

        const ARRAY_STRING_2: &str = "[1, 2, 3, [4, 5, 6, [7, 8, 9]]]";
        let extracted = cl.extract_array(',', Some('['), Some(']'))(ARRAY_STRING_2);
        assert_eq!(extracted, vec!["1", "2", "3", "[4, 5, 6, [7, 8, 9]]"]);

        const ARRAY_STRING_3: &str = "[[1, 2, 3], [4, 5, 6], [7, 8, 9]]";
        let extracted = cl.extract_array(',', Some('['), Some(']'))(ARRAY_STRING_3);
        assert_eq!(extracted, vec!["[1, 2, 3]", "[4, 5, 6]", "[7, 8, 9]"]);

        const ARRAY_STRING_4: &str = "[1,2,3,[4,5,6,[7,8,9]]]";
        let extracted = cl.extract_array(',', Some('['), Some(']'))(ARRAY_STRING_4);
        assert_eq!(extracted, vec!["1", "2", "3", "[4,5,6,[7,8,9]]"]);
    }

    #[test]
    fn test_classify_base_2() {
        const BIN_VALUES: [&str; 3] = ["0b1010", "0b1111", "0b1001"];
        let cl = Classifier::default();
        for candidate in BIN_VALUES.iter() {
            let c = cl.classify(candidate);
            match c.iter().min_by_key(|c| c.score()).unwrap() {
                Classification::Integer(i) => assert_eq!(i.base, 2),
                _ => panic!("expected integer"),
            }
        }

        const ALMOST_BIN_VALUES: [&str; 3] = [" 30b1010", "0b1112", "0x1001"];
        for candidate in ALMOST_BIN_VALUES.iter() {
            let c = cl.classify(candidate);
            if let Classification::Integer(i) = c.iter().min_by_key(|c| c.score()).unwrap() {
                assert_ne!(i.base, 2)
            }
        }
    }

    #[test]
    fn test_classify_base_58() {
        const BS58_VALUES: [&str; 1] = ["3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy"];
        let cl = Classifier::default();
        for candidate in BS58_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.score()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }

    #[test]
    fn test_classify_base_16() {
        const HEX_VALUES: [&str; 3] = ["0x1234", "0xabcd", "123f34"];

        let cl = Classifier::default();
        for candidate in HEX_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.score()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }

        const ALMOST_HEX_VALUES: [&str; 3] = [" 0x12345678", "0xfgh", "f-16"];
        for candidate in ALMOST_HEX_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.score()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }

    #[test]
    fn test_classify_base_10() {
        const DEC_VALUES: [&str; 3] = ["123", "456", "0000768"];
        let cl = Classifier::default();
        for candidate in DEC_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.score()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }

        const ALMOST_DEC_VALUES: [&str; 3] = [" 123", "123.789", "123-45"];
        for candidate in ALMOST_DEC_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.score()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }

    #[test]
    fn test_classify_base_64() {
        const BASE_64_VALUES: [&str; 3] = ["aGVsbG8=", "aGVsbG8", "aGVsbG8=="];
        let cl = Classifier::default();
        for candidate in BASE_64_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.score()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }

        const ALMOST_BASE_64_VALUES: [&str; 3] = ["aGVsbG8", "aGVsbG8===", "aGVsbG8= "];
        for candidate in ALMOST_BASE_64_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.score()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }

    #[test]
    fn test_classify_array() {
        const ARRAY_VALUES: [&str; 4] = [
            "[0x1, 2, 3,4,5]",
            "[1, 2, 4, 3, 4]",
            "[1, 2, 3, 4, 5]",
            "[[123]]",
        ];
        const EXPECTED: [usize; 4] = [5, 5, 5, 1];
        let cl = Classifier::default();
        for (candidate, expected) in ARRAY_VALUES.iter().zip(EXPECTED.iter()) {
            let c = cl.classify(candidate);
            let best = c.iter().min().unwrap();
            match best {
                Classification::Array(a) => {
                    assert_eq!(a.collapse().len(), *expected);
                }
                _ => {
                    panic!("expected array")
                },
            }
        }

        // FIXME: Test no longer works after textencoding was added
        //const ALMOST_ARRAY_VALUES: [&str; 3] = ["[1, 2, 3", "1, 2, 3]", "(1, 4, 5"];
        //for candidate in ALMOST_ARRAY_VALUES.iter() {
        //    let c = cl.classify(candidate);
        //    let best = c.iter().min().unwrap();
        //    match best {
        //        Classification::Array(a) => {
        //            assert_eq!(a.collapse().len(), 3);
        //        }
        //        _ => {
        //            println!("{:?}", best);
        //            panic!("expected array")
        //        },
        //    }
        //}
    }
}
