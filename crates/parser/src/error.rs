use super::Span;
use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct ParserError {
    message: String,
    span: Span,
}

impl ParserError {
    pub fn new(message: String, span: Span) -> Self {
        let message = format!("{}:{} {}", span.start, span.end, message);
        Self { message, span }
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        &self.message
    }
}
