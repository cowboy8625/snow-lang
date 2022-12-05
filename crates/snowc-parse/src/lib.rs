pub mod error;
pub mod expr;
pub mod op;
pub mod parser;
pub use error::ParserError;
pub use expr::{Atom, Expr};
pub use op::Op;
pub use parser::parse;
mod precedence;
#[cfg(test)]
mod tests;
use snowc_errors::CResult;
use snowc_lexer::{Scanner, Token};

type Span = std::ops::Range<usize>;

#[macro_export]
macro_rules! bail {
    ($span:expr $(, $arg:expr)* $(,)?) => {{
        let msg = format!($($arg, )*);
        return Err(Box::new(crate::error::ParserError::new(msg, $span)));
    }};
}
