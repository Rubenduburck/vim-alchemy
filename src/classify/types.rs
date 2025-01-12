use std::fmt::{Display, Formatter};

use crate::encode::{
    encoding::{ArrayEncoding, BaseEncoding, Encoding},
    types::{Brackets, Separator},
};

#[derive(Debug, Default)]
pub enum Classification<'a> {
    Array(Array<'a>),
    Integer(Integer<'a>),
    #[default]
    Empty,
}

impl Display for Classification<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Classification::Array(arr) => arr.fmt(f),
            Classification::Integer(int) => int.fmt(f),
            Classification::Empty => write!(f, "Empty"),
        }
    }
}

impl Default for &Classification<'_> {
    fn default() -> Self {
        &Classification::Empty
    }
}

impl Eq for Classification<'_> {}

impl PartialEq for Classification<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.error() == other.error() && self.encoding() == other.encoding()
    }
}

impl PartialOrd for Classification<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Classification<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.error().cmp(&other.error()) {
            std::cmp::Ordering::Equal => self.encoding().cmp(&other.encoding()),
            ord => ord,
        }
    }
}

impl Classification<'_> {
    pub fn error(&self) -> usize {
        match self {
            Classification::Array(arr) => arr.err,
            Classification::Integer(int) => int.err,
            Classification::Empty => usize::MAX,
        }
    }
}

impl<'a> From<Array<'a>> for Classification<'a> {
    fn from(arr: Array<'a>) -> Self {
        Classification::Array(arr)
    }
}

impl<'a> From<Integer<'a>> for Classification<'a> {
    fn from(int: Integer<'a>) -> Self {
        Classification::Integer(int)
    }
}

impl Classification<'_> {
    pub fn encoding(&self) -> Encoding {
        match self {
            Classification::Array(v) => Encoding::Array(ArrayEncoding::new(
                v.collapse().iter().map(|c| c.encoding()).collect(),
                Some(v.brackets.clone()),
                Some(v.separator),
            )),
            Classification::Integer(i) => Encoding::Base(BaseEncoding::new(i.base)),
            Classification::Empty => Encoding::Empty,
        }
    }

    pub fn is_lines(&self) -> bool {
        match self {
            Classification::Array(arr) => arr.is_lines(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Array<'a> {
    pub values: Vec<Vec<Classification<'a>>>,
    pub brackets: Brackets,
    pub separator: Separator,
    pub err: usize,
}

impl Display for Array<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.brackets.string_pair().join(
                &self
                    .values
                    .iter()
                    .map(|v| self.brackets.string_pair().join(
                        &v.iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join(&self.separator.to_string())
                    ))
                    .collect::<Vec<_>>()
                    .join(&self.separator.to_string())
            )
        )
    }
}

impl<'a> Array<'a> {
    pub fn new(
        values: Vec<Vec<Classification<'a>>>,
        brackets: &Brackets,
        separator: Separator,
        err: usize,
    ) -> Array<'a> {
        Self {
            values,
            brackets: brackets.clone(),
            separator,
            err,
        }
    }

    pub fn is_lines(&self) -> bool {
        self.separator.is_newline() && self.brackets.is_none()
    }

    pub fn collapse(&self) -> Vec<&Classification> {
        self.values
            .iter()
            .map(|classifications| {
                classifications
                    .iter()
                    .min()
                    .unwrap_or(&Classification::Empty)
            })
            .collect()
    }

    pub fn values(&self) -> &Vec<Vec<Classification>> {
        &self.values
    }
}

#[derive(Debug)]
pub struct Integer<'a> {
    pub base: i32,
    pub value: &'a str,
    pub err: usize,
}

impl Display for Integer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.base {
            2 => write!(f, "bin {}", self.value),
            8 => write!(f, "oct {}", self.value),
            10 => write!(f, "dec {}", self.value),
            16 => write!(f, "hex {}", self.value),
            _ => write!(f, "base{} {}", self.base, self.value),
        }
    }
}

impl<'a> Integer<'a> {
    pub fn new(base: i32, value: &'a str, err: usize) -> Self {
        Self { base, value, err }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification_ord() {
        let left = Classification::Integer(Integer::new(10, "10", 0));
        let right = Classification::Integer(Integer::new(2, "10", 0));
        let result = left.cmp(&right);
        assert_eq!(result, std::cmp::Ordering::Less);

        let left = Classification::Integer(Integer::new(2, "10", 0));
        let right = Classification::Integer(Integer::new(16, "10", 0));
        let result = left.cmp(&right);
        assert_eq!(result, std::cmp::Ordering::Less);
    }
}
