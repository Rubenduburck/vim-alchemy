use std::collections::HashMap;

use regex::{Regex, RegexBuilder};

use crate::encode::encoding::TextEncoding;

pub struct RegexCache {
    match_cache: RegexMatch,
    extract_cache: RegexExtract,
}

pub struct RegexMatch {
    base16: Regex,
    base10: Regex,
    base64: Regex,
    array: Regex,
    base2: Regex,
    base58: Regex,
}

pub struct RegexExtract {
    base16: Regex,
    base10: Regex,
    base64: Regex,
    brackets: Regex,
    base2: Regex,
    base58: Regex,
    separators: Regex,
    array: Regex,
}

fn init_regex(case_insensitive: bool, pattern: &str) -> Regex {
    RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .build()
        .expect("could not build regex")
}

impl RegexCache {
    pub fn new() -> Self {
        Self {
            match_cache: RegexMatch::new(),
            extract_cache: RegexExtract::new(),
        }
    }

    pub fn extract_common<'a>(re: &'a Regex) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
        move |s: &'a str| {
            Self::extract_all(re)(s)
                .into_iter()
                .fold(HashMap::new(), |mut acc, s| {
                    *acc.entry(s).or_insert(0) += 1;
                    acc
                })
                .into_iter()
                .max_by_key(|(_, count)| *count)
                .map(|(s, _)| s)
        }
    }

    pub fn extract_first<'a>(re: &'a Regex) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
        move |s: &'a str| re.find(s).map(|s| s.as_str())
    }

    pub fn extract_longest<'a>(re: &'a Regex) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
        move |s: &'a str| re.find_iter(s).max_by_key(|s| s.len()).map(|s| s.as_str())
    }

    pub fn extract_all<'a>(re: &'a Regex) -> impl 'a + Fn(&'a str) -> Vec<&'a str> {
        move |s: &'a str| re.find_iter(s).map(|s| s.as_str()).collect()
    }

    pub fn match_count<'a>(re: &'a Regex) -> impl 'a + Fn(&'a str) -> usize {
        move |s: &'a str| re.find_iter(s).count()
    }

    pub fn match_length<'a>(re: &'a Regex) -> impl 'a + Fn(&'a str) -> usize {
        move |s: &'a str| re.find_iter(s).map(|s| s.as_str().len()).sum()
    }

    pub fn match_base<'a>(&'a self, base: i32) -> impl 'a + Fn(&'a str) -> usize {
        Self::match_length(self.match_cache.base(base))
    }

    pub fn extract_base<'a>(&'a self, base: i32) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
        Self::extract_longest(self.extract_cache.base(base))
    }

    pub fn match_text<'a>(&'a self, encoding: &TextEncoding) -> impl 'a + Fn(&'a str) -> usize {
        match encoding {
            TextEncoding::Utf(8) | TextEncoding::Ascii => {
                |s: &'a str| s.chars().filter(|c| c.is_ascii()).count()
            }
            TextEncoding::Utf(16) => |s: &'a str| s.len(),
            _ => |_: &'a str| 0,
        }
    }

    pub fn match_array<'a>(&'a self) -> impl 'a + Fn(&'a str) -> usize {
        Self::match_count(&self.match_cache.array)
    }

    pub fn extract_brackets<'a>(&'a self) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
        Self::extract_common(&self.extract_cache.brackets)
    }

    pub fn extract_first_brackets<'a>(&'a self) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
        Self::extract_first(&self.extract_cache.brackets)
    }

    pub fn extract_separators<'a>(&'a self) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
        Self::extract_common(&self.extract_cache.separators)
    }
}

impl Default for RegexCache {
    fn default() -> Self {
        Self::new()
    }
}

impl RegexMatch {
    const BASE_64: &'static str = r"[A-Za-z0-9+/]+={0,2}";
    const BASE_58: &'static str =
        r"^[123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz]+$";
    const BASE_16: &'static str = r"(0x)?[0-9a-f]+";
    const BASE_10: &'static str = r"[0-9]+";
    const BASE_2: &'static str = r"(0b)?[01]+";
    const ARRAY: &'static str = r"(.+)([\s*,]\s*.+)";

    pub fn new() -> Self {
        Self {
            base2: init_regex(true, Self::BASE_2),
            base16: init_regex(true, Self::BASE_16),
            base10: init_regex(false, Self::BASE_10),
            base58: init_regex(false, Self::BASE_58),
            base64: init_regex(false, Self::BASE_64),

            array: init_regex(false, Self::ARRAY),
        }
    }

    pub fn base(&self, base: i32) -> &Regex {
        match base {
            2 => &self.base2,
            10 => &self.base10,
            16 => &self.base16,
            58 => &self.base58,
            64 => &self.base64,
            _ => &self.base10,
        }
    }

    pub fn array(&self) -> &Regex {
        &self.array
    }
}

impl Default for RegexMatch {
    fn default() -> Self {
        Self::new()
    }
}

impl RegexExtract {
    const BASE_64: &'static str = r"[A-Za-z0-9+/]+";
    const BASE_58: &'static str =
        r"^[123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz]+$";
    const BASE_16: &'static str = r"[0-9a-f]+";
    const BASE_10: &'static str = r"[0-9]+";
    const BASE_2: &'static str = r"[01]+";
    const BRACKETS: &'static str = r"[<\[\(\{\}\)\]>]";
    const SEPARATORS: &'static str = r"[\s,]\s*";
    const ARRAY: &'static str = r"(.+?)(?:[\s,]+|$)";

    pub fn new() -> Self {
        Self {
            base2: init_regex(true, Self::BASE_2),
            base16: init_regex(true, Self::BASE_16),
            base10: init_regex(false, Self::BASE_10),
            base58: init_regex(false, Self::BASE_58),
            base64: init_regex(false, Self::BASE_64),
            brackets: init_regex(false, Self::BRACKETS),
            separators: init_regex(false, Self::SEPARATORS),
            array: init_regex(false, Self::ARRAY),
        }
    }

    pub fn base(&self, base: i32) -> &Regex {
        match base {
            2 => &self.base2,
            10 => &self.base10,
            16 => &self.base16,
            58 => &self.base58,
            64 => &self.base64,
            _ => &self.base10,
        }
    }

    pub fn brackets(&self) -> &Regex {
        &self.brackets
    }

    pub fn separators(&self) -> &Regex {
        &self.separators
    }

    pub fn array(&self) -> &Regex {
        &self.array
    }
}

impl Default for RegexExtract {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_match() {
        let cache = RegexCache::new();
        assert_eq!(cache.match_base(2)("0b1010"), 6);
        assert_eq!(cache.match_base(10)("1010"), 4);
        assert_eq!(cache.match_base(16)("0x1010"), 6);
        assert_eq!(cache.match_base(58)("1A"), 2);
        assert_eq!(cache.match_base(64)("aGVsbG8="), 8);
    }

    #[test]
    fn test_regex_extract() {
        let cache = RegexCache::new();
        assert_eq!(cache.extract_base(2)("0b1010").unwrap(), "1010");
        assert_eq!(cache.extract_base(10)("1010").unwrap(), "1010");
        assert_eq!(cache.extract_base(16)("0x1010").unwrap(), "1010");
        assert_eq!(cache.extract_base(58)("1A").unwrap(), "1A");
        assert_eq!(cache.extract_base(64)("aGVsbG8=").unwrap(), "aGVsbG8");
    }
}
