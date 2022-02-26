use super::position::Span;
use std::error::Error;
use std::fmt;
#[derive(Debug, Clone)]
pub struct ParserError {
    msg: String,
    span: Span,
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ParserError: {}", self.span, self.msg)
    }
}

impl Error for ParserError {}

#[derive(Debug, Clone)]
pub struct EvalError {
    msg: String,
    span: Span,
}

impl EvalError {
    pub fn new(msg: &str, span: Span) -> Box<dyn Error> {
        Box::new(Self {
            msg: msg.to_string(),
            span,
        })
    }
}
impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} EvalError: {}", self.span, self.msg)
    }
}

impl Error for EvalError {}
