use snowc_parse::{Expr, Span};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(i32, Span),
    Float(String, Span),
    Bool(bool, Span),
    String(String, Span),
    Char(char, Span),
    Array(Vec<Self>, Span),
    Func(Expr, Span),
}

impl Value {
    pub fn span(&self) -> Span {
        match self {
            Self::Int(_, span) => *span,
            Self::Float(_, span) => *span,
            Self::Bool(_, span) => *span,
            Self::String(_, span) => *span,
            Self::Char(_, span) => *span,
            Self::Array(_, span) => *span,
            Self::Func(_, span) => *span,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i, ..) => write!(f, "{i}"),
            Self::Float(i, ..) => write!(f, "{i}"),
            Self::Bool(b, ..) => write!(f, "{b}"),
            Self::String(s, ..) => write!(f, "{s}"),
            Self::Char(s, ..) => write!(f, "{s}"),
            Self::Array(array, ..) => {
                let mut a = array.iter().enumerate().fold(
                    "[".to_string(),
                    |mut acc, (idx, item)| {
                        if idx != 0 {
                            acc += ", ";
                        }
                        acc += item.to_string().as_str();
                        acc
                    },
                );
                a += "]";
                write!(f, "{a}")
            }
            Self::Func(expr, ..) => write!(f, "{expr}"),
        }
    }
}
