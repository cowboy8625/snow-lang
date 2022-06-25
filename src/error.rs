#![allow(unused)]
use super::position::{Span, Spanned};
use std::fmt;

pub type CResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
pub struct MulitError {
    pub errors: Vec<Box<dyn std::error::Error>>,
}

impl std::error::Error for MulitError {}

impl MulitError {
    pub fn push(&mut self, error: Box<dyn std::error::Error>) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}
impl fmt::Display for MulitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in self.errors.iter() {
            write!(f, "{}\n", e)?;
        }
        Ok(())
    }
}

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

    pub fn span(&self) -> Span {
        self.span.clone()
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

    UnknownChar,
    UnclosedDelimiter,
    InvalidIndentation,
    ReserverdWord,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMain => write!(f, "NoMain"),
            Self::EmptyReturn => write!(f, "EmptyReturn"),
            Self::Undefined => write!(f, "Undefined"),
            Self::TypeError => write!(f, "TypeError"),
            Self::LexeringFailer => write!(f, "LexeringFailer"),
            Self::UnknownChar => write!(f, "UnknownChar"),
            Self::UnclosedDelimiter => write!(f, "UnclosedDelimiter"),
            Self::InvalidIndentation => write!(f, "InvalidIndentation"),
            Self::ReserverdWord => write!(f, "ReserverdWord"),
        }
    }
}
