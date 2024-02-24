use core::fmt;
use std::fmt::{Display, Formatter};


#[derive(Debug, Default, Hash, Clone)]
pub struct Brackets {
    open: Bracket,
    close: Bracket,
}

impl Brackets {
    pub fn new(open: Bracket, close: Bracket) -> Self {
        Self { open, close }
    }

    pub fn open(&self) -> char {
        self.open.open()
    }

    pub fn close(&self) -> char {
        self.close.close()
    }

    pub fn pair(&self) -> [char; 2] {
        [self.open(), self.close()]
    }

    pub fn string_pair(&self) -> Vec<String> {
        vec![self.open().to_string(), self.close().to_string()]
    }
}

impl From<&[Bracket]> for Brackets {
    fn from(brackets: &[Bracket]) -> Self {
        let (first_bracket, last_bracket) = brackets.iter()
            .next()
            .map_or((Bracket::default(), Bracket::default()), |first| {
                (*first, *brackets.last().unwrap_or(first))
            });

        Self::new(first_bracket, last_bracket)
    }
}

impl From<Bracket> for Brackets {
    fn from(bracket: Bracket) -> Self {
        Self::new(bracket, bracket)
    }
}

impl From<char> for Brackets {
    fn from(c: char) -> Self {
        Bracket::try_from(c)
            .map_or(Brackets::default(), |bracket| bracket.into())
    }
}

impl From<&[char]> for Brackets {
    fn from(chars: &[char]) -> Self {
        chars.iter().map(|c| Bracket::try_from(*c).unwrap_or_default()).collect::<Vec<_>>().as_slice().into()
    }
}

impl From<&str> for Brackets {
    fn from(s: &str) -> Self {
        s.chars().collect::<Vec<_>>().as_slice().into()
    }
}

#[derive(Debug, Hash, Default, Clone, Copy)]
pub enum Bracket {
    #[default]
    Square,
    Round,
    Curly,
    Angle,
}

impl TryFrom<char> for Bracket {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' | ')' => Ok(Bracket::Round),
            '[' | ']' => Ok(Bracket::Square),
            '{' | '}' => Ok(Bracket::Curly),
            '<' | '>' => Ok(Bracket::Angle),
            _ => Err("Invalid bracket"),
        }
    }
}

impl Bracket {
    pub fn open(&self) -> char {
        match self {
            Bracket::Round => '(',
            Bracket::Square => '[',
            Bracket::Curly => '{',
            Bracket::Angle => '<',
        }
    }

    pub fn close(&self) -> char {
        match self {
            Bracket::Round => ')',
            Bracket::Square => ']',
            Bracket::Curly => '}',
            Bracket::Angle => '>',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Separator {
    pub char: char,
}

impl Display for Separator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char)
    }
}

impl Separator {
    pub fn new(separator: char) -> Self {
        Self { char: separator }
    }

    pub fn to_char(&self) -> char {
        self.char
    }

}

impl Default for Separator {
    fn default() -> Self {
        Self { char: ',' }
    }
}

impl From<char> for Separator {
    fn from(separator: char) -> Self {
        Self { char: separator }
    }
}

impl From<&str> for Separator {
    fn from(separator: &str) -> Self {
        Separator::new(separator.chars().next().unwrap_or_default())
    }
}

