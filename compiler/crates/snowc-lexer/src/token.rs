use super::Span;
use std::fmt;

macro_rules! is_token {
    ($i:ident, $t:ident) => {
        pub fn $i(&self) -> bool {
            match self {
                Self::$t(..) => true,
                _ => false,
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    KeyWord(String, Span),
    Id(String, Span),
    Op(String, Span),
    Int(String, Span),
    Float(String, Span),
    String(String, Span),
    Char(String, Span),
    Error(String, Span),
    Eof(Span),
}
impl Token {
    pub fn lookup<'a>(id: &'a str) -> Option<&'a str> {
        match id {
            "enum" | "data" | "type" | "true" | "false" | "return" | "let" | "and"
            | "or" | "not" | "if" | "then" | "else" | "fn" => Some(id),
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

    pub fn is_keyword_a(&self, item: impl Into<String>) -> bool {
        match self {
            Self::KeyWord(ref inner, ..) => inner == &item.into(),
            _ => false,
        }
    }

    pub fn is_op_a(&self, item: impl Into<String>) -> bool {
        match self {
            Self::Op(ref inner, ..) => inner == &item.into(),
            _ => false,
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Self::KeyWord(i, ..) => i,
            Self::Id(i, ..) => i,
            Self::Op(i, ..) => i,
            Self::Int(i, ..) => i,
            Self::Float(i, ..) => i,
            Self::String(i, ..) => i,
            Self::Char(i, ..) => i,
            Self::Error(i, ..) => i,
            Self::Eof(..) => panic!("eof does not have a value"),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::KeyWord(.., span) => *span,
            Self::Id(.., span) => *span,
            Self::Op(.., span) => *span,
            Self::Int(.., span) => *span,
            Self::Float(.., span) => *span,
            Self::String(.., span) => *span,
            Self::Char(.., span) => *span,
            Self::Error(.., span) => *span,
            Self::Eof(span) => *span,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyWord(i, ..) => write!(f, "{i}"),
            Self::Id(i, ..) => write!(f, "{i}"),
            Self::Op(i, ..) => write!(f, "{i}"),
            Self::Int(i, ..) => write!(f, "{i}"),
            Self::Float(i, ..) => write!(f, "{i}"),
            Self::String(i, ..) => write!(f, "{i}"),
            Self::Char(i, ..) => write!(f, "{i}"),
            Self::Error(i, ..) => write!(f, "{i}"),
            Self::Eof(..) => write!(f, "EOF"),
        }
    }
}
