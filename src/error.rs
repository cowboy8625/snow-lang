#![allow(unused)]
use super::position::{Span, Spanned};
use std::fmt;

pub type CResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
    kind: ErrorKind,
    span: Span,
}

impl Error {
    pub fn new(msg: &str, span: Span, kind: ErrorKind) -> Box<dyn std::error::Error> {
        Box::new(Self {
            msg: msg.to_string(),
            kind,
            span,
        })
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: {}", self.span, self.kind, self.msg)
    }
}

impl std::error::Error for Error {}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ErrorKind {
    NoMain,
    EmptyReturn,
    Undefined,
    TypeError,
    LexeringFailer,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMain => write!(f, "NoMain"),
            Self::EmptyReturn => write!(f, "EmptyReturn"),
            Self::Undefined => write!(f, "Undefined"),
            Self::TypeError => write!(f, "TypeError"),
            Self::LexeringFailer => write!(f, "LexeringFailer"),
        }
    }
}
