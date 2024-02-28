use crate::{
    classify::{
        regex::RegexCache,
        types::{ArrayClassification, Classification},
    },
    encode::types::{Bracket, Brackets, Separator},
};

use super::types::IntegerClassification;

pub struct Classifier {
    re: RegexCache,
}

impl Default for Classifier {
    fn default() -> Self {
        Self::new()
    }
}

// Classification
impl Classifier {
    const PRECISION: usize = 1000;
    pub fn new() -> Self {
        Self {
            re: RegexCache::new(),
        }
    }

    /// Extracts an array from a string Current approach is to iterate within the outer brackets
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

    pub fn classify<'a>(&'a self, candidate: &'a str) -> Vec<Classification<'a>> {
        vec![
            self.classify_base(candidate, 2),
            self.classify_base(candidate, 10),
            self.classify_base(candidate, 16),
            self.classify_base(candidate, 58),
            self.classify_base(candidate, 64),
            self.classify_array(candidate),
        ]
    }

    pub fn classify_base<'a>(&'a self, candidate: &'a str, base: i32) -> Classification<'a> {
        Classification::Integer(IntegerClassification::new(
            base,
            self.re.extract_base(base)(candidate).unwrap_or(""),
            self.base_n_err(candidate, base),
        ))
    }

    pub fn classify_array<'a>(&'a self, candidate: &'a str) -> Classification<'a> {
        match (
            self.re.extract_separators()(candidate),
            self.re.extract_brackets()(candidate),
        ) {
            (None, None) => Classification::Empty,
            (separator, brackets) => {
                let separator: Separator = separator.map_or(Separator::default(), |s| s.into());
                let brackets: Brackets = brackets.map_or(Brackets::default(), |b| b.into());
                let values =
                    self.extract_array(separator.to_char(), brackets.open(), brackets.close())(candidate);

                Classification::Array(ArrayClassification::new(
                    values.iter().map(|v| self.classify(v)).collect(),
                    brackets,
                    separator,
                ))
            }
        }
    }

    /// Returns the percentage of the string that does not match teh base
    fn base_n_err(&self, candidate: &str, base: i32) -> usize {
        match candidate.len() {
            0 => Self::PRECISION,
            _ => {
                Self::PRECISION
                    - Self::PRECISION * self.re.match_base(base)(candidate) / candidate.len()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_array() {
        const ARRAY_STRING: &str = "[1, 2, 3, 4]";
        let cl = Classifier::new();
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
        let cl = Classifier::new();
        for candidate in BIN_VALUES.iter() {
            let c = cl.classify(candidate);
            match c.iter().min_by_key(|c| c.error()).unwrap() {
                Classification::Integer(i) => assert_eq!(i.base, 2),
                _ => panic!("expected integer"),
            }
        }

        const ALMOST_BIN_VALUES: [&str; 3] = [" 30b1010", "0b1112", "0x1001"];
        for candidate in ALMOST_BIN_VALUES.iter() {
            let c = cl.classify(candidate);
            if let Classification::Integer(i) = c.iter().min_by_key(|c| c.error()).unwrap() {
                assert_ne!(i.base, 2)
            }
        }
    }

    #[test]
    fn test_classify_base_58() {
        const BS58_VALUES: [&str; 1] = ["3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy"];
        let cl = Classifier::new();
        for candidate in BS58_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }

    #[test]
    fn test_classify_base_16() {
        const HEX_VALUES: [&str; 3] = ["0x1234", "0xabcd", "123f34"];

        let cl = Classifier::new();
        for candidate in HEX_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }

        const ALMOST_HEX_VALUES: [&str; 3] = [" 0x12345678", "0xfgh", "f-16"];
        for candidate in ALMOST_HEX_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }

    #[test]
    fn test_classify_base_10() {
        const DEC_VALUES: [&str; 3] = ["123", "456", "0000768"];
        let cl = Classifier::new();
        for candidate in DEC_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }

        const ALMOST_DEC_VALUES: [&str; 3] = [" 123", "123.789", "123-45"];
        for candidate in ALMOST_DEC_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }

    #[test]
    fn test_classify_base_64() {
        const BASE_64_VALUES: [&str; 3] = ["aGVsbG8=", "aGVsbG8", "aGVsbG8=="];
        let cl = Classifier::new();
        for candidate in BASE_64_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }

        const ALMOST_BASE_64_VALUES: [&str; 3] = ["aGVsbG8", "aGVsbG8===", "aGVsbG8= "];
        for candidate in ALMOST_BASE_64_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }

    #[test]
    fn test_classify_array() {
        const ARRAY_VALUES: [&str; 3] = ["[0x1, 2, 3,4,5]", "[1, 2, 4, 3, 4]", "[1, 2, 3, 4, 5]"];
        let cl = Classifier::new();
        for candidate in ARRAY_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            match best {
                Classification::Array(a) => {
                    assert_eq!(a.collapse().len(), 5);
                }
                _ => panic!("expected array"),
            }
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }

        const ALMOST_ARRAY_VALUES: [&str; 3] = ["[1, 2, 3", "1, 2, 3]", "(1, 4, 5"];
        for candidate in ALMOST_ARRAY_VALUES.iter() {
            let c = cl.classify(candidate);
            let best = c.iter().min_by_key(|c| c.error()).unwrap();
            match best {
                Classification::Array(a) => {
                    assert_eq!(a.collapse().len(), 3);
                }
                _ => panic!("expected array"),
            }
            println!("candidate {:?}", candidate);
            println!("best {:?}", best);
        }
    }
}
