use super::{
    super::decoding::Decoded,
    super::error::Error,
    super::types::{Brackets,Bracket, Separator},
    super::encoding::Encoding,
};

#[derive(Debug, Clone)]
pub struct ArrayEncoding {
    pub values: Vec<Encoding>,
    pub brackets: Brackets,
    pub separator: Separator,
}

impl std::fmt::Display for ArrayEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values = self
            .values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(&self.separator.to_string());
        write!(
            f,
            "{}{}{}",
            self.brackets
                .open()
                .map(|c| c.to_string())
                .unwrap_or_default(),
            values,
            self.brackets
                .close()
                .map(|c| c.to_string())
                .unwrap_or_default(),
        )
    }
}

impl ArrayEncoding {
    pub fn new(
        values: Vec<Encoding>,
        brackets: Option<Brackets>,
        separator: Option<Separator>,
    ) -> Self {
        Self {
            values,
            brackets: brackets.unwrap_or_default(),
            separator: separator.unwrap_or_default(),
        }
    }

    pub fn flattened_values(&self) -> Vec<Encoding> {
        self.values
            .iter()
            .flat_map(|v| match v {
                Encoding::Array(a) => a.flattened_values(),
                _ => vec![v.clone()],
            })
            .collect()
    }

    pub fn flatten(&self) -> Self {
        Self::new(
            self.flattened_values(),
            Some(self.brackets.clone()),
            Some(self.separator),
        )
    }

    pub fn brackets(&self) -> [String; 2] {
        [
            self.brackets
                .open()
                .map(|c| c.to_string())
                .unwrap_or_default(),
            self.brackets
                .close()
                .map(|c| c.to_string())
                .unwrap_or_default(),
        ]
    }

    pub fn inner(&self) -> &Vec<Encoding> {
        &self.values
    }

    pub fn newline(&self) -> bool {
        self.separator.newline
    }

    pub fn encode(&self, input: &Decoded, pad: Option<bool>) -> Result<String, Error> {
        Ok(self.brackets().join(
            &input
                .to_vec()
                .iter()
                .zip(self.values.iter().cycle())
                .map(|(x, y)| y.encode(x, pad))
                .collect::<Result<Vec<String>, Error>>()?
                .join(&self.separator.to_string()),
        ))
    }
}

impl From<Vec<Encoding>> for ArrayEncoding {
    fn from(values: Vec<Encoding>) -> Self {
        Self::new(
            values,
            Some(Brackets::new(
                Some(Bracket::default()),
                Some(Bracket::default()),
            )),
            Some(Separator::default()),
        )
    }
}

impl Eq for ArrayEncoding {}

impl PartialEq for ArrayEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl Ord for ArrayEncoding {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.values.cmp(&other.values)
    }
}

impl PartialOrd for ArrayEncoding {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
