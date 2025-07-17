use std::collections::HashMap;

use crate::encode::encoding::TextEncoding;

// Include the compile-time generated regex data
include!(concat!(env!("OUT_DIR"), "/compiled_regexes.rs"));

// Public functions for regex operations
pub fn extract_common<'a>(get_regex: fn() -> &'static Regex) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
    move |s: &'a str| {
        extract_all(get_regex)(s)
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

pub fn extract_first<'a>(get_regex: fn() -> &'static Regex) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
    move |s: &'a str| {
        let regex = get_regex();
        regex.find(s).map(|m| m.as_str())
    }
}

pub fn extract_longest<'a>(get_regex: fn() -> &'static Regex) -> impl 'a + Fn(&'a str) -> Option<&'a str> {
    move |s: &'a str| {
        get_regex()
            .find_iter(s)
            .max_by_key(|m| m.len())
            .map(|m| m.as_str())
    }
}

pub fn extract_all<'a>(get_regex: fn() -> &'static Regex) -> impl 'a + Fn(&'a str) -> Vec<&'a str> {
    move |s: &'a str| {
        get_regex()
            .find_iter(s)
            .map(|m| m.as_str())
            .collect()
    }
}

pub fn match_count<'a>(get_regex: fn() -> &'static Regex) -> impl 'a + Fn(&'a str) -> usize {
    move |s: &'a str| get_regex().find_iter(s).count()
}

pub fn match_length<'a>(get_regex: fn() -> &'static Regex) -> impl 'a + Fn(&'a str) -> usize {
    move |s: &'a str| {
        get_regex()
            .find_iter(s)
            .map(|m| m.len())
            .sum()
    }
}

pub fn match_base(base: i32) -> impl Fn(&str) -> usize {
    move |s: &str| {
        let get_regex = match base {
            2 => get_match_base2,
            10 => get_match_base10,
            16 => get_match_base16,
            58 => get_match_base58,
            64 => get_match_base64,
            _ => get_match_base10,
        };
        match_length(get_regex)(s)
    }
}

pub fn extract_base(base: i32) -> impl Fn(&str) -> Option<&str> {
    move |s: &str| {
        let get_regex = match base {
            2 => get_extract_base2,
            10 => get_extract_base10,
            16 => get_extract_base16,
            58 => get_extract_base58,
            64 => get_extract_base64,
            _ => get_extract_base10,
        };
        extract_longest(get_regex)(s)
    }
}

pub fn match_text<'a>(encoding: &TextEncoding) -> impl 'a + Fn(&'a str) -> usize {
    match encoding {
        TextEncoding::Utf(8) | TextEncoding::Ascii => {
            |s: &'a str| s.chars().filter(|c| c.is_ascii()).count()
        }
        TextEncoding::Utf(16) => |s: &'a str| s.len(),
        _ => |_: &'a str| 0,
    }
}

pub fn match_array() -> impl Fn(&str) -> usize {
    |s: &str| match_count(get_match_array)(s)
}

pub fn extract_brackets() -> impl Fn(&str) -> Option<&str> {
    |s: &str| extract_common(get_extract_brackets)(s)
}

pub fn extract_first_brackets() -> impl Fn(&str) -> Option<&str> {
    |s: &str| extract_first(get_extract_brackets)(s)
}

pub fn extract_separators() -> impl Fn(&str) -> Option<&str> {
    |s: &str| extract_common(get_extract_separators)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_match() {
        assert_eq!(match_base(2)("0b1010"), 6);
        assert_eq!(match_base(10)("1010"), 4);
        assert_eq!(match_base(16)("0x1010"), 6);
        assert_eq!(match_base(58)("1A"), 2);
        assert_eq!(match_base(64)("aGVsbG8="), 8);
    }

    #[test]
    fn test_regex_extract() {
        assert_eq!(extract_base(2)("0b1010").unwrap(), "1010");
        assert_eq!(extract_base(10)("1010").unwrap(), "1010");
        assert_eq!(extract_base(16)("0x1010").unwrap(), "1010");
        assert_eq!(extract_base(58)("1A").unwrap(), "1A");
        assert_eq!(extract_base(64)("aGVsbG8=").unwrap(), "aGVsbG8");
    }
}