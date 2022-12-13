pub mod error;
pub mod expr;
pub mod op;
pub mod parser;
pub use error::ParserError;
pub use expr::{Atom, Expr};
pub use op::Op;
mod precedence;
#[cfg(test)]
mod tests;
use snowc_errors::CResult;
pub use snowc_lexer::{LexerDebug, Scanner, Token};

type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, Copy)]
pub enum OutOfMain {
    Disable,
    Enable,
}

impl Default for OutOfMain {
    fn default() -> Self {
        Self::Disable
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParserDebug {
    On,
    Off,
}

impl Default for ParserDebug {
    fn default() -> Self {
        Self::Off
    }
}

impl From<bool> for ParserDebug {
    fn from(toggle: bool) -> Self {
        if toggle {
            return Self::On;
        }
        Self::Off
    }
}

#[derive(Debug, Default)]
pub struct ParserBuilder {
    debug_lexer: LexerDebug,
    debug_parser: ParserDebug,
    out_of_main: OutOfMain,
}

impl ParserBuilder {
    pub fn debug_lexer(mut self, tog: bool) -> Self {
        if tog {
            self.debug_lexer = LexerDebug::On;
        }
        self
    }

    pub fn debug_parser(mut self, tog: bool) -> Self {
        if tog {
            self.debug_parser = ParserDebug::On;
        }
        self
    }

    pub fn out_of_main(mut self, tog: bool) -> Self {
        if tog {
            self.out_of_main = OutOfMain::Enable;
        }
        self
    }

    pub fn build(self, src: &str) -> parser::Parser {
        let Self {
            debug_lexer,
            debug_parser,
            out_of_main,
        } = self;
        let lexer = Scanner::new(src, debug_lexer).peekable();
        parser::Parser::new_with_debug(lexer, out_of_main, debug_parser)
    }
}

#[macro_export]
macro_rules! bail {
    ($span:expr $(, $arg:expr)* $(,)?) => {{
        let msg = format!($($arg, )*);
        return Err(Box::new(crate::error::ParserError::new(msg, $span)));
    }};
}
