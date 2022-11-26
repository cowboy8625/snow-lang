use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KeyWord(String),
    Id(String),
    Op(String),
    Int(String),
    Float(String),
    String(String),
    Char(char),
    Error(char),
    Eof,
}
impl Token {
    pub fn lookup(id: &str) -> Option<Self> {
        match id {
            "type" | "true" | "false" | "return" | "let" | "and" | "or" | "not"
            | "if" | "then" | "else" | "fn" => Some(Self::KeyWord(id.into())),
            _ => None,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyWord(i) => write!(f, "{i}"),
            Self::Id(i) => write!(f, "{i}"),
            Self::Op(i) => write!(f, "{i}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(i) => write!(f, "{i}"),
            Self::String(i) => write!(f, "{i}"),
            Self::Char(i) => write!(f, "{i}"),
            Self::Error(i) => write!(f, "{i}"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
