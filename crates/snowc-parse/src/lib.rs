pub mod expr;
pub mod op;
pub mod parser;
pub use expr::{Atom, Expr};
pub use op::Op;
mod precedence;
#[cfg(test)]
mod tests;
use snowc_error::ErrorCode;
use snowc_error_messages::Error;
pub use snowc_lexer::{LexerDebug, Scanner, Span, Token};

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

#[derive(Debug, Default)]
pub struct ParserBuilder {
    debug_lexer: LexerDebug,
    debug_parser: ParserDebug,
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

    pub fn build(self, src: &str) -> parser::Parser {
        let Self {
            debug_lexer,
            debug_parser,
        } = self;
        let lexer = Scanner::new(src, debug_lexer).peekable();
        parser::Parser::new_with_debug(lexer, debug_parser)
    }
}

#[macro_export]
macro_rules! bail {
    ($span:expr $(, $arg:expr)* $(,)?) => {{
        let msg = format!($($arg, )*);
        return Err(Box::new($crate::error::ParserError::new(msg, $span)));
    }};
}
