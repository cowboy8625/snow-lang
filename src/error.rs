#![allow(unused)]
use super::position::{Span, Spanned};
use std::fmt;

pub type CResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
pub struct MulitError {
    errors: Vec<Box<dyn std::error::Error>>,
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
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMain => write!(f, "no main"),
            Self::EmptyReturn => write!(f, "empty return"),
            Self::Undefined => write!(f, "undefined"),
            Self::TypeError => write!(f, "type error"),
            Self::LexeringFailer => write!(f, "lexering failer"),
            Self::UnknownChar => write!(f, "unknown char"),
            Self::UnclosedDelimiter => write!(f, "unclosed delimiter"),
        }
    }
}
