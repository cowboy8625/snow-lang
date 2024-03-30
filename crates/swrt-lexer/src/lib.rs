// TODO:[2](cowboy) move scanner over to its own
//              stream type to support peeking
//              at nth none destructively.
mod scanner;
mod span;
mod token;

pub use span::Span;

pub use crate::scanner::Scanner;
pub use crate::token::Token;

#[derive(Debug, Clone, Copy)]
pub enum LexerDebug {
    On,
    Off,
}

impl Default for LexerDebug {
    fn default() -> Self {
        Self::Off
    }
}

impl From<bool> for LexerDebug {
    fn from(toggle: bool) -> Self {
        if toggle {
            return Self::On;
        }
        Self::Off
    }
}
