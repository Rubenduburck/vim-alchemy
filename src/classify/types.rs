use std::fmt::{Display, Formatter};

use crate::encode::{
    encoding::{ArrayEncoding, Encoding},
    types::{Brackets, Separator},
};

#[derive(Debug, Default)]
pub enum Classification<'a> {
    Array(ArrayClassification<'a>),
    Integer(IntegerClassification<'a>),
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

impl<'a> From<ArrayClassification<'a>> for Classification<'a> {
    fn from(arr: ArrayClassification<'a>) -> Self {
        Classification::Array(arr)
    }
}

impl<'a> From<IntegerClassification<'a>> for Classification<'a> {
    fn from(int: IntegerClassification<'a>) -> Self {
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
            Classification::Integer(i) => Encoding::Base(i.base),
            Classification::Empty => Encoding::Empty,
        }
    }
}

#[derive(Debug)]
pub struct ArrayClassification<'a> {
    pub values: Vec<Vec<Classification<'a>>>,
    pub brackets: Brackets,
    pub separator: Separator,
    pub err: usize,
}

impl Display for ArrayClassification<'_> {
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

impl<'a> ArrayClassification<'a> {
    pub fn new(
        values: Vec<Vec<Classification<'a>>>,
        brackets: &Brackets,
        separator: Separator,
        err: usize,
    ) -> ArrayClassification<'a> {
        Self {
            values,
            brackets: brackets.clone(),
            separator,
            err,
        }
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
pub struct IntegerClassification<'a> {
    pub base: i32,
    pub value: &'a str,
    pub err: usize,
}

impl Display for IntegerClassification<'_> {
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

impl<'a> IntegerClassification<'a> {
    pub fn new(base: i32, value: &'a str, err: usize) -> Self {
        Self { base, value, err }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification_ord() {
        let left = Classification::Integer(IntegerClassification::new(10, "10", 0));
        let right = Classification::Integer(IntegerClassification::new(2, "10", 0));
        let result = left.cmp(&right);
        assert_eq!(result, std::cmp::Ordering::Less);

        let left = Classification::Integer(IntegerClassification::new(2, "10", 0));
        let right = Classification::Integer(IntegerClassification::new(16, "10", 0));
        let result = left.cmp(&right);
        assert_eq!(result, std::cmp::Ordering::Less);
    }
}
