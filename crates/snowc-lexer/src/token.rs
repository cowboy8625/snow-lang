use std::fmt;

macro_rules! is_token {
    ($i:ident, Eof) => {
        pub fn $i(&self) -> bool {
            match self {
                Self::Eof => true,
                _ => false,
            }
        }
    };
    ($i:ident, $t:ident) => {
        pub fn $i(&self) -> bool {
            match self {
                Self::$t(..) => true,
                _ => false,
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KeyWord(String),
    Id(String),
    Op(String),
    Int(String),
    Float(String),
    String(String),
    Char(String),
    Error(String),
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
    is_token!(is_keyword, KeyWord);
    is_token!(is_id, Id);
    is_token!(is_op, Op);
    is_token!(is_int, Int);
    is_token!(is_float, Float);
    is_token!(is_string, String);
    is_token!(is_char, Char);
    is_token!(is_error, Error);
    is_token!(is_eof, Eof);
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
