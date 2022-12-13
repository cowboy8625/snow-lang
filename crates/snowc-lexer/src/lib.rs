mod scanner;
#[cfg(test)]
mod test;
mod token;

pub type Span = std::ops::Range<usize>;

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
