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

#[derive(Clone, PartialEq, Eq)]
pub enum Token {
    KeyWord(String, TokenPosition, Span),
    Id(String, TokenPosition, Span),
    Op(String, TokenPosition, Span),
    Int(String, TokenPosition, Span),
    Float(String, TokenPosition, Span),
    String(String, TokenPosition, Span),
    Char(String, TokenPosition, Span),
    Error(String, Span),
    Eof(Span),
}

impl Token {
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

    pub fn is_id_a(&self, item: impl Into<String>) -> bool {
        match self {
            Self::Id(ref inner, ..) => inner == &item.into(),
            _ => false,
        }
    }

    pub fn position(&self) -> &TokenPosition {
        match self {
            Self::KeyWord(_, tp, ..) => tp,
            Self::Id(_, tp, ..) => tp,
            Self::Op(_, tp, ..) => tp,
            Self::Int(_, tp, ..) => tp,
            Self::Float(_, tp, ..) => tp,
            Self::String(_, tp, ..) => tp,
            Self::Char(_, tp, ..) => tp,
            // FIXME: I don't like this
            Self::Error(_, ..) => &TokenPosition::End,
            Self::Eof(..) => &TokenPosition::End,
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
            Self::Eof(..) => "Eof",
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyWord(i, tp, ..) => write!(f, "{i} {tp:?}"),
            Self::Id(i, tp, ..) => write!(f, "{i} {tp:?}"),
            Self::Op(i, tp, ..) => write!(f, "{i} {tp:?}"),
            Self::Int(i, tp, ..) => write!(f, "{i} {tp:?}"),
            Self::Float(i, tp, ..) => write!(f, "{i} {tp:?}"),
            Self::String(i, tp, ..) => write!(f, "{i:?} {tp:?}"),
            Self::Char(i, tp, ..) => write!(f, "{i} {tp:?}"),
            Self::Error(i, ..) => write!(f, "{i}"),
            Self::Eof(..) => write!(f, "EOF"),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenPosition {
    Start,
    Middle,
    End,
    FullSpan,
}
