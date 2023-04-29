pub mod expr;
pub mod op;
pub mod parser;
pub use expr::{Atom, Expr};
pub use op::Op;
mod precedence;
#[cfg(test)]
mod tests;
use parser::Parser;
use snowc_error::ErrorCode;
use snowc_error_messages::Error;
pub use snowc_lexer::{Scanner, Span, Token};

pub fn parse(scanner: Scanner) -> Result<Vec<Expr>, Error> {
    let stream = scanner.peekable();
    Parser::new(stream).parse()
}

#[macro_export]
macro_rules! bail {
    ($span:expr $(, $arg:expr)* $(,)?) => {{
        let msg = format!($($arg, )*);
        return Err(Box::new($crate::error::ParserError::new(msg, $span)));
    }};
}
